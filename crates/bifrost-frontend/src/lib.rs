#![allow(non_snake_case)]
#![allow(unused_qualifications)]

pub mod component;
pub mod daisyui;
pub mod grommet_icons;
pub mod hue_client;
pub mod icons;
pub mod state;
pub mod toast;
pub mod traits;

use std::sync::LazyLock;

use dioxus::prelude::*;
use reqwest::Url;

use bifrost_api::Client;

static BIFROST_SERVER: LazyLock<String> =
    LazyLock::new(|| option_env!("BIFROST_SERVER").map_or_else(base_url, ToString::to_string));

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::from_url(Url::try_from(format!("{}/bifrost/", *BIFROST_SERVER).as_str()).unwrap())
});

const LOGO_SVG: Asset = asset!("/assets/logo.svg");

#[must_use]
pub fn base_url() -> String {
    web_sys::window().unwrap().location().origin().unwrap()
}

pub fn use_context_signal_provider<T: 'static>(f: impl FnOnce() -> T) -> Signal<T, UnsyncStorage> {
    let inner = use_signal(f);
    use_context_provider(move || inner)
}

#[must_use]
pub fn use_context_signal<T>() -> Signal<T> {
    use_context::<Signal<T>>()
}
