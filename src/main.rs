#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
//! <h1>Sprout</h1>
//! <h5>A Stardew Valley Mod Manager</h5>
//! <hr>
//! Sprout is a simple SMAPI mod manager for Stardew Valley.
//! Currently caching is not implemented, and it works on a simple
//! no-DB file-scan system.

#[cfg(target_os = "macos")]
use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

use dioxus::{
    desktop::{tao::window::Theme, Config, LogicalSize, WindowBuilder},
    document::Link,
    prelude::*,
};
use import::ImportScreen;
use index::IndexScreen;
use mod_screen::ModScreen;
use sprout::views::{import, index, mod_screen};
use sprout::AppState;

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
        Link { rel: "stylesheet", href: asset!("public/global.css") }
        Router::<Routes> {}
    }
}

fn main() {
    // Force mod listings to be fetched on another thread as they take
    // time to parse, and shouldn't be generated on-the-fly when required.
    let _ = std::thread::spawn(|| {
        let _: std::sync::Arc<Vec<_>> = sprout::smapi::fetcher::MOD_LISTINGS.clone();
    });
    launch();
}

#[cfg(target_os = "macos")]
fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((34, 47, 62, 1))
                .with_disable_context_menu(true)
                .with_window(
                    WindowBuilder::new()
                        .with_theme(Some(Theme::Dark))
                        .with_title("Sprout")
                        .with_fullsize_content_view(true)
                        .with_title_hidden(true)
                        .with_titlebar_transparent(true)
                        .with_titlebar_buttons_hidden(true)
                        .with_inner_size(LogicalSize::new(1000, 685))
                        .with_resizable(false),
                ),
        )
        .launch(App);
}

#[cfg(target_os = "linux")]
fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((34, 47, 62, 1))
                .with_disable_context_menu(true)
                .with_window(
                    WindowBuilder::new()
                        .with_theme(Some(Theme::Dark))
                        .with_title("Sprout")
                        .with_decorations(false)
                        .with_inner_size(LogicalSize::new(1000, 685))
                        .with_resizable(false),
                ),
        )
        .launch(App);
}

#[cfg(target_os = "windows")]
fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((34, 47, 62, 1))
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

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn launch() {
    eprintln!("Unsupported operating system.");
}
