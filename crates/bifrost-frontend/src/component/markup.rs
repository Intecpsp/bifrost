use dioxus::prelude::*;

#[component]
pub fn Section(children: Element) -> Element {
    rsx! {
        div {
            class: "divider divider-primary",
            {children}
        }
    }
}

#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        h2 {
            class: "py-2",
            class: "text-xl",
            class: "font-bold",
            "> " {children}
        }
    }
}

#[component]
pub fn UrlSpan(children: Element) -> Element {
    rsx! {
        span {
            class: "font-bold",
            class: "font-mono",
            class: "bg-sky-500",
            class: "rounded-md",
            class: "px-1",
            class: "py-1",
            {children}
        }
    }
}

#[component]
pub fn Filename(children: Element) -> Element {
    rsx! {
        span {
            class: "font-bold",
            class: "font-mono",
            class: "bg-green-400",
            class: "text-stone-700",
            class: "rounded-md",
            class: "px-1",
            class: "py-1",
            {children}
        }
    }
}

#[component]
pub fn IpEdit() -> Element {
    let mut valid = use_signal(|| false);
    let mut value = use_signal(String::new);

    rsx! {
        input {
            type: "text",
            class: "border-2",
            class: if !valid() { "border-red-500" } else { "border-grey-500" },
            value: "{value}",
            oninput: move |event| {
                let val = event.value();
                valid.set(val == "foo");
                value.set(val);
            }
        }
    }
}

#[component]
pub fn OptionValue(name: String, value: Option<String>) -> Element {
    rsx! {
        if let Some(value) = value {
            p { "{name}: {value}" }
        } else {
            p { "{name}: ❌" }
        }
    }
}
