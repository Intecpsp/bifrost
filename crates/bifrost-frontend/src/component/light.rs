use dioxus::prelude::*;
use uuid::Uuid;

use hue::api::{
    ColorTemperature, Dimming, Light, LightColor, LightGradient, LightGradientMode,
    LightGradientUpdate, LightUpdate, On,
};
use hue::xy::XY;
use z2m::hexcolor::HexColor;

use crate::HUE_CLIENT;
use crate::component::brightness::Brightness;
use crate::component::colortemp::ColorTemp;

#[component]
pub fn LightOnIcon(id: Uuid, on: On) -> Element {
    rsx! {
        input {
            type: "checkbox",
            class: "toggle",
            checked: on.on,
            onclick: move |evt| {
                evt.prevent_default();

                let upd = LightUpdate::new().with_on(Some(On { on: !on.on }));

                async move {
                    HUE_CLIENT.light_update(id, upd).await?;
                    Ok(())
                }
            },
        }
    }
}

#[component]
pub fn LightColorView(id: Uuid, dimming: Option<Dimming>, color: Option<LightColor>) -> Element {
    let (Some(dim), Some(color)) = (&dimming, &color) else {
        return rsx! {};
    };

    let hc: HexColor = color.xy.to_rgb(dim.brightness * 2.55).into();
    rsx! {
        div {
            class: "font-mono inline-flex h-10 items-center gap-5",
            input {
                type: "color",
                value: "{hc}",
                onchange: move |evt| {
                    evt.prevent_default();

                    let col = HexColor::try_from(evt.value().as_str()).unwrap();
                    tracing::info!("col: {col}");

                    let (xy, bri) = XY::from_rgb(col.r, col.g, col.b);
                    tracing::info!("xy/b: {xy:.4?}/{bri:.2}");

                    let upd = LightUpdate::new()
                        .with_color_xy(Some(xy))
                        .with_brightness(Some(bri / 2.55));

                    async move {
                        HUE_CLIENT.light_update(id, upd).await?;
                        Ok(())
                    }
                }
            }
            span { "{hc}" }
        }
        /* div { */
        /*     class: "spectrum", */
        /*     class: "ml-4 w-20 h-20 rounded-full", */
        /* } */

    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[component]
pub fn LightDimming(id: Uuid, dimming: Option<Dimming>) -> Element {
    let Some(dim) = &dimming else {
        return rsx! {};
    };

    rsx! {
        Brightness {
            onchange: move |evt: FormEvent| {
                evt.prevent_default();
                evt.stop_propagation();
                let bri: f64 = evt.value().parse().unwrap();
                async move {
                    HUE_CLIENT.light_update(id, LightUpdate::new().with_brightness(Some(bri))).await?;
                    Ok(())
                }
            },
            value: dim.brightness as u32,
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn LightColorTemperature(id: Uuid, color_temperature: Option<ColorTemperature>) -> Element {
    let Some(ct) = &color_temperature else {
        return rsx! {};
    };

    rsx! {
        ColorTemp {
            range: ct.mirek_schema,
            onchange: move |evt: FormEvent| {
                evt.prevent_default();

                let ct: u16 = evt.value().parse().unwrap();
                async move {
                    let upd = LightUpdate::new()
                        .with_color_temperature(Some(ct));

                    HUE_CLIENT.light_update(id, upd).await?;
                    Ok(())
                }
            },
            value: ct.mirek.map(u32::from),
        }
    }
}

#[component]
pub fn LightGradientModeButton(
    id: Uuid,
    mode: LightGradientMode,
    gradient: LightGradient,
    children: Element,
) -> Element {
    rsx! {
        input {
            class: "btn join-item",
            name: "{id}-grad-mode",
            type: "radio",
            checked: gradient.mode == mode,
            onchange: move |_| {
                let points = gradient.points.clone();
                let gradient_update = LightGradientUpdate {
                    mode: Some(mode),
                    points,
                };
                async move {
                    let upd = LightUpdate::new()
                        .with_gradient(Some(gradient_update));

                    HUE_CLIENT.light_update(id, upd).await?;
                    Ok(())
                }
            },
            {children}
        }
    }
}

#[component]
pub fn LightGradientModeView(id: Uuid, gradient: LightGradient) -> Element {
    rsx! {
        div {
            class: "join",
            LightGradientModeButton {
                id,
                mode: LightGradientMode::InterpolatedPalette,
                gradient: gradient.clone(),
                "Linear"
            }
            LightGradientModeButton {
                id,
                mode: LightGradientMode::InterpolatedPaletteMirrored,
                gradient: gradient.clone(),
                "Mirrored"
            }
            LightGradientModeButton {
                id,
                mode: LightGradientMode::RandomPixelated,
                gradient: gradient,
                "Scattered"
            }
        }
    }
}

#[component]
pub fn LightGradientColorView(id: Uuid, gradient: LightGradient) -> Element {
    rsx! {
        div {
            class: "join",
            class: "items-center",
            for point in gradient.points {
                div {
                    class: "w-8 h-8 border-2",
                    class: "join-item",
                    style: "background-color: {HexColor::from_xy_color(point.color.xy, 255.0)}",
                }
            }
        }
    }
}

#[component]
pub fn LightGradientView(id: Uuid, gradient: Option<LightGradient>) -> Element {
    let Some(gradient) = gradient else {
        return rsx! {};
    };

    rsx! {
        div {
            class: "flex flex-row items-center justify-between gap-4",
            LightGradientModeView { id, gradient: gradient.clone() }
            LightGradientColorView { id, gradient }
        }
    }
}

#[component]
pub fn LightView(id: Uuid, light: Light) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4",
            div {
                class: "flex flex-row items-center gap-4 justify-between",
                LightOnIcon { id, on: light.on }
                LightColorView { id, dimming: light.dimming, color: light.color }
                LightDimming { id, dimming: light.dimming }
                LightColorTemperature { id, color_temperature: light.color_temperature }
            }
            LightGradientView { id, gradient: light.gradient }
        }
    }
}
