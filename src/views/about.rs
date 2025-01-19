use dioxus::prelude::*;

pub fn About() -> Element {
    rsx! {
        h3 { "About" }
        p { "The about text is not available yet." }
    }
}
