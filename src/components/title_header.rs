use dioxus::desktop::window;
use dioxus::document::Link;
use dioxus::prelude::*;

#[component]
// #[cfg(not(target_os = "macos"))]
pub fn TitleHeader(sub_title: String) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("src/components/css/title_header.css") }
        if std::env::consts::OS != "macos" {
            div {
                class: "title-buttons",
                button {
                    class: "material-symbols-outlined button title-button",
                    style: "color:#ff9f43",
                    onclick: |_| window().set_minimized(true),
                    "close_fullscreen"
                },
                button {
                    class: "material-symbols-outlined button title-button",
                    style: "color:#ee5253",
                    onclick: |_| window().close(),
                    "cancel"
                }
            }
        }
        div {
            onmousedown: |_| window().drag(),
            id: "title-header",
            class: "w-full flex flex-row items-center justify-center gap-4 p-5",
            button {
                class: "mx-2",
                img { class: "sprout-icon", src: asset!("public/sprout@1x-opt.webp") },
            }
            h1 { class: "text-5xl font-bold text-base-content", "Sprout" },
            h1 { class: "text-5xl font-normal text-base-content", "|" },
            h1 { class: "text-5xl font-light text-base-content", "{sub_title}" }
        }

        hr { class: "h-px border-accent my-1 opacity-50 drop-shadow-sm" }
        // hr { class: "divider" }
    }
}

// #[component]
// #[cfg(target_os = "macos")]
// pub fn TitleHeader(sub_title: String) -> Element {
//     rsx! {
//         Link { rel: "stylesheet", href: asset!("src/components/css/title_header.css") }
//         div {
//             onmousedown: |_| window().drag(),
//             id: "title-header",
//             class: "title-header",
//             img { class: "sprout-icon", src: asset!("public/sprout@1x-opt.webp") },
//             h1 { class: "text-5xl font-bold text-base-content", "Sprout" },
//             h1 { class: "text-5xl font-normal text-base-content", "|" },
//             h1 { class: "text-5xl font-light text-base-content", "{sub_title}" }
//         }

//         hr { class: "h-px border-accent my-1 opacity-50 drop-shadow-sm" }
//         // hr { class: "divider" }
//     }
// }
