#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModListing {
    name: String,
    author: String,
    github_repo: Option<String>,
    source_url: Option<String>,
    compatibility: Option<CompatInfo>,
    mod_pages: Vec<AltVersion>,
    warnings: Vec<String>,
    mod_page_sites: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CompatInfo {
    status: CompatStatus,
    summary: String,
    broke_in: Option<String>,
    unofficial_version: Option<AltVersion>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum CompatStatus {
    Ok,
    Unofficial,
    Workaround,
    Broken,
    Abandoned,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AltVersion {
    url: String,
    text: String,
}

pub fn split_raw_arrays(raw_data: &str) -> Vec<&str> {
    let mut arrays: Vec<&str> = vec![];
    let mut i: usize = 0;
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut t: bool = false;
    let mut lb_stack: usize = 0;

    // check first byte
    let mut chars = raw_data.chars();
    if chars.next() == Some('{') {
        i += 1;
        lb_stack += 1;
    } else {
        return arrays;
    }

    // iterate over the rest
    for c in chars {
        if c == '{' {
            if lb_stack == 0 {
                p1 = i;
                t = false;
            }
            lb_stack += 1;
        } else if c == '}' {
            lb_stack -= 1;
            if lb_stack == 0 {
                p2 = i + 1;
            }
        }
        if lb_stack == 0 && !t {
            arrays.push(&raw_data[p1..p2]);
            t = !t;
        }
        i += c.len_utf8();
    }

    arrays
}
