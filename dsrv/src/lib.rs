#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

pub mod auth;
pub mod b64;
pub mod config;
pub mod envs;
pub use config::web_config;

pub mod server;
pub mod state;
pub use server::*;
pub use state::*;
