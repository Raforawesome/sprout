//! A module to interact with SMAPI.
//! The scope covers both SMAPI the app and the website, however
//! there are only web-related functions planned so far.

fn get_raw_mod_data() -> reqwest::Result<String> {
    let buffer: String = reqwest::blocking::get("https://smapi.io/mods")?.text()?;

    let mut lb_count = 0;
    let raw_data: String = buffer.chars()
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

fn split_raw_arrays(raw_data: String) -> Vec<String> {
    let mut arrays: Vec<String> = vec![];
    let mut buffer: String = String::new();
    let mut lb_stack: bool = false;
    arrays
}
