use crate::libsprout::path_manager;
use crate::{AppState, components::TitleHeader};
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use rfd::FileDialog;
use std::path::{Path, PathBuf};

#[component]
pub fn ImportScreen() -> Element {
    let mut state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mut picker_clr: Signal<&str> = use_signal(|| "neutral");

    rsx! {
        TitleHeader { sub_title: "Import".to_string() }
        div {  // content frame for the rest of the page
            class: "bg-base-100 flex-grow",
            div { // container for input elements
                class: "flex items-center justify-center h-full",
                fieldset { // input set
                    class: "w-md fieldset -mt-20",
                    legend { class: "w-md text-base fieldset-legend", "Find game directory" }
                    input { type: "file",
                        directory: true,
                        class: "file-input file-input-{picker_clr}" ,
                        onchange: |evt| {
                            let _ = dbg!(evt.files().unwrap().files());
                        }
                    }
                    label { class: "fieldset-label", "Make sure you don't select the 'Mods/' directory." }
                }
            }
        }
    }
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
