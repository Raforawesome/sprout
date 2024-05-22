#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod components;
mod import_screen;
mod index_screen;
mod mod_screen;
use dioxus::{
    desktop::{
        tao::{platform::macos::WindowBuilderExtMacOS, window::Theme},
        Config, LogicalSize, WindowBuilder,
    },
    prelude::*,
};
use import_screen::ImportScreen;
use index_screen::IndexScreen;
/// # Sprout
/// A stardew valley mod manager.

#[derive(Routable, PartialEq, Clone)]
enum Routes {
    #[route("/")]
    IndexScreen {},
    #[route("/import")]
    ImportScreen {},
}

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
                        .with_movable_by_window_background(true)
                        // .with_titlebar_hidden(true)
                        .with_title_hidden(true)
                        .with_titlebar_transparent(true)
                        .with_inner_size(LogicalSize::new(1000, 685))
                        .with_resizable(false),
                ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../public/global.css")} }
        link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" }
        Router::<Routes> {}
    }
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
