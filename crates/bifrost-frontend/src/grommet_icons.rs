//! Grommet icon wrappers
//!
//! These icons are Apache-licensed.

use dioxus::prelude::*;

use crate::icons::SvgIcon;

#[component]
pub fn GrServices() -> Element {
    rsx! {
        SvgIcon {
            path: "M6 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm0-6V0m0 12V9M0 6h3m6 0h3M2 2l2 2m4 4 2 2m0-8L8 4M4 8l-2 2m16 2a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm0-6V3m0 12v-3m-6-3h3m6 0h3M14 5l2 2m4 4 2 2m0-8-2 2m-4 4-2 2m-5 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm0-6v-3m0 12v-3m-6-3h3m6 0h3M5 14l2 2m4 4 2 2m0-8-2 2m-4 4-2 2"
        }
    }
}

#[component]
pub fn GrDatabase() -> Element {
    rsx! {
        SvgIcon {
            path: "M1 2h22v7H1V2zm3 10h1v1H4v-1zm0-7h1v1H4V5zm0 14h1v1H4v-1zm-3-3h22v7H1v-7zm0-7h22v7H1V9z"
        }
    }
}

#[component]
pub fn GrStorage() -> Element {
    rsx! {
        SvgIcon {
            path: "M2 5.077S3.667 2 12 2s10 3.077 10 3.077v13.846S20.333 22 12 22 2 18.923 2 18.923V5.077zM2 13s3.333 3 10 3 10-3 10-3M2 7s3.333 3 10 3 10-3 10-3"
        }
    }
}

#[component]
pub fn GrInfo() -> Element {
    rsx! {
        SvgIcon {
            path: "M15 17c0-3 4-5 4-9s-3-7-7-7-7 3-7 7 4 6 4 9v3c0 2 1 3 3 3s3-1 3-3v-3zm-6 1h6"
        }
    }
}

#[component]
pub fn GrIteration() -> Element {
    rsx! {
        SvgIcon {
            path: "M1 9v14h14M5 5v14h14M9 15h14V1H9v14z"
        }
    }
}

#[component]
pub fn GrMultiple() -> Element {
    rsx! {
        SvgIcon {
            path: "M19 15h4V1H9v4m6 14h4V5H5v4M1 23h14V9H1v14z"
        }
    }
}

#[component]
pub fn GrSystem() -> Element {
    rsx! {
        SvgIcon {
            path: "M1 19h22V1H1v18zm4 4h14H5zm3 0h8v-4H8v4zM7.757 5.757l2.122 2.122-2.122-2.122zM9 10H6h3zm.879 2.121-2.122 2.122 2.122-2.122zM12 13v3-3zm2.121-.879 2.122 2.122-2.122-2.122zM18 10h-3 3zm-1.757-4.243-2.122 2.122 2.122-2.122zM12 7V4v3zm0 0a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"
        }
    }
}

#[component]
pub fn GrAction() -> Element {
    rsx! {
        SvgIcon {
            path: "m1 23 3-3-3 3zM20 4l3-3-3 3zM9 11l3-3-3 3zm4 4 3-3-3 3zM10 5l9 9 1-1c2-2 4.053-5 0-9s-7-2-9 0l-1 1zm-6 6 1-1 9 9-1 1c-2 2-5 4.087-9 0s-2-7 0-9z"
        }
    }
}

#[component]
pub fn GrStatusInfo() -> Element {
    rsx! {
        SvgIcon {
            path: "M2 3.99C2 2.892 2.898 2 3.99 2h16.02C21.108 2 22 2.898 22 3.99v16.02c0 1.099-.898 1.99-1.99 1.99H3.99A1.995 1.995 0 0 1 2 20.01V3.99zM12 10v8m0-12v2"
        }
    }
}
