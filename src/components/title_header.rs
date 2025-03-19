use dioxus::desktop::window;
use dioxus::document::Link;
use dioxus::prelude::*;

#[component]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("src/components/css/title_header.css") }
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
        div {
            onmousedown: |_| window().drag(),
            id: "title-header",
            class: "title-header",
            // img { class: "sprout-icon", src: asset!("public/sprout@1x-opt.webp") },
            h1 { class: "text-5xl font-bold text-base-content", "Sprout" },
            h1 { class: "text-5xl font-normal text-base-content", "|" },
            h1 { class: "text-5xl font-light text-base-content", "{sub_title}" }
        }

        // hr { class: "divider" }
    }
}
