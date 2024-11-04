pub mod components;
pub mod interface;
pub mod screens;
pub mod web;

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
        use crate::web::smapi_fetcher::get_raw_mod_data;
        let result = get_raw_mod_data();
        assert!(result.is_ok());

        let data = result.unwrap();
        // assert!(data.starts_with('['));
        // assert!(data.ends_with(']'));
        println!("{data}");
    }
}
