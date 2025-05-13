use dioxus::prelude::*;

use bifrost_api::config::AppConfig;
use bifrost_api::error::BifrostError;

use crate::component::z2m::{Z2mServerAdd, Z2mServerView};
use crate::icons::{IconChevronDown, IconChevronUp};
use crate::traits::Negateable;
use crate::use_context_signal;

#[component]
pub fn Backends() -> Element {
    let config = use_context_signal::<Result<AppConfig, BifrostError>>();
    let mut show_add = use_signal(|| false);

    match &*config.read() {
        Ok(cfg) => {
            rsx! {
                div {
                    class: "flex flex-col gap-4",


                    for (name, server) in cfg.z2m.servers.clone() {
                        div {
                            Z2mServerView { name, server }
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_add.negate(),
                        if show_add() {
                            IconChevronUp {}
                            "Cancel"
                        } else {
                            IconChevronDown {}
                            "Add new backend"
                        }
                    }

                    Z2mServerAdd { show: show_add }
                }
            }
        }

        Err(_err) => rsx! {
            div {
                class: "skeleton",
                class: "w-64",
                class: "h-32",
            }
        },
    }
}
