//! Module to scan the filesystem for
//! SMAPI mods and create Mod objects from them

use super::mod_types::{Manifest, Mod};
use std::path::{Path, PathBuf};

pub fn find_mods(p: &Path) -> Vec<Mod> {
    #[cfg(not(target_os = "macos"))]
    let mod_dir: PathBuf = p.join("Mods/");
    #[cfg(target_os = "macos")]
    let mod_dir: PathBuf = p.join("Contents/MacOS/Mods/");

    mod_dir
        .read_dir()
        .expect("Failed to read mods folder!")
        .filter_map(Result::ok) // filter out unreadable files
        .filter(|f| f.file_type().is_ok_and(|f| f.is_dir())) // only read folders
        .map(|entry| entry.path().join("manifest.json")) // convert items to file path
        .filter(|path| path.exists()) // filter out non-mod folders
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .filter_map(|manifest_json| {
            let manifest: Manifest = serde_json::from_str(&manifest_json).ok()?;
            let mut mod_struct: Mod = Mod::default();
            mod_struct.set_name(manifest.Name);
            mod_struct.set_version(manifest.Version);
            mod_struct.set_min_api(manifest.MinimumApiVersion);
            mod_struct.set_enabled(true);
            Some(mod_struct)
        })
        .collect::<Vec<Mod>>()
}
