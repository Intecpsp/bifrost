use std::collections::BTreeMap;

use dioxus::prelude::*;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::{Serializer, Value};
use uuid::Uuid;

use hue::api::ResourceRecord;

use crate::use_context_signal;

#[component]
pub fn Collapse(header: Element, content: Element) -> Element {
    rsx! {
        div {
            class: "collapse collapse-arrow peer",
            input {
                type: "checkbox",
                class: "peer",
            }
            div {
                class: "collapse-title font-semibold",
                { header }
            }
            div {
                class: "collapse-content text-sm",
                { content }
            }
        }
    }
}

fn pretty_json(value: &Value) -> Result<String, serde_json::Error> {
    let mut writer = Vec::with_capacity(128);
    let mut ser = Serializer::with_formatter(&mut writer, PrettyFormatter::with_indent(b"    "));
    value.serialize(&mut ser)?;

    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    };
    Ok(string)
}

#[component]
pub fn Highlight(value: Value) -> Element {
    use syntect::html::{ClassStyle, ClassedHTMLGenerator};
    use syntect::parsing::SyntaxSet;
    use syntect::util::LinesWithEndings;

    let current_code = pretty_json(&value)?;

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_name("JSON").unwrap();
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&current_code) {
        html_generator.parse_html_for_line_which_includes_newline(line)?;
    }
    let output_html = html_generator.finalize();

    rsx! {
        div {
            dangerous_inner_html: output_html,
        }
    }
}

#[allow(clippy::option_if_let_else)]
#[component]
pub fn ResourcesView() -> Element {
    let res = use_context_signal::<BTreeMap<Uuid, ResourceRecord>>();

    rsx! {
        h2 { class: "card-title", "Resources" }

        div {
            div {
                class: "*:odd:bg-base-200",
                for (uuid, item) in &*res.read() {
                    div {
                        key: "{uuid}",
                        div {
                            Collapse {
                                header: rsx! {
                                    div {
                                        class: "flex flex-col lg:flex-row gap-4",
                                        span { class: "badge font-mono text-nowrap px-0",  "{uuid}" }
                                        span {
                                            class: "grow",
                                            {
                                                if let Some(item) = &item.id_v1 {
                                                    rsx! {
                                                        div {
                                                            class: "badge badge-primary text-nowrap font-mono",
                                                            "{item}"
                                                        }
                                                    }
                                                } else {
                                                    rsx! {
                                                        "-"
                                                    }
                                                }
                                            }
                                        }
                                        span {
                                            "{item.obj.rtype():?}"
                                        }
                                    }

                                },
                                content: rsx! {
                                    span {
                                        class: "font-mono",
                                        class: "whitespace-pre",
                                        Highlight {
                                            value: serde_json::to_value(&item.obj).unwrap(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
