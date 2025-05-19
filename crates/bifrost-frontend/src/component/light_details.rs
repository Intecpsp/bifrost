use std::collections::BTreeMap;

use dioxus::prelude::*;

use hue::api::{Resource, ResourceRecord};
use uuid::Uuid;

use crate::component::light::{
    LightColorTemperature, LightColorView, LightDimming, LightGradientView, LightOnIcon,
};
use crate::{Route, use_context_signal};

#[component]
pub fn LightDetailView(id: Uuid) -> Element {
    let res = use_context_signal::<BTreeMap<Uuid, ResourceRecord>>();

    let Some(Resource::Light(light)) = res.read().get(&id).cloned().map(|x| x.obj) else {
        return rsx! {};
    };

    rsx! {
        div {
            class: "max-w-160",

            Link { to: Route::LightsView, class: "btn btn-primary", "Back" },

            h2 { class: "divider divider-primary divider-start", "On" }
            LightOnIcon { id, on: light.on }

            if light.color.is_some() {
                h2 { class: "divider divider-primary divider-start", "Color" }
                LightColorView { id, dimming: light.dimming, color: light.color }
            }

            if light.dimming.is_some() {
                h2 { class: "divider divider-primary divider-start", "Dimming" }
                LightDimming { id, dimming: light.dimming }
            }

            if light.color_temperature.is_some() {
                h2 { class: "divider divider-primary divider-start", "Color temperature" }
                LightColorTemperature { id, color_temperature: light.color_temperature }
            }

            if light.gradient.is_some() {
                h2 { class: "divider divider-primary divider-start", "Gradient" }
                LightGradientView { id, gradient: light.gradient }
            }
        }
    }
}
