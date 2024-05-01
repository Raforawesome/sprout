use dioxus::prelude::*;

#[component]
pub fn IndexScreen() -> Element {
    let nav: Navigator = navigator();
    nav.replace("/import");
    None
}
