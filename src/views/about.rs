use dioxus::prelude::*;

pub fn About() -> Element {
    rsx! {
        div { class: "max-w-3xl mx-auto px-6 py-12 space-y-8 text-gray-800 font-sans",
            h3 { class: "text-3xl font-bold text-gray-900 border-b border-gray-300 pb-2",
                "About"
            }
            p { class: "text-base md:text-lg text-gray-700 leading-relaxed",
                "This site uses the "
                a {
                    class: "text-blue-800",
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    "dioxus"
                }
                " web framework."
            }
            p { class: "text-base md:text-lg text-gray-700 leading-relaxed",
                "Data is available at "
                a {
                    class: "text-blue-800",
                    href: "https://doi.org/10.5281/zenodo.14292542",
                    target: "_blank",
                    "zenodo"
                }
                "."

            }

            div { class: "flex h-72 shrink-0 items-center bg-white",
                img {
                    class: "h-64 w-auto bg-white",
                    src: "/assets/logo.png",
                    alt: "logo",
                }
            }
        }
    }
}
