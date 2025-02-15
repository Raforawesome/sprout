pub mod components;
pub mod interface;
pub mod screens;
pub mod smapi;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let app_state = AppState {
            game_path: PathBuf::from("C:/Program Files/Stardew Valley"),
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
        use crate::smapi::fetcher::get_raw_mod_list;
        use crate::smapi::mod_decoder::split_raw_arrays;

        let result: String = get_raw_mod_list().unwrap();
        let arrays: Vec<&str> = split_raw_arrays(&result);

        println!("{}", arrays[0]);
    }

    #[test]
    fn test_decode_all_mods() {
        use crate::smapi::fetcher::get_raw_mod_list;
        use crate::smapi::mod_decoder::{split_raw_arrays, ModListing};

        let result: String = get_raw_mod_list().unwrap();
        let arrays: Vec<&str> = split_raw_arrays(&result);
        let mods: Vec<ModListing> = arrays
            .iter()
            .map(|s| json5::from_str(s).expect("Failed to parse mod!"))
            .collect();

        dbg!(&mods[11]);
    }
}
