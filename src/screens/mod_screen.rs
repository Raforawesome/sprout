use crate::components::TitleHeader;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn ModScreen(path: PathBuf) -> Element {
    rsx! {
        TitleHeader { sub_title: "Mod List" }
    }
}
