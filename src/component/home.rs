#![allow(non_snake_case)]
use crate::Layout;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let content = rsx! {
        div { class: "divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow",
            div { class: "px-4 py-5 sm:px-6", "GLS Dashboard" }
            div { class: "px-4 py-5 sm:p-6 m-4", "Details goes here" }
        }
    };
    rsx! {
        Layout { content }
    }
}
