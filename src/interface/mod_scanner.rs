//! Module to scan the filesystem for
//! SMAPI mods and create Mod objects from them

use super::mod_types::{Manifest, Mod};
use json5;
use std::path::{Path, PathBuf};

/// # Scanner system
/// This function scans the provided game directory
/// for mod folders containing a manifest.json, then
/// uses that information to create a [Vec] of [Mod] objects.
pub fn find_active_mods(p: &Path) -> Vec<Mod> {
    #[cfg(not(target_os = "macos"))]
    let mod_dir: PathBuf = p.join("Mods/");
    #[cfg(target_os = "macos")]
    let mod_dir: PathBuf = p.join("Contents/MacOS/Mods/");

    mod_dir
        .read_dir()
        .expect("Failed to read mods folder!")
        .filter_map(Result::ok) // filter out inaccessible files
        .filter(|f| f.file_type().is_ok_and(|f| f.is_dir())) // only read folders
        .map(|entry| entry.path().join("manifest.json")) // convert items to file path
        .filter(|path| path.exists()) // filter out non-mod folders
        .filter_map(|path| std::fs::read_to_string(path).ok()) // Filter out files we can't read
        .filter_map(|manifest_json| {
            let manifest: Manifest = json5::from_str(&manifest_json).ok()?;
            let mut mod_struct: Mod = Mod::default();
            mod_struct.set_name(manifest.Name);
            mod_struct.set_version(manifest.Version);
            if let Some(s) = manifest.MinimumApiVersion {
                mod_struct.set_min_api(s);
            } else {
                mod_struct.set_min_api(String::from("None"));
            }
            mod_struct.set_enabled(true);
            Some(mod_struct)
        })
        .collect::<Vec<Mod>>()
}
