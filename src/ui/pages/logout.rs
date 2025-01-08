use crate::server::functions::{get_user_name, logout};
use crate::ui::routes::USR;
use crate::ui::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Logout() -> Element {
    info!("Logging out");
    let logout = use_resource(move || logout());
    match &*logout.read_unchecked() {
        Some(Ok(logout)) => info!("logged out {logout:?}"),
        _ => (),
    }
    let username = use_resource(move || get_user_name());
    match &*username.read_unchecked() {
        Some(Ok(username)) => *USR.write() = username.clone(),
        _ => (),
    }
    rsx! {
        p { {"Logged out"} }
    }
}
