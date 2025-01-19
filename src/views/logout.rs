use crate::routes::USR;
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
//use dsrv::server::functions::{get_user_name, logout};
//
#[cfg(feature = "server")]
use dsrv::auth::{Session, User};

#[component]
pub fn Logout() -> Element {
    info!("Logging out");
    let logout = use_resource(logout);
    if let Some(Ok(logout)) = &*logout.read_unchecked() {
        info!("logged out {logout:?}")
    }
    let username = use_resource(get_user_name);
    if let Some(Ok(username)) = &*username.read_unchecked() {
        *USR.write() = username.clone()
    }
    rsx! {
        p { {"Logged out"} }
    }
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    Ok(session.0.current_user.unwrap().username.to_string())
}

#[server(ServerLogout)]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth: Session = extract().await?;
    auth.logout_user();
    Ok(())
}
