use crate::{AppState, components::TitleHeader, mod_scanner, mod_types::Mod};
use dioxus::prelude::*;

#[component]
pub fn ModScreen() -> Element {
    let state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mods: Vec<Mod> = mod_scanner::find_all_mods(state);

    rsx! {
        TitleHeader { sub_title: "Mod List" }
        div {
            class: "overflow-y-auto flex flex-row flex-grow",
            // split containing div into two "sub-divs"
            div { // left div: mods table
                class: "w-7/10 h-full items-center justify-center p-5",  // set width to 70%
                ModTable { mods: mods }
            }
            div { // right div: buttons
                class: "w-3/10",
            }
        }
    }
}

#[component]
pub fn ModTable(mods: Vec<Mod>) -> Element {
    let signal_smask: Vec<Signal<bool>> = vec![Signal::new(false); mods.len()];

    let mod_entries = mods.iter().enumerate().map(|(i, m)| {
        rsx! {
            tr {
                th { "{i}" } // line number
                td { "{m.name()}" }
                td { "{m.version()" }
                td { "{m.min_api_version()}" }
                td { if signal_smask[i]() { "enabled" } else { "disabled" } }
            }
        }
    });

    rsx! {
        div {
            class: "w-auto h-auto overflow-x-hidden overflow-y-auto rounded-box border border-base-content/5 bg-base-100",
            table {
                class: "table overflow-y-auto",

                thead {  // table head
                    tr { // header row
                        // class: "sticky",

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
