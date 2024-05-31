use dioxus::prelude::*;
use dioxus::desktop::window;

#[component]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        div {
            class: "title-buttons",
            button {
                class: "button title-button",
                onclick: |_| window().set_minimized(true),
                "–"
            },
            button {
                class: "button title-button",
                onclick: |_| window().close(),
                "x"
            }
        }
        style { {include_str!("css/title_header.css")} }
        div {
            onmousedown: |_| window().drag(),
            id: "title-header",
            class: "title-header",
            img { class: "icon", src: "/sprout@1x.png" },
            h1 { class: "title", "Sprout" },
            h1 { class: "subtitle", "|" },
            h1 { class: "subtitle", "{sub_title}" }
        }

        hr { class: "divider" }
    }
}
