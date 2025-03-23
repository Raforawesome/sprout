pub mod components;
pub mod libsprout;
pub mod views;

use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder, tao::window::Theme},
    signals::GlobalSignal,
};
pub use libsprout::{mod_scanner, mod_types, path_manager, smapi};
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
    pub mods_path: PathBuf,
}

// OS-specific launch configs
#[cfg(target_os = "macos")]
use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

pub static THEME: GlobalSignal<&str> = GlobalSignal::new(|| "abyss");

#[cfg(target_os = "macos")]
pub fn launch_config() -> Config {
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
                // .with_titlebar_buttons_hidden(true)
                .with_inner_size(LogicalSize::new(1000, 685))
                .with_resizable(true),
        )
}

#[cfg(target_os = "linux")]
pub fn launch_config(visible: bool) -> Config {
    Config::new()
        .with_background_color((34, 47, 62, 1))
        .with_disable_context_menu(true)
        .with_window(
            WindowBuilder::new()
                .with_theme(Some(Theme::Dark))
                .with_title("Sprout")
                .with_decorations(false)
                .with_inner_size(LogicalSize::new(1000, 685))
                .with_resizable(false)
                .with_visible(visible),
        )
}

#[cfg(target_os = "windows")]
pub fn launch_config(visible: bool) -> Config {
    Config::new()
        .with_background_color((34, 47, 62, 1))
        .with_disable_context_menu(true)
        .with_window(
            WindowBuilder::new()
                .with_theme(Some(Theme::Dark))
                .with_title("Sprout")
                .with_inner_size(LogicalSize::new(1000, 685))
                .with_resizable(false)
                .with_decorations(false)
                .with_visible(false),
        )
}

// unit tests
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let app_state = AppState {
            game_path: PathBuf::from("C:/Program Files/Stardew Valley"),
            mods_path: PathBuf::from("C:/Program Files/Stardew Valley/Mods"),
        };

        assert_eq!(
            app_state.game_path,
            PathBuf::from("C:/Program Files/Stardew Valley")
        );
    }

    #[test]
    fn test_get_raw_mod_data() {
        use crate::smapi::fetcher::get_raw_mod_list;
        let result = get_raw_mod_list();
        assert!(result.is_ok());

        let data = result.unwrap();
        // println!("{data}");
        println!("{}", &data[0..1000]);
    }

    #[test]
    fn test_split_raw_arrays() {
        use crate::libsprout::smapi::fetcher::get_raw_mod_list;
        use crate::libsprout::smapi::mod_decoder::split_raw_arrays;

        let result: String = get_raw_mod_list().unwrap();
        let arrays: Vec<&str> = split_raw_arrays(&result);

        println!("{}", arrays[0]);
    }

    #[test]
    fn test_decode_all_mods() {
        use crate::libsprout::smapi::fetcher::get_raw_mod_list;
        use crate::libsprout::smapi::mod_decoder::{ModListing, split_raw_arrays};

        let result: String = get_raw_mod_list().unwrap();
        let arrays: Vec<&str> = split_raw_arrays(&result);
        let mods: Vec<ModListing> = arrays
            .iter()
            .map(|s| json5::from_str(s).expect("Failed to parse mod!"))
            .collect();

        dbg!(&mods[11]);
    }
}
