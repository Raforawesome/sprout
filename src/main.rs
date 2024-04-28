#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use dioxus::{desktop::Config, prelude::*};
/// # Sprout
/// A stardew valley mod manager.

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_background_color((39, 174, 96, 1))
                .with_data_directory(dirs::data_dir().unwrap()) // set to $XDG_CONFIG_HOME/.svsprout
                .with_resource_directory("./assets/")
                .with_disable_context_menu(true), // disable right click menu
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx!("hello, world!")
}
