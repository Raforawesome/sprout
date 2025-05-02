use std::{
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use crate::{
    AppState, components::TitleHeader, launch_config, mod_scanner, mod_types::Mod,
    views::download_mods::DownloadScreen,
};
use dioxus::{desktop::use_window, prelude::*};
use zip::{ZipWriter, write::SimpleFileOptions};

#[component]
pub fn ModScreen() -> Element {
    let state: Signal<AppState> = use_context::<Signal<AppState>>();
    let mut mods: Signal<Vec<Mod>> = use_signal(move || mod_scanner::find_all_mods(state));
    let mods_dir: PathBuf = state().mods_path.clone();
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

                p { class: "text-xs text-base-content opacity-50 font-black", "CONTROLS" }
                div {
                    class: "flex flex-col gap-2",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| mods.with_mut(|v| v[i].enable()));
                            signal_smask.with_mut(|v| v.iter_mut().for_each(|b| *b = false));
                        },
                        "Enable"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| mods.with_mut(|v| v[i].disable()));
                            signal_smask.with_mut(|v| v.iter_mut().for_each(|b| *b = false));
                        },
                        "Disable"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| {
                                    mods.with_mut(|v| v.remove(i).delete()).expect("Permission to delete mod");
                                    signal_smask.with_mut(|v| v.remove(i));
                                });
                            signal_smask.with_mut(|v| v.iter_mut().for_each(|b| *b = false));
                        },
                        "Delete"
                    }
                }

                div {
                    class: "flex flex-col gap-2",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            use_window().new_window(VirtualDom::new(DownloadScreen), launch_config());
                        },
                        "Download Mods"
                    }
                    button { class: "btn btn-primary btn-disabled", "Check for Updates" }
                }

                div {
                    class: "flex flex-col gap-2",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let Some(file_path) = rfd::FileDialog::new().add_filter("zip", &["zip"]).pick_file() else { return; };
                            let file: Vec<u8> = std::fs::read(&file_path).expect("Read permissions");
                            let mut zip = zip::ZipArchive::new(Cursor::new(file)).unwrap();

                            zip.extract(&mods_dir).expect("Write permissions");
                        },
                        "Import Mods"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let Some(out_path) = rfd::FileDialog::new().add_filter("zip", &["zip"]).save_file() else { return; };
                            let mut out_file: File = File::create(&out_path).expect("Write permissions");
                            let mut zip_file: ZipWriter<BufWriter<&mut File>> = ZipWriter::new(BufWriter::new(&mut out_file));
                            let zip_opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

                            signal_smask().iter().enumerate()
                                .filter(|(_, m)| **m)
                                .for_each(|(i, _)| {
                                    let mod_list: Vec<Mod> = mods();  // load all mods
                                    let cur_mod: &Mod = &mod_list[i];  // get next selected mod
                                    let mod_folder = if cur_mod.enabled() { cur_mod.enabled_folder() } else { cur_mod.disabled_folder() };
                                    let mod_path: PathBuf = PathBuf::from(mod_folder.file_name().unwrap()); // create new base path
                                    zip_dir(&mut zip_file, mod_folder, &mod_path, zip_opts); // recursively zip folder
                                });
                            zip_file.finish().unwrap();
                        },
                        "Export Mods"
                    }
                }
            }
        }
    }
}

fn zip_dir(
    zip: &mut ZipWriter<BufWriter<&mut File>>,
    folder_path: &Path,
    path: &Path,
    opts: SimpleFileOptions,
) {
    for entry in std::fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let entry_path: PathBuf = entry.path();
        let entry_name = entry.file_name();

        let zip_path = path.join(&entry_name);
        if entry_path.is_file() {
            let file = std::fs::read(&entry_path).unwrap();
            zip.start_file_from_path(&zip_path, opts).unwrap();
            zip.write_all(&file).unwrap();
        } else {
            // is directory
            zip.add_directory_from_path(&zip_path, opts).unwrap();
            zip_dir(zip, &entry_path, &zip_path, opts);
        }
    }
}

#[component]
pub fn ModTable(mods: Signal<Vec<Mod>>, signal_smask: Signal<Vec<bool>>) -> Element {
    let mut db: bool = true;

    let mod_entries = mods.iter().enumerate().map(|(i, m)| {
        db = !db;
        rsx! {
            tr {
                class: if signal_smask()[i] {
                    "bg-primary"
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
                td { class: "text-base-content", "{m.name()}" }
                td { class: "font-semibold text-base-content", "{m.version()}" }
                td { class: "font-semibold text-base-content", "{m.min_api_version()}" }
                if mods()[i].enabled() {
                    td { class: "font-semibold text-success", " enabled" }
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
