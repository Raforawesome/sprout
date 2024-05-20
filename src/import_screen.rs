use std::path::PathBuf;

use crate::components::TitleHeader;
use dioxus::prelude::*;
use rfd::FileDialog;

#[component]
pub fn ImportScreen() -> Element {
    let mut file_path = use_signal(PathBuf::new);

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
                    class: "path-box",
                    value: file_path().as_os_str().to_str().unwrap(),
                    onchange: move |new| {
                        file_path.set(PathBuf::from(new.value()))
                    }
                }
            }
            span {
                class: "material-symbols-outlined button picker",
                onclick: move |_| {
                    if let Some(path) = pick_folder() {
                        file_path.set(path);
                    }
                },
                "folder"
            }
        }
        button {
            id: "import-button",
            class: "button import-button",
            onclick: move |_| {
            },
            "Import"
        }
    }
}

fn pick_folder() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Choose your Stardew Valley folder:")
        .pick_folder()
}
