use std::path::{Path, PathBuf};

use crate::components::TitleHeader;
use dioxus::prelude::*;
use rfd::FileDialog;

#[component]
pub fn ImportScreen() -> Element {
    let mut file_path = use_signal(PathBuf::new);
    let mut hide_error = use_signal(|| true);

    rsx! {
        TitleHeader { sub_title: "Import".to_string() }
        style { {include_str!("../css/import_screen.css")} }
        div {
            id: "import",
            class: "import",
            div {
                class: "display:inline-block;flex-direction:column;justify-content:left;",
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
        div {
            class: "button-container",
            p {
                class: "error-text",
                hidden: hide_error(),
                "This game path is invalid!"
            }
            button {
                id: "import-button",
                class: "button import-button",
                onclick: move |_| hide_error.set(validate_game_path(&file_path())),
                "Import"
            }
        }
    }
}

fn pick_folder() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Choose your Stardew Valley folder:")
        .pick_folder()
}

fn validate_game_path(p: &Path) -> bool {
    #[cfg(not(target_os = "macos"))]
    let mod_dir: PathBuf = p.join("Mods/");
    #[cfg(target_os = "macos")]
    let mod_dir: PathBuf = p.join("Contents/MacOS/Mods/");

    p.exists()
        && p.is_dir()
        && p.file_name().is_some_and(|s| s == "Stardew Valley")
        && mod_dir.exists()
        && mod_dir.is_dir()
}
