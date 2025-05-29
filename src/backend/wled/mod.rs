mod backend_event;
pub mod error;
pub mod websocket;

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::StreamExt;
use maplit::btreeset;
use thiserror::Error;
use tokio::select;
use tokio::sync::Mutex;
use tokio::sync::broadcast::Receiver;
use tokio_tungstenite::{connect_async, tungstenite};

use bifrost_api::backend::BackendRequest;
use bifrost_api::config::WledServer;
use hue::api::{
    BridgeHome, ColorTemperature, DeviceArchetype, DeviceProductData, Dimming, GroupedLight, Light,
    LightColor, LightEffects, LightEffectsV2, LightMetadata, MirekSchema, RType, Resource,
    ResourceLink, Room, RoomArchetype, RoomMetadata, Stub,
};
use hue::xy::XY;
use svc::error::SvcError;
use svc::template::ServiceTemplate;
use svc::traits::{BoxDynService, Service};
use wled::{Color, SegCap, StateSeg, WledFrame, WledInfo};

use crate::backend::wled::websocket::WledWebSocket;
use crate::error::{ApiError, ApiResult};
use crate::resource::Resources;
use crate::server::appstate::AppState;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("No config found for wled server {0:?}")]
    NotFound(String),
}

pub struct WledServiceTemplate {
    state: AppState,
}

impl WledServiceTemplate {
    #[must_use]
    pub const fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ServiceTemplate for WledServiceTemplate {
    fn generate(&self, name: String) -> Result<BoxDynService, SvcError> {
        let config = self.state.config();
        let Some(server) = config.wled.servers.get(&name) else {
            return Err(SvcError::generation(TemplateError::NotFound(name)));
        };
        let svc = WledBackend::new(name, server.clone(), self.state.res.clone())
            .map_err(SvcError::generation)?;

        Ok(svc.boxed())
    }
}

pub struct WledBackend {
    name: String,
    server: WledServer,
    state: Arc<Mutex<Resources>>,
    info: Option<WledInfo>,
    room: Option<ResourceLink>,
    glight: Option<ResourceLink>,
    map: HashMap<u8, ResourceLink>,
    rmap: HashMap<ResourceLink, u8>,
    socket: Option<WledWebSocket>,
}

impl WledBackend {
    pub fn new(name: String, server: WledServer, state: Arc<Mutex<Resources>>) -> ApiResult<Self> {
        let map = HashMap::new();
        let rmap = HashMap::new();
        Ok(Self {
            name,
            server,
            state,
            map,
            rmap,
            room: None,
            glight: None,
            info: None,
            socket: None,
        })
    }

