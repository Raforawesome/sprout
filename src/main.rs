#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
//! <h1>Sprout</h1>
//! <h5>A Stardew Valley Mod Manager</h5>
//! <hr>
//! Sprout is a simple SMAPI mod manager for Stardew Valley.
//! Currently caching is not implemented, and it works on a simple
//! no-DB file-scan system.
mod components;
mod screens;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

use dioxus::{
    desktop::{tao::window::Theme, Config, LogicalSize, WindowBuilder},
    prelude::*,
};
use import_screen::ImportScreen;
use index_screen::IndexScreen;
use mod_screen::ModScreen;
use screens::{import_screen, index_screen, mod_screen};

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
}

#[derive(Routable, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
enum Routes {
    #[route("/")]
    IndexScreen {},
    #[route("/import")]
    ImportScreen {},
    #[route("/mods")]
    ModScreen {},
}

#[component]
fn App() -> Element {
    let _state: Signal<AppState> = use_context_provider(|| Signal::new(AppState::default()));

    rsx! {
        style { {include_str!("../public/global.css")} }
        link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" }
        Router::<Routes> {}
    }
}

#[cfg(target_os = "macos")]
fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((34, 47, 62, 1))
                .with_data_directory(get_data_dir().join("data/"))
                .with_resource_directory(get_data_dir().join("assets/"))
                .with_disable_context_menu(true)
                .with_window(
                    WindowBuilder::new()
                        .with_theme(Some(Theme::Dark))
                        .with_title("Sprout")
                        .with_fullsize_content_view(true)
                        .with_title_hidden(true)
                        .with_titlebar_transparent(true)
                        .with_inner_size(LogicalSize::new(1000, 685))
                        .with_resizable(false),
                ),
        )
        .launch(App);
}

#[cfg(target_os = "windows")]
fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((34, 47, 62, 1))
                .with_data_directory(get_data_dir().join("data/"))
                .with_resource_directory(get_data_dir().join("assets/"))
                .with_disable_context_menu(true)
                .with_window(
                    WindowBuilder::new()
                        .with_theme(Some(Theme::Dark))
                        .with_title("Sprout")
                        .with_inner_size(LogicalSize::new(1000, 685))
                        .with_resizable(false)
                        .with_decorations(false),
                ),
        )
        .launch(App);
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
