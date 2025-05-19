use std::collections::BTreeMap;

use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{Resource, ResourceRecord};

use crate::component::group::GroupView;
use crate::daisyui::Level;
use crate::daisyui::badge::Badge;
use crate::use_context_signal;

#[component]
pub fn GroupsView() -> Element {
    let rres = use_context_signal::<BTreeMap<Uuid, ResourceRecord>>();
    let res = &*rres.read();

    rsx! {
        div {
            class: "grid gap-4",
            class: "max-w-200",
            for (uuid, item) in res {

                if let Resource::Room(room) = &item.obj {
                    div {
                        class: "max-w-220",
                        key: "{uuid}",
                        div {
                            class: "bg-base-200 w-full p-5 border-b-2 border-primary/40 rounded-t-xl",
                            div {
                                class: "flex flex-col lg:flex-row gap-4 *:text-nowrap",
                                div {
                                    class: "grow",
                                    Badge {
                                        level: Level::Info,
                                        soft: true,
                                        class: "font-mono",
                                        "{room.metadata.name}"
                                    }
                                }

                                div {
                                    class: "badge badge-soft badge-info font-mono",
                                    { item.id_v1.as_deref().unwrap_or("-") }
                                }
                                "|",
                                div {
                                    Badge {
                                        level: Level::Info,
                                        soft: true,
                                        class: "font-mono",
                                        "{uuid}"
                                    }
                                }
                            }
                        }
                        div {
                            class: "bg-base-300 p-5 rounded-b-xl",
                            GroupView { res: rres, id: *uuid, room: room.clone() }
                        }
                    }
                }
            }
        }
    }
}
