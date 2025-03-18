use crate::components::TitleHeader;
use dioxus::document::Link;
use dioxus::prelude::*;

/// This component is spawned in a different window, so we treat this as a new root
/// (include global.css again)
#[component]
pub fn UpdateScreen() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("public/global.css") }
        style { {include_str!("../css/mod_screen.css")} }
        TitleHeader { sub_title: "Update" }

        // Mod table
        div { // main grid
            class: "mod-subgrid",
        }
    }
}
