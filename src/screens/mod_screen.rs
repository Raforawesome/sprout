use crate::{components::TitleHeader, AppState};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn ModScreen() -> Element {
    let mut state: Signal<AppState> = use_context::<Signal<AppState>>();

    rsx! {
        style { {include_str!("../css/mod_screen.css")} }
        TitleHeader { sub_title: "Mod List" }
        div {
            class: "mod-list",
            div {
                class: "header-row",
                span { style: "margin-left:1.06rem", input { "type": "checkbox" } }
                p { style: "margin-left:1.94rem", "Name" }
                p { style: "margin-left:12.69rem", "Version" }
                p { style: "margin-left:4.31rem", "Min. Game Version" }
                p { style: "margin-left:4.31rem;margin-right:2rem", "Enabled" }
            }
        }
    }
}
