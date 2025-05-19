use dioxus::prelude::*;

use bifrost_api::export::svc::traits::ServiceState;
use bifrost_api::service::{Service, ServiceList};
use uuid::Uuid;

use crate::{CLIENT, use_context_signal};

#[component]
pub fn ServiceStopButton(svc: Service) -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
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
        button {
            class: "btn btn-primary",
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
pub fn ServiceCard(uuid: Uuid, svc: Service) -> Element {
    rsx! {
        div {
            class: "flex flex-col-auto gap-4 p-4 items-baseline",
            key: "{uuid}",
            div { class:"grow",
                  if let Some(inst) = svc.name.instance() {
                      "{svc.name.name()}",
                      span { class: "badge badge-success font-mono mx-2 px-1", "Instance: {inst}" }
                  } else {
                      "{svc.name}"
                  }
            }
            div { ServiceStateIcon { state: svc.state } " {svc.state:?}" }
            div { ServiceStopButton { svc: svc.clone() } }
            div { ServiceStartButton { svc: svc } }
        }
    }
}

#[component]
pub fn ServicesView() -> Element {
    let slist = use_context_signal::<ServiceList>();
    let svcs = slist.read();

    let mut svcs: Vec<(_, _)> = svcs.services.iter().collect();
    svcs.sort_by(|a, b| Ord::cmp(a.1.name.name(), b.1.name.name()));

    rsx! {
        h2 { class: "card-title", "Services" }

        div {
            class: "*:odd:bg-base-200",
            class: "max-w-200",
            for (uuid, svc) in svcs {
                ServiceCard { uuid: *uuid, svc: svc.clone() }
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
