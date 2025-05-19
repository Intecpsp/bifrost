use std::collections::BTreeMap;

use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{Resource, ResourceRecord};

use crate::component::light::LightView;
use crate::{Route, use_context_signal};

#[component]
pub fn LightsView() -> Element {
    let res = use_context_signal::<BTreeMap<Uuid, ResourceRecord>>();

    rsx! {
        h2 { class: "card-title", "Lights" }

        div {
            for (uuid, item) in &*res.read() {
                if let Resource::Light(light) = &item.obj {
                    div {
                        class: "max-w-220",
                        key: "{uuid}",
                        div {
                            class: "bg-base-200 w-full mt-4 p-5 border-b-2 border-primary/40 rounded-t-xl",

                            div {
                                class: "flex gap-4 *:text-nowrap",
                                Link {
                                    class: "badge badge-soft badge-info font-mono",
                                    to: Route::LightDetailView { id: *uuid },
                                    { item.id_v1.as_deref().unwrap_or("-") }
                                },
                                "|",
                                div {
                                    class: "badge badge-soft font-mono",
                                    "{light.metadata.name}"
                                }
                                "|",
                                span { class: "badge badge-soft font-mono", "{uuid}" }
                            }
                        }
                        div {
                            class: "bg-base-300 p-5 rounded-b-xl",
                            LightView { id: *uuid, light: light.clone() }
                        }
                    }
                }
            }
        }
    }
}
