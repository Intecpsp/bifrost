use dioxus::prelude::*;

use bifrost_api::export::svc::traits::ServiceState;
use bifrost_api::service::{Service, ServiceList};

use crate::CLIENT;
use crate::component::button::Button;

#[component]
pub fn ServiceStopButton(svc: Service) -> Element {
    rsx! {
        Button {
            onclick: move |_| async move {
                CLIENT.service_stop(svc.id).await?;
                Ok(())
            },
            disabled: svc.state != ServiceState::Running,
            "Stop"
        }
    }
}

#[component]
pub fn ServiceStartButton(svc: Service) -> Element {
    rsx! {
        Button {
            onclick: move |_| async move {
                CLIENT.service_start(svc.id).await?;
                Ok(())
            },
            disabled: svc.state != ServiceState::Stopped,
            "Start"
        }
    }
}

#[component]
pub fn ServiceStateIcon(state: ServiceState) -> Element {
    match state {
        ServiceState::Configured | ServiceState::Registered | ServiceState::Starting => rsx! {
            div { class: "status status-warning" }
        },

        ServiceState::Running => rsx! {
            div { class: "status status-success" }
        },

        ServiceState::Stopping | ServiceState::Stopped | ServiceState::Failed => rsx! {
            div { class: "status status-error" }
        },
    }
}

#[component]
pub fn ServicesView(svcs: ServiceList) -> Element {
    rsx! {
        h2 { class: "card-title", "Services" }

        div {
            div {
               div {
                    span { "Name"  }
                    span { "State" }
                    span {         }
                    span {         }
                }
            }
            div {
                class: "*:odd:bg-base-200",
                for (uuid, svc) in svcs.services {
                    div {
                        class: "flex flex-col-auto gap-4 p-4 items-baseline",
                        key: "{uuid}",
                        div { class:"grow", "{svc.name}"    }
                        div { ServiceStateIcon { state: svc.state } " {svc.state:?}" }
                        div { ServiceStopButton { svc: svc.clone() } }
                        div { ServiceStartButton { svc: svc } }
                    }
                }
            }
        }
    }
}

/* #[component] */
/* pub fn ServicesView(svcs: ServiceList) -> Element { */
/*     rsx! { */
/*         h2 { class: "card-title", "Services" } */

/*         table { */
/*             class: "table table-zebra", */
/*             thead { */
/*                 tr { */
/*                     th { "Name"  } */
/*                     th { "State" } */
/*                     th {         } */
/*                     th {         } */
/*                 } */
/*             } */
/*             tbody { */
/*                 for (uuid, svc) in svcs.services { */
/*                     tr { */
/*                         key: "{uuid}", */
/*                         td { "{svc.name}"    } */
/*                         td { ServiceStateIcon { state: svc.state } " {svc.state:?}" } */
/*                         td { ServiceStopButton { svc: svc.clone() } } */
/*                         td { ServiceStartButton { svc: svc } } */
/*                     } */
/*                 } */
/*             } */
/*         } */
/*     } */
/* } */
