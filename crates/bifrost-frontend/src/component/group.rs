use std::collections::BTreeMap;

use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{Resource, ResourceRecord, Room, Scene, SceneActive, SceneStatus, SceneUpdate};
use hue::colorspace::WIDE;
use hue::colortemp::cct_to_xy;
use hue::xy::XY;

use crate::HUE_CLIENT;
use crate::component::colortemp::mirek_to_kelvin;
use crate::daisyui::Level;
use crate::daisyui::badge::Badge;
use crate::icons::RoomIcon;

fn css_rgb_from_xy(xy: XY) -> String {
    let [x, y, z] = WIDE.xyy_to_xyz(xy.x, xy.y, 0.7);
    format!("color(xyz-d65 {x:.3} {y:.3} {z:.3})")
    /* let rgb = SRGB */
    /*     .xy_to_rgb_color(xy.x, xy.y, 100.0) */
    /*     .map(Clamp::unit_to_u8_clamped); */
    /* format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]) */
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
        RoomIcon {
            archetype: room.metadata.archetype,
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
