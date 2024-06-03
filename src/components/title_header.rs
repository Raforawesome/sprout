use dioxus::desktop::window;
use dioxus::prelude::*;

#[component]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        style { {include_str!("css/title_header.css")} }
        div {
            class: "title-buttons",
            button {
                class: "material-symbols-outlined button title-button",
                onclick: |_| window().set_minimized(true),
                "close_fullscreen"
            },
            button {
                class: "material-symbols-outlined button title-button",
                onclick: |_| window().close(),
                "cancel"
            }
        }
        div {
            onmousedown: |_| window().drag(),
            id: "title-header",
            class: "title-header",
            img { class: "sprout-icon", src: "/sprout@1x.png" },
            h1 { class: "title", "Sprout" },
            h1 { class: "subtitle", "|" },
            h1 { class: "subtitle", "{sub_title}" }
        }

        hr { class: "divider" }
    }
}
