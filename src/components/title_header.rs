use dioxus::prelude::*;
use manganis::{mg, ImageAsset};

pub const ICON: ImageAsset = mg!(image("./public/sprout@1x.png")
    .size(440, 657)
    // .format(ImageType::Png)
    .preload());

#[component]
pub fn TitleHeader() -> Element {
    rsx! {
        style { {include_str!("css/title_header.css")} }
        div {
            id: "title-header",
            class: "title-header",
            img {
                src: "{ICON}"
            }
        }
    }
}
