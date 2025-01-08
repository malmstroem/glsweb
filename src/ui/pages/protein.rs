use crate::server::functions::{get_permissions, get_user_name, login, logout};
use crate::server::{get_protein_q, get_proteins, Protein, QM};
use dioxus_logger::tracing::info;

use charming::{component::Axis, element::AxisType, series::Bar, Chart, WasmRenderer};

use crate::ui::routes::USR;
use crate::ui::Route;
use dioxus::prelude::*;

pub fn ProteinList() -> Element {
    let mut items: Signal<Vec<Protein>> = use_signal(|| vec![]);
    let data = use_resource(move || get_proteins(items.read().len() as i32));

    let ren = match &*data.read_unchecked() {
        Some(Ok(dtas)) => rsx! {
            div {
                class: "bg-white rounded-md w-full min-w-44",
                for protein in dtas {
                    ProteinRow { protein: protein.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! { "An error occurred while fetching the data: {err}" },
        None => rsx! {
            p { class: "p-4", "Loading items" }
        },
    };
    let c1 = rsx! {
        button {
            class: "inline-flex items-center gap-x-1.5 rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
            onclick: move |_| async move {
                if let Ok(data) = get_proteins(items.len() as i32).await {
                    items.extend(data);
                }
            },
            "Load more data"
        }
    };
    rsx! {
    div { class: "bg-gray-100",
        div { class: "flex flex-row min-h-12 justify-center items-center drop-shadow-2xl",
                {ren}
            }
        }
        {c1}
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ProteinRowProps {
    pub protein: Protein,
}

#[component]
pub fn ProteinRow(props: ProteinRowProps) -> Element {
    rsx! {
        div { class: "flex min-w-40 w-full shadow-sm pb-4 m-2", id: "pri:{props.protein.id}", key: "prk:{props.protein.id}",
            div { class: "min-w-0 flex-auto",
                p { class: "text-black text-lg font-bold", "{props.protein.entry}" }
                p { class: "text-black text-sm", "{props.protein.entry_name} / {props.protein.frm}" }
            }
            div { class: "hidden shrink-0 sm:flex sm:flex-col sm:items-end overflow-x-clip",
                p { class: "text-black text-sm", "{props.protein.id}" }
                Link {
                    class: "inline-flex items-center gap-x-1.5 rounded-md bg-indigo-600 px-2.5 py-1.5 my-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    id: "proteinid{props.protein.id}",
                    key: "protein{props.protein.id}",
                    to: Route::ProteinDetails { id: props.protein.id},

                    "View details (id: {props.protein.id})"
                }

            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProteinDetailProps {
    pub id: i32,
}

#[component]
pub fn ProteinDetails(props: ProteinDetailProps) -> Element {
    let mut iid: Signal<i32> = use_signal(|| props.id);
    let rslt = use_resource(move || async move {
        let renderer: WasmRenderer = WasmRenderer::new(600, 400);
        let mut data = vec![];
        let mut ndata = vec![];
        let ds = get_protein_q(*iid.read()).await.unwrap();
        for d in ds {
            data.push(d.tissue.clone());
            ndata.push(d.value);
        }
        let chart = Chart::new()
            .x_axis(Axis::new().type_(AxisType::Category).data(data))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(ndata));
        renderer.render(&format!("chart"), &chart)
    });
    let ren = match &*rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! { p { "ok"}}
        }
        Some(Err(e)) => {
            rsx! { p { "err: {e}"}}
        }
        _ => {
            rsx! { p { "Not ok"}}
        }
    };
    rsx! {
        div { class:"divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow",
           div { class:"px-4 py-5 sm:px-6",
               "Protein Details for {props.id}"
           }
           div { class:"px-8 py-8 sm:p-6 m-4",
               "Details goes here"
           }
           div {
             id: "chart",
             style: "display: inline-block;",
           }
           {ren}
        }
    }
}
