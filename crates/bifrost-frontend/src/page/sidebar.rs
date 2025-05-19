use dioxus::prelude::*;

use hue::stream::HueStreamLightsV2;
use z2m::hexcolor::HexColor;

use crate::grommet_icons::{
    GrAction, GrInfo, GrIteration, GrServices, GrStatusInfo, GrStorage, GrSystem,
};
use crate::page::footer::Footer;
use crate::{LOGO_SVG, Route, use_context_signal};

#[component]
pub fn Sidebar() -> Element {
    let ent = use_context_signal::<Option<HueStreamLightsV2>>();

    rsx! {
        div {
            class: "lg:w-80",
            div {
                class: "lg:inset-0 sticky lg:min-h-svh lg:min-w-70 flex flex-col bg-base-300",
                nav {
                    class: "flex",
                    img {
                        src: LOGO_SVG,
                        class: "w-16",
                        class: "m-4",
                    }
                    h1 {
                        class: "text-5xl",
                        class: "font-bold",
                        class: "self-center",
                        "Bifrost"
                    }
                }

                ul {
                    class: "menu text-lg font-bold grow w-full *:gap-0",

                    li { Link { to: Route::Index,                         "Main page"  } }
                    li { Link { to: Route::LightsView,    GrInfo {}       "Lights"     } }
                    li { Link { to: Route::GroupsView,    GrIteration {}  "Groups"     } }
                    li { Link { to: Route::ResourcesView, GrStorage {}    "Resources"  } }
                    li { Link { to: Route::ServicesView,  GrServices {}   "Services"   } }
                    li { Link { to: Route::Backends,      GrAction {}     "Backends"   } }
                    li { Link { to: Route::Config,        GrSystem {}     "Config"     } }
                    li { Link { to: Route::About,         GrStatusInfo {} "About page" } }
                }

                if let Some(ent) = &*ent.read() {
                    match ent {
                        HueStreamLightsV2::Xy(xyv) => {
                            rsx! {
                                ul {
                                    for value in xyv {
                                        li { "{value:?}" }
                                    }
                                }
                            }
                        }
                        HueStreamLightsV2::Rgb(rgbs) => {
                            rsx! {
                                ul {
                                    class: "flex flex-row justify-stretch h-8 *:grow",
                                    for value in rgbs {
                                        {
                                            let rgb = HexColor::new((value.rgb.r / 256) as u8, (value.rgb.g / 256) as u8, (value.rgb.b / 256) as u8);
                                            rsx! {
                                                li { style: "background-color: {rgb}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Footer {}
            }
        }
    }
}
