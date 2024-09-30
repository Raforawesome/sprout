use crate::{components::TitleHeader, interface::mod_types::Mod, AppState};
use dioxus::prelude::*;

#[component]
pub fn ModRow(mod_obj: Mod, alt: bool) -> Element {
    rsx! {
        tr {
            class: if alt { "mod-row-alt" } else { "mod-row" },
            td {
                style: "width:7%",
                input {
                    "type": "checkbox",
                }
            }
            td {
                style: "width:40%",
                p { {mod_obj.name()} }
            }
            td {
                style: "width:21%",
                p { {mod_obj.version()} }
            }
            td {
                style: "width:18%",
                p { {mod_obj.min_api_version()} }
            }
            td {
                style: "width:10%",
                p { {mod_obj.enabled().to_string()} }
            }
        }
    }
}

#[component]
pub fn ModScreen() -> Element {
    let state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mods: Vec<Mod> = crate::interface::mod_scanner::find_mods(state().game_path.as_path());

    let mut alt: bool = true;
    let mod_list = mods.iter().map(|m| {
        alt = !alt;
        rsx! {
            ModRow {
                mod_obj: m.clone(),
                alt,
            }
        }
    });

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
                        span {
                            style: "width:7%",
                            input {
                                "type": "checkbox",
                                style: "display:flex;margin-left:auto;margin-right:auto"
                            }
                        }
                        p { style: "width:40%", "Name" }
                        p { style: "width:21%", "Version" }
                        p { style: "width:18%", "Min API Ver." }
                        p { style: "width:10%", "Enabled" }
                    }
                    {mod_list}
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
