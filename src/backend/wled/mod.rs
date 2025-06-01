mod backend_event;
pub mod entertainment;
pub mod error;
pub mod websocket;
pub mod wled_import;

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
    BridgeHome, GroupedLight, RType, Resource, ResourceLink, Room, RoomArchetype, RoomMetadata,
};
use svc::error::SvcError;
use svc::template::ServiceTemplate;
use svc::traits::{BoxDynService, Service};
use wled::{WledFrame, WledInfo};

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
