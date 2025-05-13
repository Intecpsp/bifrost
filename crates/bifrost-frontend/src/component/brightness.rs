use dioxus::prelude::*;

use crate::traits::Optional;

#[component]
pub fn Brightness(
    value: Option<u32>,
    onchange: Option<EventHandler<FormEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    rsx! {
        div {
            class: "grow self-end",
            input {
                type: "range",
                class: "range",
                class: "range-xs",
                class: "w-full",
                class: "[--range-fill:0]",
                style: "background: linear-gradient(90deg in oklab, black, white)",
                min: "0",
                max: "100",
                onchange: move |evt| onchange.call_if_some(evt),
                oninput: move |evt| oninput.call_if_some(evt),
                value,
            }
            div { class: "flex justify-between px-2 mt-2 text-xs",
                  span { "|" }
                  span { "|" }
                  span { "|" }
            }
            div { class: "flex justify-between px-2 mt-2 text-xs *:w-7",
                  span { "0%" }
                  span { "50%" }
                  span { "100%" }
            }
        }
    }
}
