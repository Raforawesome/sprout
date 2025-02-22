use crate::views::update_mods::UpdateScreen;
use crate::{components::TitleHeader, interface::mod_types::Mod, AppState};
use dioxus::desktop::{window, LogicalSize};
use dioxus::prelude::*;
use std::ops::DerefMut;

#[component]
pub fn ModRow(mut mod_ptr: *mut Mod, alt: bool) -> Element {
    let mod_obj: &mut Mod;
    unsafe {
        mod_obj = mod_ptr.as_mut().unwrap();
    }
    let enabled_signal: Signal<bool> = mod_obj.enabled_signal();
    rsx! {
        tr {
            class: if alt { "mod-row-alt" } else { "mod-row" },
            td {
                style: "width:7%",
                input {
                    "type": "checkbox",
                    class: "mod-checkbox",
                    checked: mod_obj.checked(),
                    onclick: move |_| mod_obj.set_checked(!mod_obj.checked())
                }
            }
            td {
                style: "width:44%",
                p { {mod_obj.name()} }
            }
            td {
                style: "width:17%",
                p { {mod_obj.version()} }
            }
            td {
                style: "width:17%",
                p { {mod_obj.min_api_version()} }
            }
            td {
                style: "width:10%",
                p {
                    class: if enabled_signal() { "enabled" } else { "disabled" },
                    {if enabled_signal() { "Enabled" } else { "Disabled" }}
                }
            }
        }
    }
}

#[component]
pub fn ModScreen() -> Element {
    let state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mut mod_signal: Signal<Vec<Mod>> =
        use_signal(|| crate::interface::mod_scanner::find_all_mods(state().game_path.as_path()));
    let mut all_checked: Signal<bool> = use_signal(|| false);

    let mut alt: bool = true;
    let mod_list = mod_signal.iter_mut().map(|mut m| {
        alt = !alt;
        rsx! {
            ModRow {
                mod_ptr: m.deref_mut(),
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
                                style: "display:flex;margin-left:auto;margin-right:auto",
                                disabled: false,
                                onclick: move |_| {
                                    all_checked.with_mut(|b| *b = !*b);
                                    mod_signal.with_mut(|mods| {
                                        mods.iter_mut().for_each(|m| m.set_checked(all_checked()));
                                    });
                                    document::eval(&format!(r#"
                                        document.querySelectorAll(".mod-checkbox").forEach(box => {{
                                            box.checked = {};
                                        }});
                                    "#, all_checked()));
                                }
                            }
                        }
                        p { style: "width:44%", "Name" }
                        p { style: "width:17%", "Version" }
                        p { style: "width:17%", "Min API Ver." }
                        p { style: "width:10%", "Status" }
                    }
                    {mod_list}
                }
            }
            div {  // buttons container
                class: "button-subgrid",
                button {
                    class: "button mod-action-button",
                    onclick: move |_| {
                        mod_signal.with_mut(|mods| {
                            mods.iter_mut()
                                .filter(|m| m.checked())
                                .for_each(|m| {
                                    m.set_enabled(true);
                                    let _ = m.enable();
                                });
                        });
                    },
                    "Enable"
                }
                button {
                    class: "button mod-action-button",
                    onclick: move |_| {
                        mod_signal.with_mut(|mods| {
                            mods.iter_mut()
                                .filter(|m| m.checked())
                                .for_each(|m| {
                                    m.set_enabled(false);
                                    let _ = m.disable();
                                });
                        });
                    },
                    "Disable"
                }
                span { class: "divider-gap" }
                button {
                    class: "button mod-action-button",
                    onclick: move |_| {
                        window().new_window(
                            VirtualDom::new(UpdateScreen),
                            dioxus::desktop::Config::new().with_disable_context_menu(true).with_window(
                                dioxus::desktop::WindowBuilder::new()
                                    // .with_title("Sprout Updater")
                                    .with_inner_size(LogicalSize::new(1000, 685))
                                    .with_resizable(false)
                                    .with_decorations(false))
                        );
                    },
                    "Find Updates"
                }
                button { class: "button mod-action-button", "Export Mods" }
            }
        }
    }
}
