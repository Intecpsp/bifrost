use std::collections::BTreeMap;

use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{
    ColorTemperatureUpdate, DimmingUpdate, GroupedLightUpdate, MirekSchema, On, Resource,
    ResourceRecord, Room, Scene, SceneActive, SceneStatus, SceneUpdate,
};
use hue::colorspace::WIDE;
use hue::colortemp::cct_to_xy;
use hue::xy::XY;

use crate::HUE_CLIENT;
use crate::component::brightness::Brightness;
use crate::component::colortemp::{ColorTemp, mirek_to_kelvin};
use crate::daisyui::Level;
use crate::daisyui::badge::Badge;

fn css_rgb_from_xy(xy: XY) -> String {
    let [x, y, z] = WIDE.xyy_to_xyz(xy.x, xy.y, 0.7);
    format!("color(xyz-d65 {x:.3} {y:.3} {z:.3})")
    /* let rgb = SRGB */
    /*     .xy_to_rgb_color(xy.x, xy.y, 100.0) */
    /*     .map(Clamp::unit_to_u8_clamped); */
    /* format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]) */
}

#[component]
pub fn GroupOnIcon(id: Uuid, on: On) -> Element {
    rsx! {
        input {
            type: "checkbox",
            class: "toggle",
            checked: on.on,
            onclick: move |evt| {
                evt.prevent_default();

                let upd = GroupedLightUpdate::new().with_on(Some(On { on: !on.on }));

                async move {
                    HUE_CLIENT.grouped_light_update(id, upd).await?;
                    Ok(())
                }
            },
        }
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[component]
pub fn LightDimming(id: Uuid, dimming: Option<DimmingUpdate>) -> Element {
    let Some(dim) = &dimming else {
        return rsx! {};
    };

    rsx! {
        Brightness {
            onchange: move |evt: FormEvent| {
                evt.prevent_default();
                evt.stop_propagation();
                let bri: f64 = evt.value().parse().unwrap();
                async move {
                    HUE_CLIENT.grouped_light_update(id, GroupedLightUpdate::new().with_brightness(Some(bri))).await?;
                    Ok(())
                }
            },
            value: dim.brightness as u32,
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn LightColorTemperature(
    id: Uuid,
    color_temperature: Option<ColorTemperatureUpdate>,
) -> Element {
    let Some(ct) = &color_temperature else {
        return rsx! {};
    };

    rsx! {
        ColorTemp {
            range: MirekSchema {
                mirek_minimum: 153,
                mirek_maximum: 500,
            },
            onchange: move |evt: FormEvent| {
                evt.prevent_default();

                let ct: u16 = evt.value().parse().unwrap();
                async move {
                    let upd = GroupedLightUpdate::new()
                        .with_color_temperature(Some(ct));

                    HUE_CLIENT.grouped_light_update(id, upd).await?;
                    Ok(())
                }
            },
            value: ct.mirek.map(u32::from),
        }
    }
}

fn css_scene_style(scene: &Scene) -> String {
    let mut parts = vec![];

    if let Some(pal) = &scene.palette {
        if !pal.color.is_empty() {
            for c in &pal.color {
                parts.push(c.color.xy);
            }
        } else if !pal.color_temperature.is_empty() {
            for c in &pal.color_temperature {
                let kelvin = mirek_to_kelvin(u32::from(c.color_temperature.mirek));
                let xy = cct_to_xy(f64::from(kelvin));
                parts.push(xy);
            }
        }
    } else {
        for act in &scene.actions {
            if let Some(c) = act.action.color {
                parts.push(c.xy);
            } else if let Some(ColorTemperatureUpdate { mirek: Some(c) }) =
                act.action.color_temperature
            {
                let kelvin = mirek_to_kelvin(u32::from(c));
                let xy = cct_to_xy(f64::from(kelvin));
                parts.push(xy);
            }
        }
    }

    let gradient = parts
        .into_iter()
        .map(css_rgb_from_xy)
        .collect::<Vec<_>>()
        .join(",");

    format!("background: linear-gradient(315deg in xyz, {gradient})")
}

#[component]
pub fn GroupView(res: Signal<BTreeMap<Uuid, ResourceRecord>>, id: Uuid, room: Room) -> Element {
    let read = res.read();

    let scenes: BTreeMap<Uuid, Scene> = read
        .iter()
        .filter_map(|(k, v)| {
            if let Resource::Scene(scn) = &v.obj {
                if scn.group.rid == id {
                    Some((*k, scn.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    rsx! {
        if let Some(gl) = room.grouped_light_service() {
            if let Some(ResourceRecord {
                obj: Resource::GroupedLight(grp),
                ..
            }) = read.get(&gl.rid) {
                div {
                    class: "flex flex-col-2 gap-4",
                    LightDimming { id: gl.rid, dimming: grp.dimming }
                    LightColorTemperature { id: gl.rid, color_temperature: grp.color_temperature }
                }
            }
        }

        ul {
            for child in room.children {
                li {
                    class: "list-item",
                    "Member: ",
                    Badge {
                        class: "inline font-mono text-nowrap text-xs",
                        level: Level::Primary,
                        "{child:?}"
                    }
                }
            }
        }

        div {
            class: "flex flex-row gap-2",
            for (scene_id, scene) in scenes {
                button {
                    class: "btn btn-scene text-nowrap w-20 h-15",
                    style: css_scene_style(&scene),
                    onclick: move |evt| {
                        evt.prevent_default();

                        let upd = SceneUpdate::new().with_recall_action(Some(
                            SceneStatus { active: SceneActive::Static, last_recall: None }
                        ));

                        async move {
                            HUE_CLIENT.scene_update(scene_id, upd).await?;
                            Ok(())
                        }
                    },

                    "{scene.metadata.name}"
                }
            }
        }
    }
}
