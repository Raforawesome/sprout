#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod import_screen;
mod index_screen;
use dioxus::{desktop::Config, prelude::*};
use index_screen::IndexScreen;
/// # Sprout
/// A stardew valley mod manager.

#[derive(Routable, PartialEq, Clone)]
enum Routes {
    #[route("/")]
    IndexScreen {},
}

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((39, 174, 96, 1))
                .with_data_directory(get_data_dir()) // set to $XDG_CONFIG_HOME/.svsprout
                // .with_resource_directory("./assets/")
                .with_disable_context_menu(true), // disable right click menu
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Routes> {} }
}

pub fn get_data_dir() -> std::path::PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| {
        eprintln!("Unsupported operating system.");
        std::process::exit(1);
    });
    path.push(".svsprout/");
    if !path.exists() {
        std::fs::create_dir(&path).unwrap_or_else(|_| {
            eprintln!("No permission to write to data directory.");
            std::process::exit(1);
        });
    };
    path
}
