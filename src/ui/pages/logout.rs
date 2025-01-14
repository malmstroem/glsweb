use crate::server::functions::{get_user_name, logout};
use crate::ui::routes::USR;
use crate::ui::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Logout() -> Element {
    info!("Logging out");
    let logout = use_resource(logout);
    if let Some(Ok(logout)) = &*logout.read_unchecked() { info!("logged out {logout:?}") }
    let username = use_resource(get_user_name);
    if let Some(Ok(username)) = &*username.read_unchecked() { *USR.write() = username.clone() }
    rsx! {
        p { {"Logged out"} }
    }
}
