#![allow(clippy::needless_pass_by_ref_mut, clippy::unused_async)]

use std::sync::Arc;

use hue::clamp::Clamp;
use uuid::Uuid;

use bifrost_api::backend::BackendRequest;
use hue::api::{
    GroupedLightUpdate, LightEffect, LightEffectsV2Update, LightUpdate, RType, Resource,
    ResourceLink, RoomUpdate, Scene, SceneActive, SceneStatus, SceneStatusEnum, SceneUpdate,
    ZigbeeDeviceDiscoveryUpdate,
};
use hue::stream::HueStreamLightsV2;
use wled::{Color, Colors, StateSegUpdate, StateUpdate};

use crate::backend::wled::WledBackend;
use crate::backend::wled::websocket::WledWebSocket;
use crate::error::ApiResult;
use crate::model::state::AuxData;

#[allow(clippy::match_same_arms)]
pub const fn hue_effect_to_wled(effect: LightEffect) -> Option<u8> {
    /* EffectType::Opal => Some(179), // FX_MODE_FLOWSTRIPE */
    match effect {
        LightEffect::NoEffect => Some(0),    // FX_MODE_STATIC
        LightEffect::Candle => Some(102),    // FX_MODE_CANDLE_MULTI
        LightEffect::Fire => None,           // FIXME
        LightEffect::Prism => None,          // FIXME
        LightEffect::Sparkle => Some(86),    // FX_MODE_SPOTS_FADE
        LightEffect::Opal => Some(8),        // FX_MODE_RAINBOW
        LightEffect::Glisten => Some(109),   // FX_MODE_PHASEDNOISE
        LightEffect::Underwater => Some(75), // FX_MODE_LAKE
        LightEffect::Cosmos => Some(51),     // FX_MODE_FAIRYTWINKLE (+ bg: effect color, fx: black)
        LightEffect::Sunbeam => Some(107),   // FX_MODE_NOISEPAL
        LightEffect::Enchant => None,        // FIXME
    }
}

