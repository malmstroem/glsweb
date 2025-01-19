#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[cfg(feature = "server")]
use dsrv::web_config;

mod app;
mod routes;
mod views;
pub use app::App;
pub use routes::Route;

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "server")]
    dsrv::server::server_start(App)
}
