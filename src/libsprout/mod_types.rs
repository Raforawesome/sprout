//! Class/type definition for a struct
//! which represents a mod on the filesystem.

use crate::libsprout::path_manager;
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
    enabled: Signal<bool>,
    folder: PathBuf,
    disabled_folder: PathBuf,
    enabled_folder: PathBuf,
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
        (self.enabled)()
    }

    pub fn folder(&self) -> &Path {
        self.folder.as_path()
    }
}

// Setters
impl Mod {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled.set(enabled);
    }

    pub fn flip_enabled(&mut self) {
        self.enabled.with_mut(|b| *b = !*b);
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

    pub fn set_folder(&mut self, folder: PathBuf, mods_path: &Path) {
        let folder_name = folder.file_name().unwrap().to_str().unwrap();
        self.disabled_folder = path_manager::disabled_mods_dir().join(folder_name);
        self.enabled_folder = mods_path.join(folder_name);
        self.folder = folder;
    }
}

impl Mod {
    pub fn enable(&mut self) -> std::io::Result<()> {
        self.folder = self.enabled_folder.clone();
        std::fs::rename(
            self.disabled_folder.as_path(),
            self.enabled_folder.as_path(),
        )
    }

    pub fn disable(&mut self) -> std::io::Result<()> {
        self.folder = self.disabled_folder.clone();
        std::fs::rename(
            self.enabled_folder.as_path(),
            self.disabled_folder.as_path(),
        )
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