impl WledBackend {
    async fn backend_light_update(
        &self,
        wledws: &mut WledWebSocket,
        link: &ResourceLink,
        upd: &LightUpdate,
    ) -> ApiResult<()> {
        let Some(index) = self.rmap.get(link) else {
            return Ok(());
        };

        let mut state = StateSegUpdate {
            id: Some(*index),
            on: upd.on(),
            bri: upd
                .dimming()
                .map(|br| Clamp::unit_to_u8_clamped_light(br / 100.0)),
            col: upd.color_xy().map(|xy| {
                let [r, g, b] = xy.to_rgb(255.0);
                Colors {
                    primary: Color::Rgb([r, g, b]),
                    background: Color::NONE,
                    custom: Color::NONE,
                }
            }),
            ..Default::default()
        };

        if let Some(LightEffectsV2Update {
            action: Some(act), ..
        }) = &upd.effects_v2
        {
            if let Some(fx) = act.effect {
                state.fx = hue_effect_to_wled(fx);
            }
            if let Some(speed) = &act.parameters.speed {
                state.sx = Some(speed.unit_to_u8_clamped());
            }
            /* if let Some(mirek) = act.parameters.color_temperature_mirek() { */
            /*     state.cct = mirek; */
            /* } */
            if let Some(color) = &dbg!(act.parameters.color) {
                let [r, g, b] = color.xy.to_rgb(255.0);
                state.col = Some(Colors {
                    primary: Color::Rgb([r, g, b]),
                    background: Color::NONE,
                    custom: Color::NONE,
                });
            }
        }

        wledws.send_state(state).await?;

        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn backend_scene_create(
        &self,
        _wledws: &mut WledWebSocket,
        link_scene: &ResourceLink,
        sid: u32,
        scene: &Scene,
    ) -> ApiResult<()> {
        if Some(scene.group) != self.room {
            return Ok(());
        }

        log::info!("New scene: {link_scene:?} ({})", scene.metadata.name);

        let mut lock = self.state.lock().await;

        let auxdata = AuxData::new()
            .with_topic(&scene.metadata.name)
            .with_index(sid);

        lock.aux_set(link_scene, auxdata);

        /* wledws.send_scene_store(sid as u8).await?; */

        lock.add(link_scene, Resource::Scene(scene.clone()))?;
        drop(lock);

        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn backend_scene_update(
        &mut self,
        wledws: &mut WledWebSocket,
        link: &ResourceLink,
        upd: &SceneUpdate,
    ) -> ApiResult<()> {
        let mut lock = self.state.lock().await;

        let scene = lock.get::<Scene>(link)?.clone();

        if Some(scene.group) != self.room {
            return Ok(());
        }

        if let Some(recall) = &upd.recall {
            if recall.action == Some(SceneStatusEnum::Active) {
                let scenes = lock.get_scenes_for_room(&scene.group);
                for rid in scenes {
                    lock.update::<Scene>(&rid, |scn| {
                        scn.status = Some(SceneStatus {
                            active: if rid == link.rid {
                                SceneActive::Static
                            } else {
                                SceneActive::Inactive
                            },
                            last_recall: None,
                        });
                    })?;
                }

                drop(lock);

                if Some(scene.group) == self.room {
                    log::info!("[{}] Recall scene: {link:?}", self.name);

                    let mut states: Vec<StateSegUpdate> = vec![];

                    for act in &scene.actions {
                        let Some(index) = dbg!(self.rmap.get(&act.target)) else {
                            continue;
                        };

                        let state = StateSegUpdate {
                            id: Some(*index),
                            on: act.action.on.map(|on| on.on),
                            bri: act
                                .action
                                .dimming
                                .map(|br| Clamp::unit_to_u8_clamped_light(br.brightness / 100.0)),
                            col: act.action.color.map(|col| {
                                let [r, g, b] = col.xy.to_rgb(255.0);
                                Colors {
                                    primary: Color::Rgb([r, g, b]),
                                    background: Color::NONE,
                                    custom: Color::NONE,
                                }
                            }),
                            ..Default::default()
                        };
                        states.push(state);
                    }

                    if !states.is_empty() {
                        wledws.send_states(&states).await?;
                    }
                }
            } else {
                log::error!("Scene recall type not supported: {recall:?}");
            }
        } else {
            // We're not recalling the scene, so we are updating the scene
            log::info!("[{}] Store scene: {link:?}", self.name);

            // Wled scenes are stored entirely in Bifrost, so update the
            // state database accordingly
            lock.update::<Scene>(&link.rid, |scene| {
                *scene += upd;
            })?;

            drop(lock);
        }

        Ok(())
    }

    async fn backend_grouped_light_update(
        &self,
        wledws: &mut WledWebSocket,
        link: &ResourceLink,
        upd: &GroupedLightUpdate,
    ) -> ApiResult<()> {
        if Some(*link) != self.glight {
            return Ok(());
        }

        let state = StateUpdate {
            on: upd.on(),
            bri: upd
                .dimming()
                .map(|br| Clamp::unit_to_u8_clamped_light(br / 100.0)),
        };

        wledws.send_group_state(state).await
    }

    async fn backend_room_update(
        &self,
        _wledws: &mut WledWebSocket,
        _link: &ResourceLink,
        _upd: &RoomUpdate,
    ) -> ApiResult<()> {
        Ok(())
    }

    async fn backend_delete(
        &mut self,
        _wledws: &mut WledWebSocket,
        link: &ResourceLink,
    ) -> ApiResult<()> {
        match link.rtype {
            RType::Scene => {
                let mut lock = self.state.lock().await;
                lock.delete(link)?;
                lock.aux_remove(link);
                drop(lock);
            }

            rtype => {
                log::warn!(
                    "[{}] Deleting objects of type {rtype:?} is not supported",
                    self.name
                );
            }
        }
        Ok(())
    }

    async fn backend_entertainment_start(
        &mut self,
        _wledws: &mut WledWebSocket,
        _ent_id: &Uuid,
    ) -> ApiResult<()> {
        Ok(())
    }

    async fn backend_entertainment_frame(
        &mut self,
        _wledws: &mut WledWebSocket,
        _frame: &HueStreamLightsV2,
    ) -> ApiResult<()> {
        Ok(())
    }

    async fn backend_entertainment_stop(&mut self, _wledws: &mut WledWebSocket) -> ApiResult<()> {
        Ok(())
    }

    async fn backend_zigbee_device_discovery(
        &self,
        _z2mws: &mut WledWebSocket,
        _rlink: &ResourceLink,
        _zbd: &ZigbeeDeviceDiscoveryUpdate,
    ) -> ApiResult<()> {
        Ok(())
    }

    pub async fn handle_backend_event(
        &mut self,
        wledws: &mut WledWebSocket,
        req: Arc<BackendRequest>,
    ) -> ApiResult<()> {
        /* self.learner.cleanup(); */

        match &*req {
            BackendRequest::LightUpdate(link, upd) => {
                self.backend_light_update(wledws, link, upd).await
            }

            BackendRequest::SceneCreate(link, sid, scene) => {
                self.backend_scene_create(wledws, link, *sid, scene).await
            }

            BackendRequest::SceneUpdate(link, upd) => {
                self.backend_scene_update(wledws, link, upd).await
            }

            BackendRequest::GroupedLightUpdate(link, upd) => {
                self.backend_grouped_light_update(wledws, link, upd).await
            }

            BackendRequest::RoomUpdate(link, upd) => {
                self.backend_room_update(wledws, link, upd).await
            }

            BackendRequest::Delete(link) => self.backend_delete(wledws, link).await,

            BackendRequest::EntertainmentStart(ent_id) => {
                self.backend_entertainment_start(wledws, ent_id).await
            }

            BackendRequest::EntertainmentFrame(frame) => {
                self.backend_entertainment_frame(wledws, frame).await
            }

            BackendRequest::EntertainmentStop() => self.backend_entertainment_stop(wledws).await,

            BackendRequest::ZigbeeDeviceDiscovery(rlink, zbd) => {
                self.backend_zigbee_device_discovery(wledws, rlink, zbd)
                    .await
            }
        }
    }
}
