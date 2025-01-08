#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[cfg(feature = "server")]
mod auth;
#[cfg(feature = "server")]
mod b64;
#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
mod envs;
#[cfg(feature = "server")]
use config::web_config;

mod server;
mod ui;

fn main() {
    #[cfg(feature = "web")]
    launch(ui::App);

    #[cfg(feature = "server")]
    server::server_start(ui::App)
}
