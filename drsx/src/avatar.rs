#![allow(non_snake_case)]
use crate::MENU;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    name: String,
    imgurl: String,
    #[props(into)]
    login: String,
    #[props(into)]
    logout: String,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let loginhidden: String = match props.name.as_str() {
        "Guest" => "".into(),
        _ => "hidden".into(),
    };
    let logouthidden: String = match props.name.as_str() {
        "Guest" => "hidden".into(),
        _ => "".into(),
    };
    rsx! {
        div { class: "flex items-center gap-x-4 px-6 py-3 text-sm font-semibold leading-6 text-gray-900 hover:bg-gray-50",
            a { href: "#",
                img {
                    class: "h-8 w-8 rounded-full bg-gray-50",
                    src: props.imgurl,
                    alt: props.name.clone(),
                }
                span { class: "sr-only", "Your profile" }
                span { {props.name.clone()} }
                span {
                    Link {
                        to: props.login,
                        onclick: move |_| *MENU.write() = false,
                        class: "{loginhidden} group flex gap-x-3 rounded-md bg-gray-50 p-2 text-sm font-semibold leading-6 text-indigo-600",
                        "Login"
                    }
                }
                span {
                    Link {
                        to: props.logout,
                        onclick: move |_| *MENU.write() = false,
                        class: "{logouthidden} group flex gap-x-3 rounded-md bg-gray-50 p-2 text-sm font-semibold leading-6 text-indigo-600",
                        "Logout"
                    }
                }
            }
        }
    }
}
