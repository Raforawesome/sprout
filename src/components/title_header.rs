use dioxus::prelude::*;
use manganis::mg;

pub const ICON: &str = mg!(image("./public/sprout@1x.png")
    .size(440, 657)
    // .format(ImageType::Png)
    .preload())
.path();

#[component]
pub fn TitleHeader(sub_title: &'static str) -> Element {
    rsx! {
        style { {include_str!("css/title_header.css")} }
        div {
            id: "title-header",
            class: "title-header",
            img { class: "icon", src: "/sprout@1x.png" },
            h1 { class: "title", "Sprout" },
            h1 { class: "subtitle", "|" },
            h1 { class: "subtitle", "{sub_title}" }
        }

        hr { class: "divider" }

        div {
            id: "import",
            class: "import",
            div {
                p { class: "label", "Game location:" }
                input {
                    id: "class-box",
                    class: "path-box"
                }
            }
        }
    }
}
