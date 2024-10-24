//! Class/type definition for a struct
//! which represents a mod on the filesystem.

use dioxus::prelude::*;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Rust native abstraction representing a
/// mod folder in the Stardew Valley mods directory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Mod {
    name: String,
    version: String,
    min_api_version: String,
    enabled: bool,
    enabled_signal: Signal<bool>,
    folder: PathBuf,
    checked: bool,
}

// Getters
impl Mod {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn min_api_version(&self) -> &str {
        &self.min_api_version
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn checked(&self) -> bool {
        self.checked
    }

    pub fn folder(&self) -> &Path {
        self.folder.as_path()
    }

    pub fn enabled_signal(&self) -> Signal<bool> {
        self.enabled_signal
    }
}

// Setters
impl Mod {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.enabled_signal.set(enabled);
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        // println!("{checked}");
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_min_api(&mut self, min: String) {
        self.min_api_version = min;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn set_folder(&mut self, folder: PathBuf) {
        self.folder = folder;
    }
}

/// Serde structure to destructure a mod's manifest.json
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Manifest {
    pub Name: String,
    pub Version: String,
    pub MinimumApiVersion: Option<String>,
}
