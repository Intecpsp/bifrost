use std::collections::{BTreeMap, VecDeque};
use std::time::Duration;

use bifrost_api::logging::LogRecord;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use gloo_timers::future::sleep;
use uuid::Uuid;

use bifrost_api::backend::BackendRequest;
use bifrost_api::config::AppConfig;
use bifrost_api::error::BifrostError;
use bifrost_api::service::ServiceList;
use bifrost_api::websocket::Update;
use hue::api::{Resource, ResourceRecord};
use hue::diff::event_update_apply;
use hue::event::{Event, EventBlock};
use hue::stream::HueStreamLightsV2;

use crate::BIFROST_SERVER;
use crate::toast::{Toast, ToastMaster};

#[derive(Clone)]
pub struct State {
    slist: Signal<ServiceList>,
    config: Signal<Result<AppConfig, BifrostError>>,
    toast: Signal<ToastMaster>,
    hue: Signal<BTreeMap<Uuid, ResourceRecord>>,
    ent: Signal<Option<HueStreamLightsV2>>,
    log: Signal<VecDeque<LogRecord>>,
}

#[allow(clippy::future_not_send)]
impl State {
    #[must_use]
    pub const fn new(
        slist: Signal<ServiceList>,
        config: Signal<Result<AppConfig, BifrostError>>,
        toast: Signal<ToastMaster>,
        hue: Signal<BTreeMap<Uuid, ResourceRecord>>,
        ent: Signal<Option<HueStreamLightsV2>>,
        log: Signal<VecDeque<LogRecord>>,
    ) -> Self {
        Self {
            slist,
            config,
            toast,
            hue,
            ent,
            log,
        }
    }

    fn clear(&mut self) {
        self.slist.write().services.clear();
        self.config
            .set(Err(BifrostError::ServerError(String::from("Not ready"))));
        self.hue.write().clear();
    }

