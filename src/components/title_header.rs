use std::env::consts::OS;

use dioxus::desktop::window;
use dioxus::document::Link;
use dioxus::prelude::*;
use document::eval;

use crate::THEME;

#[component]
// #[cfg(not(target_os = "macos"))]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("src/components/css/title_header.css") }
        if std::env::consts::OS != "macos" {
            div {
                class: "title-buttons",
                button {
                    class: "material-symbols-outlined button title-button",
                    style: "color:#ff9f43",
                    onclick: |_| window().set_minimized(true),
                    "close_fullscreen"
                },
                button {
                    class: "material-symbols-outlined button title-button",
                    style: "color:#ee5253",
                    onclick: |_| window().close(),
                    "cancel"
                }
            }
        }
        div {
            onmousedown: |_| window().drag(),
            id: "title-header",
            class: "w-full flex flex-row items-center justify-center gap-4 p-4",

            button {
                class: "mx-2",
                onclick: prompt_theme_change,
                img { class: "sprout-icon h-15", src: asset!("public/sprout@1x-opt.webp") },
            }
            dialog {
                id: "theme_modal",
                class: "modal",

                div {
                    class: "modal-box w-1/3 h-1/2 bg-base-200 flex flex-col items-center",
                    // box
                    p { class: "text-base-content font-semibold text-lg", "Theme Selector" }
                    p { class: "text-base-content text-md", "Current theme: {THEME}" }
                    details {
                        class: "dropdown",
                        summary {
                            class: "btn btn-neutral mt-8 btn-wide",
                            "Show Themes"
                        }
                        ul {
                            class: "p-2 shadow menu dropdown-content rounded-box bg-base-100 w-52",
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "forest"), "forest" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "abyss"), "abyss" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "cyberpunk"), "cyberpunk" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "coffee"), "coffee" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "luxury"), "luxury" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "business"), "business" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "black"), "black" } }
                        }
                    }
                }

                form {
                    method: "dialog",
                    class: "modal-backdrop",
                    button { "close" }
                }
            }

            h1 { class: "text-5xl font-semibold text-base-content", "Sprout" },
            h1 { class: "text-5xl font-normal text-base-content", "|" },
            h1 { class: "text-5xl font-light text-base-content", "{sub_title}" }
        }

        hr { class: "h-px border-accent my-1 opacity-50 drop-shadow-sm" }
    }
}

fn prompt_theme_change(_: Event<MouseData>) {
    eval("document.getElementById('theme_modal').showModal()");
}
