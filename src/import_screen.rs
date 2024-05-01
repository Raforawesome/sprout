use crate::components::TitleHeader;
use dioxus::prelude::*;

#[component]
pub fn ImportScreen() -> Element {
    rsx! {
        TitleHeader { sub_title: "Import" }
        style { {include_str!("./css/import_screen.css")} }
        div {
            id: "import",
            class: "import",
            div {
                class: "display:flex;flex-direction:column;justify-content:left;",
                p { class: "label", "Game location:" }
                input {
                    id: "class-box",
                    class: "path-box"
                }
            }
            span { class: "material-symbols-outlined picker", "folder" }
        }
        button {
            id: "import-button",
            class: "import-button",
            "Import"
        }
    }
}
