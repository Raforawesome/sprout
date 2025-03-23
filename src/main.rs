#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
//! <h1>Sprout</h1>
//! <h5>A Stardew Valley Mod Manager</h5>
//! <hr>
//! Sprout is a simple SMAPI mod manager for Stardew Valley.
//! Currently caching is not implemented, and it works on a simple
//! no-DB file-scan system.

use std::{sync, thread};

use dioxus::{document::Stylesheet, logger::tracing::Level, prelude::*};
use document::Script;
use sprout::{
    AppState, THEME, libsprout,
    views::{import::ImportScreen, index::IndexScreen, mod_screen::ModScreen},
};

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
        Stylesheet { href: asset!("public/global.css") }
        Stylesheet { href: asset!("public/daisyui.css") }
        Stylesheet { href: asset!("public/daisy_themes.css") }
        Script { src: asset!("public/tailwind.js") }
        div {
            "data-theme": "{THEME}",
            class: "bg-base-200 flex flex-col h-screen w-screen",
            Router::<Routes> {}
        }
    }
}

fn main() {
    // initialize dioxus logger with custom trace level
    dioxus::logger::init(Level::DEBUG).expect("Failed to initialize logger");

    // force mod listings to be fetched on another thread as they take
    // time to parse, and shouldn't be generated on-the-fly when required.
    let _ = thread::spawn(|| {
        sync::LazyLock::force(&libsprout::smapi::fetcher::MOD_LISTINGS);
    });

    launch();
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(sprout::launch_config())
        .launch(App);
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn launch() {
    eprintln!("Unsupported operating system.");
}
