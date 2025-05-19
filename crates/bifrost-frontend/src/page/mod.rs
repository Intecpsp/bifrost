pub mod footer;
pub mod sidebar;

use std::collections::{BTreeMap, VecDeque};

use dioxus::prelude::*;

use bifrost_api::error::BifrostError;
use bifrost_api::service::ServiceList;
use hue::api::RoomArchetype;

use crate::icons::RoomIcon;
use crate::page::sidebar::Sidebar;
use crate::state::State;
use crate::toast::{ToastFrame, ToastMaster};
use crate::{Route, use_context_signal_provider};

#[component]
pub fn Index() -> Element {
    rsx! {
        section {
            "data-theme": "dark",
            class: "p-5",
            RoomIcon {
                archetype: RoomArchetype::Balcony,
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            "data-theme": "dark",
            class: "p-5",
            "This is the about page"
        }
    }
}

#[component]
pub fn Frame() -> Element {
    let slist = use_context_signal_provider(ServiceList::default);
    let config = use_context_signal_provider(|| Err(BifrostError::ServerError("foo".to_string())));
    let toast = use_context_signal_provider(ToastMaster::new);
    let hue = use_context_signal_provider(BTreeMap::new);
    let ent = use_context_signal_provider(|| Option::None);
    let log = use_context_signal_provider(|| VecDeque::with_capacity(1024));

    let _ws = use_future(move || {
        let state = State::new(slist, config, toast, hue, ent, log);
        async move { state.run_websocket().await }
    });

    rsx! {
        div {
            class: "flex",
            class: "lg:gap-4",
            class: "flex-col",
            class: "lg:flex-row",

            Sidebar {}

            div {
                class: "lg:grow",
                class: "py-4",
                class: "px-4",
                class: "lg:pl-0",
                ToastFrame { master: toast }
                Outlet::<Route> {}
            }
        }
    }
}
