pub mod components;
pub mod interface;
pub mod screens;

use manganis::*;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub game_path: PathBuf,
}

// pub const LOGO_1: ImageAsset = mg!(image("./public/sprout@1x.png"));
pub const LOGO_1: &str = "placeholder"; // necessary bc stupid compiler
