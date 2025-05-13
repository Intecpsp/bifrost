use std::str;

use bifrost_frontend::Route;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DARK_THEME_CSS: Asset = asset!("/assets/theme-dark-dracula.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon",       href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DARK_THEME_CSS }
        Router::<Route> {}
    }
}
