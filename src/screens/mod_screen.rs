use crate::{components::TitleHeader, AppState};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn ModScreen() -> Element {
    let mut state: Signal<AppState> = use_context::<Signal<AppState>>();

    rsx! {
        style { {include_str!("../css/mod_screen.css")} }
        TitleHeader { sub_title: "Mod List" }
        div {  // container grid
            class: "mod-grid",
            div {
                class: "mod-subgrid",
                div {  // mod list container
                    class: "mod-table",
                    div {
                        class: "header-row",
                        span { style: "margin-left:2.2%", input { "type": "checkbox" } }
                        p { style: "margin-left:4%", "Name" }
                        p { style: "margin-left:31%", "Version" }
                        p { style: "margin-left:6.1%", "Min. API Version" }
                        p { style: "margin-left:6.1%", "Enabled" }
                    }
                }
            }
            div {  // buttons container
                class: "button-subgrid",
                button { class: "button mod-action-button", "Enable" }
                button { class: "button mod-action-button", "Disable" }
                span { class: "divider-gap" }
                button { class: "button mod-action-button", "Find Updates" }
                button { class: "button mod-action-button", "Export Mods" }
            }
        }
    }
}
