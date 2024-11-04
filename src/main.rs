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
    prelude::*,
};
use import_screen::ImportScreen;
use index_screen::IndexScreen;
use mod_screen::ModScreen;
use sprout::interface::location_manager;
use sprout::screens::{import_screen, index_screen, mod_screen};
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
        style { {include_str!("../public/global.css")} }
        Router::<Routes> {}
    }
}

#[cfg(target_os = "macos")]
fn main() {
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

#[cfg(target_os = "windows")]
fn main() {
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
