#![allow(non_snake_case)]
use crate::MENU;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MenuItemProps {
    name: String,
    #[props(into)]
    to: String,
    hidden: bool,
    icon: Element,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    let ishidden: String = match props.hidden {
        true => "hidden".into(),
        false => "".into(),
    };
    rsx! {
        li { class: "{ishidden}",
            Link {
                to: props.to,
                onclick: move |_| *MENU.write() = false,
                class: "group flex gap-x-3 rounded-md bg-gray-50 p-2 text-sm font-semibold leading-6 text-indigo-600",
                {props.icon}
                {props.name}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SubMenuItemProps {
    name: String,
    letter: String,
    #[props(into)]
    to: String,
}

#[component]
pub fn SubMenuItem(props: SubMenuItemProps) -> Element {
    rsx! {
        li {
            Link {
                to: props.to,
                onclick: move |_| *MENU.write() = false,
                class: "group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-700 hover:bg-gray-50 hover:text-indigo-600",
                span { class: "flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border border-gray-200 bg-white text-[0.625rem] font-medium text-gray-400 group-hover:border-indigo-600 group-hover:text-indigo-600",
                    {props.letter}
                }
                span { class: "truncate", {props.name} }
            }
        }
    }
}
