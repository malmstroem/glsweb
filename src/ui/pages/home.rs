use crate::server::functions::{get_permissions, get_user_name, login, logout};
use crate::ui::routes::USR;
use crate::ui::Route;
use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        p { "Access proteins and their quantities over tissues and cells using the proteins link to the right." }
        p { "The data portal is under embargo until the paper is accepted; reviewers can access the data using the credentials provided in the submission. To log in, please use the login link found at the bottom right." }
    }
}
