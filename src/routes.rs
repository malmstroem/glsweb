use crate::views::{About, Home, Login, Logout, ProteinDetails, ProteinList};
use dioxus::prelude::*;

use drsx::{Avatar, Icon, IconProps, Icons, MenuItem, NavLayout, SubMenuItem};

pub static USR: GlobalSignal<String> = Signal::global(|| "Guest".into());

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/protein/")]
        ProteinList {},
        #[route("/protein/:id")]
        ProteinDetails { id: i32 },
        #[route("/login/")]
        Login {},
        #[route("/logout/")]
        Logout {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn get_sub_menu_items() -> Vec<Element> {
    vec![]
}

fn get_menu_items() -> Vec<Element> {
    let items = vec![
        rsx! {MenuItem {
            name: "Home",
            to: Route::Home {},
            icon: Icon(IconProps { icon: Icons::Home }),
            hidden: false,
        }},
        rsx! {MenuItem {
            name: "Proteins",
            to: Route::ProteinList {},
            icon: Icon(IconProps { icon: Icons::Home }),
            hidden: false,
        }},
        rsx! {MenuItem {
            name: "About",
            to: Route::About{},
            icon: Icon(IconProps { icon: Icons::About }),
            hidden: false,
        }},
    ];
    items
}

fn get_avatar() -> Element {
    let usr = USR.read();
    let name = "User";
    let imgurl = "/assets/user_avatar.svg";
    rsx! {
        Avatar {
            name: usr.clone(),
            imgurl,
            login: Route::Login {},
            logout: Route::Logout {},
        }
    }
}

#[component]
fn NavBar() -> Element {
    let menuitems = get_menu_items();
    let submenuitems = get_sub_menu_items();
    let avatar = get_avatar();
    let content = rsx! {
        Outlet::<Route> {}
    };

    rsx! {
        NavLayout {
            submenu_title: "",
            logo_url: "/assets/logo.png",
            logo_alt: "GLS",
            menuitems,
            submenuitems,
            avatar,
            content,
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "The page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
