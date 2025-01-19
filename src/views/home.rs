use crate::routes::USR;
use crate::Route;
use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        p { class: "p-4",
            "Access proteins and their quantities over tissues and cells using the proteins link to the right."
        }
        p { class: "p-4",
            "The data portal is under embargo until the paper is accepted; reviewers can access the data using the credentials provided in the submission. To log in, please use the login link found at the bottom right."
        }
    }
}
