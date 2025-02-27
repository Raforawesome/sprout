use dioxus::prelude::*;
use dioxus::document::Link;
use crate::components::TitleHeader;

/// This component is spawned in a different window, so we treat this as a new root
/// (include global.css again)
#[component]
pub fn UpdateScreen() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("public/global.css") }
        TitleHeader { sub_title: "Update" }
    }
}