    fn device_link(&self, index: u8) -> ResourceLink {
        RType::Device.deterministic((&self.name, u64::from(index)))
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub async fn add_light(
        &mut self,
        index: u8,
        state: &StateSeg,
        features: SegCap,
        info: &WledInfo,
    ) -> ApiResult<()> {
        let name = &info.name;

        let link_device = self.device_link(index);
        let link_light = RType::Light.deterministic(link_device);

        let product_data = DeviceProductData {
            model_id: self.server.url.to_string(),
            manufacturer_name: info.brand.clone(),
            product_name: info.product.clone(),
            product_archetype: DeviceArchetype::HueLightstrip,
            certified: false,
            software_version: info.ver.clone(),
            hardware_platform_type: None,
        };
        let metadata = LightMetadata::new(product_data.product_archetype.clone(), name);

        let dev = hue::api::Device {
            product_data,
            metadata: metadata.clone().into(),
            services: btreeset![link_light],
            identify: Some(Stub),
            usertest: None,
        };

        let mut light = Light::new(link_device, metadata);

        /* let features = info.leds.as_ref().and_then(|leds| leds.lc).unwrap_or(0); */

        light.dimming = Some(Dimming {
            brightness: f64::from(state.bri) / 2.55,
            min_dim_level: Some(1.0 / 255.0),
        });
        log::warn!("Detected dimming: {:?}", &light.dimming);

        light.color_temperature = None;

        if features.contains(SegCap::CCT) {
            light.color_temperature = Some(ColorTemperature {
                mirek: Some(((f64::from(state.cct) / 255.0).mul_add(424.0, 100.0)) as u16),
                mirek_valid: false,
                mirek_schema: MirekSchema {
                    mirek_minimum: 100,
                    mirek_maximum: 525,
                },
            });
            log::warn!("Detected color temperature: {:?}", &light.color_temperature);
        }

        if features.contains(SegCap::RGB) {
            let fc = state.col.primary;
            match fc {
                Color::Rgb([r, g, b]) | Color::Rgbw([r, g, b, _]) => {
                    let xy = XY::from_rgb(r, g, b).0;
                    light.color = Some(LightColor::new(xy));
                    log::trace!("Detected color: {:?}", &light.color);
                }
                Color::None([]) => {}
            }
        }

        light.effects = Some(LightEffects::all());
        light.effects_v2 = Some(LightEffectsV2::all());

        /* light.gradient = gradient.and_then(ExtractLightGradient::extract_from_expose); */
        /* log::trace!("Detected gradient support: {:?}", &light.gradient); */

        self.map.insert(index, link_light);
        self.rmap.insert(link_device, index);
        self.rmap.insert(link_light, index);

        let mut res = self.state.lock().await;
        /* res.aux_set(&link_light, AuxData::new().with_topic(name)); */
        res.add(&link_device, Resource::Device(dev))?;
        res.add(&link_light, Resource::Light(light))?;
        /* res.add(&link_enttm, Resource::Entertainment(enttm))?; */
        /* res.add(&link_taurus, Resource::Taurus(taurus))?; */
        /* res.add(&link_zigcon, Resource::ZigbeeConnectivity(zigcon))?; */
        drop(res);

        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    pub async fn handle_wled_event(&mut self, pkt: tungstenite::Message) -> ApiResult<()> {
        let tungstenite::Message::Text(txt) = pkt else {
            log::error!("[{}] Received non-text message on websocket :(", self.name);
            return Err(ApiError::UnexpectedZ2mReply(pkt));
        };

        let frame: WledFrame = serde_json::from_str(&txt)?;
        if self.info.is_none() {
            self.add_room(&frame.info).await?;

            for (index, (seg, lc)) in frame
                .state
                .seg
                .iter()
                .zip(frame.info.leds.seglc.iter())
                .enumerate()
            {
                self.add_light(index as u8, seg, *lc, &frame.info).await?;
            }
            self.info = Some(frame.info);
        }
        /* log::warn!("WLED: {frame:#?}"); */
        Ok(())
    }

    pub async fn event_loop(
        &mut self,
        chan: &mut Receiver<Arc<BackendRequest>>,
        socket: &mut WledWebSocket,
    ) -> ApiResult<()> {
        loop {
            select! {
                pkt = chan.recv() => {
                    let api_req = pkt?;
                    self.handle_backend_event(socket, api_req).await?;
                },

                pkt = socket.next() => {
                    self.handle_wled_event(pkt.ok_or(ApiError::UnexpectedZ2mEof)??).await?;
                },
            };
        }
    }

    pub async fn add_room(&mut self, info: &WledInfo) -> ApiResult<()> {
        let mac = &info.mac;

        let link_room = RType::Room.deterministic(mac);
        let link_glight = RType::GroupedLight.deterministic(mac);

        self.room = Some(link_room);
        self.glight = Some(link_glight);

        let metadata = RoomMetadata::new(RoomArchetype::Tv, &format!("WLED: {mac}"));

        let mut res = self.state.lock().await;

        if let Ok(room) = res.get::<Room>(&link_room) {
            log::info!(
                "[{}] {link_room:?} ({}) known, updating..",
                self.name,
                room.metadata.name
            );
        } else {
            log::debug!("[{}] {link_room:?} ({}) is new, adding..", self.name, mac);
        }

        let room = Room {
            children: btreeset![self.device_link(0), self.device_link(1)],
            metadata,
            services: btreeset![link_glight],
        };

        for id in &res.get_resource_ids_by_type(RType::BridgeHome) {
            res.update(id, |bh: &mut BridgeHome| {
                bh.children.insert(link_room);
            })?;
        }

        /* self.map.insert(0, link_glight); */
        /* self.rmap.insert(link_glight, 0); */
        /* self.rmap.insert(link_room, 0); */

        res.add(&link_room, Resource::Room(room))?;

        let glight = GroupedLight::new(link_room);

        res.add(&link_glight, Resource::GroupedLight(glight))?;
        drop(res);

        Ok(())
    }
}

#[async_trait]
impl Service for WledBackend {
    type Error = ApiError;

    async fn start(&mut self) -> ApiResult<()> {
        let url = &self.server.url;

        log::info!("[{}] Connecting to {}", self.name, url);
        match connect_async(url.as_str()).await {
            Ok((socket, _)) => {
                self.socket = Some(WledWebSocket::new(self.name.clone(), socket));
                Ok(())
            }
            Err(err) => {
                log::error!("[{}] Connect failed: {err:?}", self.name);
                Err(err.into())
            }
        }
    }

    async fn run(&mut self) -> ApiResult<()> {
        if let Some(ref mut socket) = self.socket.take() {
            let mut chan = self.state.lock().await.backend_event_stream();
            let res = self.event_loop(&mut chan, socket).await;
            if let Err(err) = res {
                log::error!("[{}] Event loop broke: {err}", self.name);
            }
        }
        Ok(())
    }

    async fn stop(&mut self) -> ApiResult<()> {
        self.socket.take();
        Ok(())
    }
}
