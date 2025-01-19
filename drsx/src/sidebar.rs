#![allow(non_snake_case)]
use dioxus::prelude::*;

pub static MENU: GlobalSignal<bool> = Signal::global(|| false);

pub fn CloseSideBar() -> Element {
    rsx! {
        div { class: "absolute left-full top-0 flex w-16 justify-center pt-5",
            button {
                r#type: "button",
                class: "-m-2.5 p-2.5",
                onclick: move |_| *MENU.write() = false,
                span { class: "sr-only", "Close sidebar" }
                svg {
                    class: "h-6 w-6 text-white",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M6 18L18 6M6 6l12 12",
                    }
                }
            }
        }
    }
}
