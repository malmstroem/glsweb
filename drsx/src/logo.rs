#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CompanyLogoProps {
    logo_url: String,
    alt: String,
}

pub fn CompanyLogo(props: CompanyLogoProps) -> Element {
    rsx! {

        div { class: "flex h-32 shrink-0 items-center bg-white",
            img {
                class: "h-24 w-auto bg-white",
                src: props.logo_url,
                alt: props.alt,
            }
        }
    }
}
