pub mod components;
pub mod interface;
pub mod screens;
pub mod web;

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
}
