use dioxus::{
    document::{Script, Stylesheet},
    prelude::*,
};

use crate::{THEME, components::TitleHeader};

#[component]
pub fn DownloadScreen() -> Element {
    let mod_list = use_resource(async move || {
        let data: String = reqwest::get("https://raw.githubusercontent.com/Pathoschild/SmapiCompatibilityList/refs/heads/develop/data/mods.jsonc")
            .await.unwrap().text().await.unwrap();
        crate::smapi::gh_decoder::parse_gh_data(&data)
    });

    rsx! {
        Stylesheet { href: asset!("public/global.css") }
        Stylesheet { href: asset!("public/daisyui.css") }
        Stylesheet { href: asset!("public/daisy_themes.css") }
        Script { src: asset!("public/tailwind.js") }
        div {
            "data-theme": "{THEME}",
            class: "bg-base-200 flex flex-col h-screen w-screen overflow-x-clip",
            TitleHeader { sub_title: "Download Mods" },
            div {
                class: "flex flex-row flex-wrap overflow-y-auto justify-center gap-4",
                if let Some(mods) = &*mod_list.read() {
                    {mods.iter().map(|m| rsx! {
                        div { class: "card w-75 hover:w-77 h-30 hover:h-32 bg-neutral hover:bg-primary text-neutral-content \
                            hover:text-primary-content duration-300 transition-all overflow-y-hidden hover:overflow-y-auto",
                            div { class: "card-body",
                                h2 { class: "card-title text-neutral-content", {m.name()} }
                                p { class: "text-neutral-content", {m.author()} }
                            }
                        }
                    })}
                }
            }
        }
    }
}
