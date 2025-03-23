use crate::{AppState, components::TitleHeader, mod_scanner, mod_types::Mod};
use dioxus::prelude::*;

#[component]
pub fn ModScreen() -> Element {
    let state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mut mods: Signal<Vec<Mod>> = use_signal(move || mod_scanner::find_all_mods(state));
    // select mask made up of signals
    let mut signal_smask: Signal<Vec<bool>> = use_signal(|| vec![false; mods.len()]);

    rsx! {
        TitleHeader { sub_title: "Mod List" }
        div {
            class: "overflow-y-auto flex flex-row flex-grow",
            // split containing div into two "sub-divs"
            div { // left div: mods table
                class: "w-3/4 h-full items-center justify-center p-5",  // set width to 70%
                ModTable { mods, signal_smask }
            }
            div { // right div: buttons
                class: "w-1/4 flex flex-col p-5 gap-4",

                p { class: "text-xs text-base-content opacity-25 font-black", "CONTROLS" }
                div {
                    class: "flex flex-col gap-2",
                    button {
                        class: "btn btn-neutral",
                        onclick: move |_| {
                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| mods.with_mut(|v| v[i].set_enabled(true)));
                            signal_smask.with_mut(|v| v.iter_mut().for_each(|b| *b = false));
                        },
                        "Enable"
                    }
                    button {
                        class: "btn btn-neutral",
                        onclick: move |_| {
                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| mods.with_mut(|v| v[i].set_enabled(false)));
                            signal_smask.with_mut(|v| v.iter_mut().for_each(|b| *b = false));
                        },
                        "Disable"
                    }
                }

                div {
                    class: "flex flex-col gap-2",
                    button { class: "btn btn-neutral", "Check for Updates" }
                    button { class: "btn btn-neutral", "Export Mods" }
                }
            }
        }
    }
}

#[component]
pub fn ModTable(mods: Signal<Vec<Mod>>, signal_smask: Signal<Vec<bool>>) -> Element {
    // let signal_smask: Vec<Signal<bool>> = vec![use_signal(|| false); mods.len()];
    let mut db: bool = true;

    let mod_entries = mods.iter().enumerate().map(|(i, m)| {
        db = !db;
        rsx! {
            tr {
                class: if signal_smask()[i] {
                    "bg-neutral"
                } else {
                    if db {
                        "bg-base-200 hover:bg-base-100 hover:bg-opacity-10"
                    } else {
                        "hover:bg-base-100 hover:bg-opacity-10"
                    }
                }, // hover effects
                onclick: move |_| {
                    signal_smask.with_mut(|v| v[i] = !v[i]);
                },

                th { class: "w-1 text-xs text-secondary", "{i + 1}" } // line number
                td { "{m.name()}" }
                td { class: "font-semibold", "{m.version()}" }
                td { class: "font-semibold", "{m.min_api_version()}" }
                if m.enabled() {
                    td { class: "font-semibold text-success", "enabled" }
                } else {
                    td { class: "font-semibold text-error", "disabled" }
                }
            }
        }
    });

    rsx! {
        div {
            class: "overflow-auto h-full rounded-box border bg-base-300",
            table {
                class: "table",

                thead {  // table head
                    tr { // header row
                        class: "sticky top-0 bg-base-300",

                        th {} // skip one to allow for line numbers
                        th { "Name" }
                        th { "Version" }
                        th { "Min API. Ver" }
                        th { "Status" }
                    }
                }

                tbody {  // body of table, will store mod entries
                    {mod_entries}
                }
            }
        }
    }
}
