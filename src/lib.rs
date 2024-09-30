pub mod components;
pub mod interface;
pub mod screens;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
}
