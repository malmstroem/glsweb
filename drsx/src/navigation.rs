#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SmNavigationProps {
    title: String,
    menuitems: Vec<Element>,
    submenuitems: Vec<Element>,
    avatar: Element,
}

pub fn SmNavigation(props: SmNavigationProps) -> Element {
    rsx! {
        nav { class: "flex flex-1 flex-col",
            ul { role: "list", class: "flex flex-1 flex-col gap-y-7",
                li {
                    ul { role: "list", class: "-mx-2 space-y-1",
                        for menuitem in &props.menuitems {
                            {menuitem}
                        }
                    }
                }
                li {
                    div { class: "text-xs font-semibold leading-6 text-gray-400", {props.title} }
                    ul { role: "list", class: "-mx-2 mt-2 space-y-1",
                        for submenuitem in &props.submenuitems {
                            {submenuitem}
                        }
                    }
                }
                li { class: "-mx-6 mt-auto", {props.avatar} }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LgNavigationProps {
    title: String,
    menuitems: Vec<Element>,
    submenuitems: Vec<Element>,
    avatar: Element,
}

pub fn LgNavigation(props: LgNavigationProps) -> Element {
    rsx! {
        nav { class: "flex flex-1 flex-col",
            ul { role: "list", class: "flex flex-1 flex-col gap-y-7",
                li {
                    ul { role: "list", class: "-mx-2 space-y-1",
                        for menuitem in &props.menuitems {
                            {menuitem}
                        }
                    }
                }
                li {
                    div { class: "text-xs font-semibold leading-6 text-gray-400", {props.title} }
                    ul { role: "list", class: "-mx-2 mt-2 space-y-1",
                        for submenuitem in &props.submenuitems {
                            {submenuitem}
                        }
                    }
                }
                li { class: "-mx-6 mt-auto", {props.avatar} }
            }
        }
    }
}
