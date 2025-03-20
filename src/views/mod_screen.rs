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
                class: "w-3/4 h-full items-center justify-center p-5",  // set width to 70%
                ModTable { mods: mods }
            }
            div { // right div: buttons
                class: "w-1/4 flex flex-col p-5 gap-4",

                p { class: "text-xs text-base-content opacity-25 font-black", "CONTROLS" }
                div {
                    class: "flex flex-col gap-2",
                    button { class: "btn btn-neutral", "Enable" }
                    button { class: "btn btn-neutral", "Disable" }
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
pub fn ModTable(mods: Vec<Mod>) -> Element {
    let signal_smask: Vec<Signal<bool>> = vec![Signal::new(false); mods.len()];

    let mod_entries = mods.iter().enumerate().map(|(i, m)| {
        rsx! {
            tr {
                th { class: "w-1 text-xs text-secondary", "{i + 1}" } // line number
                td { "{m.name()}" }
                td { class: "font-semibold", "{m.version()}" }
                td { class: "font-semibold", "{m.min_api_version()}" }
                if signal_smask[i]() {
                    td { class: "text-success", "enabled" }
                } else {
                    td { class: "text-error", "disabled" }
                }
            }
        }
    });

    rsx! {
        div {
            class: "overflow-auto h-full rounded-box border bg-base-200",
            table {
                class: "table table-zebra",

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
