use dioxus::prelude::*;

use crate::icons::{IconDiscord, IconGithub};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer p-5",
            nav {
                h6 {
                    class: "footer-title",
                    "Bifrost"
                }
                a {
                    class: "link link-hover",
                    href: "https://github.com/chrivers/bifrost",
                    IconGithub {}
                    "github.com/chrivers/bifrost"
                }
                a {
                    class: "link link-hover",
                    href: "https://discord.gg/YvBKjHBJpA",
                    IconDiscord {}
                    "Join us on Discord"
                }
            }
        }
    }
}
