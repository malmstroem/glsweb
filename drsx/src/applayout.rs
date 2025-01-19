#![allow(non_snake_case)]

use crate::MENU;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AppLayoutProps {
    title: String,
    submenu_title: String,
    fav_icon_src: String,
    logo_url: String,
    logo_alt: String,
    stylesheets: Vec<String>,
    scripts: Vec<String>,
    menuitems: Vec<Element>,
    submenuitems: Vec<Element>,
    avatar: Element,
    content: Element,
}

pub fn AppLayout(props: AppLayoutProps) -> Element {
    let show_menu = MENU.read();
    let _menu_state = match *show_menu {
        false => "hidden",
        true => "",
    };
    rsx!(
        head {
            title { "{props.title}" }
            meta { charset: "utf-8" }
            meta { "http-equiv": "X-UA-Compatible", content: "IE=edge" }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1",
            }
            for href in &props.stylesheets {
                link { rel: "stylesheet", href: "{href}", "type": "text/css" }
            }
            for script in &props.scripts {
                script { "type": "module", src: "{script}" }
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{props.fav_icon_src}",
            }
        }
        body { class: "bg-gray-100",
            div {
                div { class: "relative z-50 lg:hidden p-4",

                    button {
                        r#type: "button",
                        onclick: move |_| *MENU.write() = true,

                        svg {
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",
                            class: "size-6",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                            }
                        }
                    }
                }
                div { class: "hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col",
                    div { class: "flex grow flex-col gap-y-5 overflow-y-auto border-r border-gray-200 bg-white px-6",
                        crate::CompanyLogo {
                            logo_url: props.logo_url.clone(),
                            alt: props.logo_alt.clone(),
                        }
                        crate::LgNavigation {
                            title: props.submenu_title.clone(),
                            menuitems: props.menuitems.clone(),
                            submenuitems: props.submenuitems.clone(),
                            avatar: props.avatar.clone(),
                        }
                    }
                    div { class: "sticky top-0 z-40 flex items-center gap-x-6 bg-white px-4 py-4 shadow-sm sm:px-6 lg:hidden",
                        button {
                            r#type: "button",
                            class: "-m-2.5 p-2.5 text-gray-700 lg:hidden",
                            span { class: "sr-only", "Open sidebar" }
                            svg {
                                class: "h-6 w-6",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                                }
                            }
                        }
                        div { class: "flex-1 text-sm font-semibold leading-6 text-gray-900",
                            "Dashboard"
                        }
                        {props.avatar}
                    }
                }

                main { class: "py-10 lg:pl-72 m-2",
                    div { class: "px-4 py-2 sm:px-6 lg:px-8 bg-white rounded-lg min-h-96",
                        {props.content}
                    }
                }
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct NavLayoutProps {
    submenu_title: String,
    logo_url: String,
    logo_alt: String,
    menuitems: Vec<Element>,
    submenuitems: Vec<Element>,
    avatar: Element,
    content: Element,
}

#[component]
pub fn NavLayout(props: NavLayoutProps) -> Element {
    let show_menu = MENU.read();
    let menu_state = match *show_menu {
        false => "hidden",
        true => "",
    };
    rsx! {
        div { class: "relative z-50 lg:hidden p-4",

            button { r#type: "button", onclick: move |_| *MENU.write() = true,

                svg {
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    class: "size-6",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                    }
                }
            }
        }
        div { class: "{menu_state} relative z-50 lg:hidden", role: "dialog",
            div { class: "fixed inset-0 bg-gray-900/80" }

            div { class: "fixed inset-0 flex",
                div { class: "relative mr-16 flex w-full max-w-xs flex-1",
                    crate::CloseSideBar {}
                    div { class: "flex grow flex-col gap-y-5 overflow-y-auto bg-white px-6 pb-2",
                        crate::CompanyLogo {
                            logo_url: props.logo_url.clone(),
                            alt: props.logo_alt.clone(),
                        }
                        crate::SmNavigation {
                            title: props.submenu_title.clone(),
                            menuitems: props.menuitems.clone(),
                            submenuitems: props.submenuitems.clone(),
                            avatar: props.avatar.clone(),
                        }
                    }
                }
            }
        }
        div { class: "hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col",
            div { class: "flex grow flex-col gap-y-5 overflow-y-auto border-r border-gray-200 bg-white px-6",
                crate::CompanyLogo {
                    logo_url: props.logo_url.clone(),
                    alt: props.logo_alt.clone(),
                }
                crate::LgNavigation {
                    title: props.submenu_title,
                    menuitems: props.menuitems,
                    submenuitems: props.submenuitems,
                    avatar: props.avatar.clone(),
                }
            }
            div { class: "sticky top-0 z-40 flex items-center gap-x-6 bg-white px-4 py-4 shadow-sm sm:px-6 lg:hidden",
                button {
                    r#type: "button",
                    class: "-m-2.5 p-2.5 text-gray-700 lg:hidden",
                    span { class: "sr-only", "Open sidebar" }
                    svg {
                        class: "h-6 w-6",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                        }
                    }
                }
                div { class: "flex-1 text-sm font-semibold leading-6 text-gray-900",
                    "Dashboard"
                }
                {props.avatar}
            }
        }
        main { class: "py-10 lg:pl-72 m-2",
            div { class: "px-4 py-2 sm:px-6 lg:px-8 bg-white rounded-lg min-h-96", {props.content} }
        }
    }
}
