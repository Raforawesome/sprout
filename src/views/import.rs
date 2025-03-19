use crate::{AppState, components::TitleHeader};
use dioxus::prelude::*;
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

                    legend { class: "w-md text-base fieldset-legend", "Select game directory:" }

                    input { type: "file",
                        directory: true,
                        class: "w-md text-base file-input file-input-{picker_clr}" ,
                        onchange: move |evt| {
                            if let Some(files) = evt.files() {
                                let files = files.files();
                                // this branch can't be reached unless files has exactly 1 entry
                                let game_dir: &str = unsafe { files.get_unchecked(0) };

                                if validate_game_path(Path::new(game_dir)) {
                                    picker_clr.set("success");
                                    state.with_mut(|s| s.game_path = PathBuf::from(game_dir));
                                    state.with_mut(|s| s.mods_path = mods_path(&s.game_path));
                                    navigator().replace("/mods");
                                } else {
                                    picker_clr.set("error");
                                }
                            }
                        }
                    }

                    label {
                        class: "w-md text-base fieldset-label",

                        { if picker_clr() == "error" {
                            "Invalid directory (couldn't find mods)!"
                        } else {
                            "Ensure you don't select the 'Mods/' directory."
                        }}
                    }
                }
            }
        }
    }
}

#[inline]
#[cfg(target_os = "macos")]
fn mods_path(p: &Path) -> PathBuf {
    p.join("Contents/MacOS/Mods/")
}

#[inline]
#[cfg(not(target_os = "macos"))]
fn mods_path(p: &Path) -> PathBuf {
    p.join("Mods/")
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
