use crate::components::TitleHeader;
use dioxus::prelude::*;

#[component]
pub fn ImportScreen() -> Element {
    rsx! {
        TitleHeader { sub_title: "Import" }
    }
}
