use crate::Route;
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
