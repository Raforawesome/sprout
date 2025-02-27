//! # Location Manager
//! A module to locate and maintain references to key locations
//! for the program, such as enabled/disabled mod directories.

// Dirs needed:
// Enabled mods dir (inside game folder)
// Disabled mods dir (probably in $XDG_CONFIG_HOME/.sprout/disabled)

use std::alloc::{Layout, alloc};
use std::cell::LazyCell;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

thread_local! {
    static GAME_PATH: LazyCell<*mut PathBuf> = LazyCell::new(|| unsafe {
        alloc(Layout::new::<PathBuf>()) as *mut PathBuf
    });
    static MODS_PATH: LazyCell<*mut PathBuf> = LazyCell::new(|| unsafe {
        alloc(Layout::new::<PathBuf>()) as *mut PathBuf
    });
}

#[cfg(not(target_os = "macos"))]
pub fn set_game_path(p: PathBuf) {
    unsafe {
        MODS_PATH.with(|ptr| ***ptr = p.join("Mods/"));
        GAME_PATH.with(|ptr| ***ptr = p);
    }
}

#[cfg(target_os = "macos")]
pub fn set_game_path(p: PathBuf) {
    unsafe {
        MODS_PATH.with(|ptr| ***ptr = p.join("Contents/MacOS/Mods/"));
        GAME_PATH.with(|ptr| ***ptr = p);
    }
}

pub fn get_game_path() -> &'static Path {
    unsafe { GAME_PATH.with(|ptr| ptr.as_ref().unwrap()) }
}

pub fn get_mods_path() -> &'static Path {
    MODS_PATH.with(|ptr| unsafe { ptr.as_ref().unwrap() })
}

pub fn sprout_home_dir() -> &'static Path {
    static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let home: PathBuf = dirs::data_local_dir().expect("Failed to find home dir!");
        let sprout_dir: PathBuf = home.join(".sprout");
        if !sprout_dir.exists() {
            std::fs::create_dir_all(&sprout_dir).expect("Failed to create sprout dir!");
        }
        sprout_dir
    });
    HOME_DIR.as_path()
}

pub fn disabled_mods_dir() -> &'static Path {
    static DISABLED_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let disabled_dir: PathBuf = sprout_home_dir().join("disabled");
        if !disabled_dir.exists() {
            std::fs::create_dir_all(&disabled_dir).expect("Failed to create disabled dir!");
        }
        disabled_dir
    });
    DISABLED_DIR.as_path()
}

// pub fn get_data_dir() -> PathBuf {
//     let mut path = dirs::data_dir().unwrap_or_else(|| {
//         eprintln!("Unsupported operating system.");
//         std::process::exit(1);
//     });
//     path.push(".sprout/");
//     if !path.exists() {
//         std::fs::create_dir(&path).unwrap_or_else(|_| {
//             eprintln!("No permission to write to data directory.");
//             std::process::exit(1);
//         });
//     };
//     path
// }
