//! A module to interact with SMAPI.
//! The scope covers both SMAPI the app and the website, however
//! there are only web-related functions planned so far.

use std::sync::{Arc, LazyLock};

use dioxus::logger::tracing::{debug, warn};

use super::mod_decoder::{ModListing, split_raw_arrays};

pub static MOD_LISTINGS: LazyLock<Arc<Vec<ModListing>>> = LazyLock::new(|| {
    let raw_contents: String = get_raw_mod_list().expect("Failed to make web request!");
    let raw_list: Vec<&str> = split_raw_arrays(&raw_contents);
    Arc::new(
        raw_list
            .iter()
            .filter_map(|s| match json5::from_str::<ModListing>(s) {
                Ok(m) => {
                    debug!(?m.name, "Loaded mod");
                    Some(m)
                }
                Err(e) => {
                    warn!(?s, ?e, "Failed to parse mod!");
                    None
                }
            })
            .collect(),
    )
});

pub fn get_listings_cached() -> Arc<Vec<ModListing>> {
    MOD_LISTINGS.clone()
}

pub fn get_all_mod_listings() -> Vec<ModListing> {
    let raw_contents: String = get_raw_mod_list().expect("Failed to make web request!");
    let raw_list: Vec<&str> = split_raw_arrays(&raw_contents);
    raw_list
        .iter()
        .map(|s| json5::from_str(s).expect("Failed to parse mod!"))
        .collect()
}

pub fn get_raw_mod_list() -> reqwest::Result<String> {
    let buffer: String = reqwest::blocking::get("https://smapi.io/mods")?.text()?;

    let mut lb_count = 1;
    let raw_data: String = buffer
        .chars()
        .skip_while(|&c| c != '[')
        .skip(1)
        .take_while(|&c| {
            if c == '[' {
                lb_count += 1;
            } else if c == ']' {
                lb_count -= 1;
            }
            lb_count != 0
        })
        .collect();

    Ok(raw_data)
}
