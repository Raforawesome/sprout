use dioxus::desktop::window;
use dioxus::prelude::*;
use sprout::LOGO_1;

#[component]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        style { {include_str!("css/title_header.css")} }
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
            // img { class: "sprout-icon", src: "/sprout@1x.png" },
            img { class: "sprout-icon", src: "{LOGO_1}" },
            h1 { class: "title", "Sprout" },
            h1 { class: "subtitle", "|" },
            h1 { class: "subtitle", "{sub_title}" }
        }

        hr { class: "divider" }
    }
}
