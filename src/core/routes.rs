use crate::component::home::Home;
use crate::component::protein::{ListProteins, ProteinDetails};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/protein/")]
    ListProteins {},
    #[route("/protein/:id")]
    ProteinDetails { id: i32 },
}
