use dioxus::desktop::window;
use dioxus::document::Link;
use dioxus::prelude::*;
use document::eval;

use crate::THEME;

#[component]
// #[cfg(not(target_os = "macos"))]
pub fn TitleHeader(sub_title: String) -> Element {
    let mut use_hr: Signal<bool> = use_signal(|| false);

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
                    p { class: "text-base-content font-semibold text-lg", "Sprout Settings" }
                    p { class: "text-base-content text-md", "Current theme: {THEME}" }

                    div {
                        class: "flex flex-row gap-3 my-4",
                        input {
                            type: "checkbox",
                            checked: "{use_hr}",
                            class: "checkbox",
                            oninput: move |_| use_hr.set(!use_hr()),
                        }
                        p { class: "text-md", "Draw header rule" }
                    }
                    details {
                        class: "dropdown",
                        summary {
                            class: "btn btn-neutral mt-4 btn-wide",
                            "Show Themes"
                        }
                        ul {
                            class: "p-2 shadow menu dropdown-content rounded-box bg-base-100 w-52",
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "light"), "light" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "dark"), "dark" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "forest"), "forest" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "synthwave"), "synthwave" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "abyss"), "abyss" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "night"), "night" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "coffee"), "coffee" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "luxury"), "luxury" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "dracula"), "dracula" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "sunset"), "sunset" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "business"), "business" } }
                            li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "black"), "black" } }
                            // li { a { href: "#", onclick: move |_| THEME.with_mut(|s| *s = "cyberpunk"), "cyberpunk" } }
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

        if use_hr() {
            hr { class: "h-px border-accent my-1 opacity-50 drop-shadow-sm" }
        }
    }
}

fn prompt_theme_change(_: Event<MouseData>) {
    eval("document.getElementById('theme_modal').showModal()");
}
