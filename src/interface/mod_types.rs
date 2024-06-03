//! Class/type definition for a struct
//! which represents a mod on the filesystem.

use std::path::{Path, PathBuf};

pub struct Mod {
    name: String,
    version: String,
    min_api_version: String,
    enabled: bool,
    checked: bool,
    folder: PathBuf,
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
}

// Setters
impl Mod {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}
