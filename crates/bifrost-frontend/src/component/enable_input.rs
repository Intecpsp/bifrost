use dioxus::prelude::*;

use crate::traits::Negateable;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default)]
    class: &'static [&'static str],
    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,
    #[props(optional)]
    label: Option<Element>,
    #[props(optional, default = None)]
    children: Element,
    enabled: Signal<bool>,
}

#[component]
pub fn EnableInput(mut props: ButtonProps) -> Element {
    rsx! {
        label {
            class: "fieldset-label",
            input {
                type: "checkbox",
                class: "toggle mr-2",
                checked: props.enabled,
                onclick: move |_evt| props.enabled.negate()
            }
            {props.label},
        }
        fieldset {
            class: "fieldset",
            {props.children}
        }
    }
}