    #[allow(clippy::cognitive_complexity)]
    fn handle_hue_event(&mut self, he: EventBlock) {
        match he.event {
            Event::Add(add) => {
                self.toast.write().add(
                    Toast::info(rsx! { "Discovered {add.data.len()} items" })
                        .with_timeout(chrono::Duration::seconds(3)),
                );
                let mut hue = self.hue.write();
                for item in add.data {
                    hue.insert(item.id, item);
                }
            }
            Event::Update(update) => {
                let mut hue = self.hue.write();
                for upd in update.data {
                    let Some(res) = &mut hue.get_mut(&upd.id) else {
                        continue;
                    };

                    match &mut res.obj {
                        Resource::Light(light) => {
                            if let Ok(new) = event_update_apply(light, upd.data.clone()) {
                                /* tracing::info!("upd: {upd:#?}"); */
                                *light = new;
                            } else {
                                tracing::error!("light: {:#?} upd: {:#?}", light, upd.data);
                                tracing::error!(
                                    "light: {:#?} upd: {:#?}",
                                    serde_json::to_string(&light),
                                    serde_json::to_string(&upd.data)
                                );
                            }

                            /* if let Ok(hue::api::Update::Light(upd)) = */
                            /*     serde_json::from_value(upd.data.clone()) */
                            /* { */
                            /*     /\* tracing::info!("upd: {upd:#?}"); *\/ */
                            /*     *light = event_apply_diff(light, upd)? */
                            /* } else { */
                            /*     tracing::error!("upd: {upd:#?}"); */
                            /* } */
                        }
                        Resource::Room(room) => {
                            if let Ok(new) = event_update_apply(room, upd.data) {
                                /* tracing::info!("upd: {upd:#?}"); */
                                *room = new;
                            }
                        }
                        Resource::Scene(scene) => {
                            if let Ok(new) = event_update_apply(scene, upd.data) {
                                *scene = new;
                            }
                        }
                        Resource::GroupedLight(obj) => {
                            if let Ok(new) = event_update_apply(obj, upd.data) {
                                *obj = new;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Delete(delete) => {
                let mut hue = self.hue.write();
                for del in delete.data {
                    hue.remove(&del.id);
                }
                /* tracing::warn!("delete: {delete:?}"); */
            }
            Event::Error(error) => {
                self.toast.write().add(
                    Toast::error(rsx! { "Error: {error:?}" })
                        .with_timeout(chrono::Duration::seconds(3)),
                );
            }
        }
    }

    fn handle_backend_request(&mut self, br: BackendRequest) {
        /* tracing::info!("BR: {br:?}"); */
        match br {
            BackendRequest::LightUpdate(_resource_link, _light_update) => {}
            BackendRequest::SceneCreate(_resource_link, _idx, _scene) => {}
            BackendRequest::SceneUpdate(_resource_link, scene_update) => {
                if scene_update.recall.is_some() {
                    self.toast
                        .write()
                        .add(Toast::info(rsx! {"Recalling scene.."}).with_timeout_secs(2));
                }
            }
            BackendRequest::GroupedLightUpdate(_resource_link, _grouped_light_update) => {}
            BackendRequest::RoomUpdate(_resource_link, _room_update) => {}
            BackendRequest::Delete(_resource_link) => {}

            BackendRequest::EntertainmentStart(_uuid) => {
                *self.ent.write() = None;
                self.toast.write().add(
                    Toast::info(rsx! {"Starting entertainment stream.."}).with_timeout_secs(2),
                );
            }
            BackendRequest::EntertainmentFrame(hue_stream_lights) => {
                *self.ent.write() = Some(hue_stream_lights);
            }
            BackendRequest::EntertainmentStop() => {
                /* *self.ent.write() = None; */
                self.toast
                    .write()
                    .add(Toast::info(rsx! {"Entertainment stream done."}).with_timeout_secs(2));
            }
            BackendRequest::ZigbeeDeviceDiscovery(
                _resource_link,
                _zigbee_device_discovery_update,
            ) => { /* */ }
        }
    }

    fn handle_log_event(&mut self, evt: LogRecord) {
        let mut log = self.log.write();

        if log.len() == log.capacity() {
            log.pop_front();
        }

        log.push_back(evt);
    }

    async fn run_connection(&mut self, mut ws: WebSocket) {
        let mut first = true;

        loop {
            let next = ws.next().await;
            let Some(Ok(msg)) = next else {
                break;
            };

            if first {
                first = false;
                self.toast
                    .write()
                    .add(Toast::success(rsx! { "Websocket connected" }).with_timeout_secs(4));
            }

            match msg {
                Message::Text(text) => {
                    let upd: Update = serde_json::from_str(&text).unwrap();

                    match upd {
                        Update::AppConfig(ac) => {
                            self.config.set(Ok(ac));
                        }
                        Update::HueEvent(he) => {
                            self.handle_hue_event(he);
                            /* self.toast.write().add( */
                            /*     Toast::info(rsx! { "{he.event:?}" }) */
                            /*         .with_timeout(chrono::Duration::seconds(3)), */
                            /* ); */
                        }
                        Update::BackendRequest(br) => {
                            self.handle_backend_request(br);
                        }
                        Update::ServiceUpdate(su) => {
                            self.slist.write().services.insert(su.id, su);
                        }
                        Update::LogEvent(evt) => {
                            self.handle_log_event(evt);
                        }
                    }
                }
                Message::Bytes(_) => {}
            }
        }
    }

    pub async fn run_websocket(mut self) {
        loop {
            let ws = match WebSocket::open(&format!("{}/bifrost/ws", *BIFROST_SERVER)) {
                Ok(ws) => ws,
                Err(e) => {
                    tracing::error!("Could not open websocket connection: {e}");
                    return;
                }
            };

            self.run_connection(ws).await;
            tracing::error!("Websocket disconnected. Retrying..");
            self.toast.write().add(
                Toast::error(rsx! { "Websocket disconnected. Retrying.." })
                    .with_timeout(chrono::Duration::seconds(2)),
            );

            self.clear();

            sleep(Duration::from_secs(2)).await;
        }
    }
}
