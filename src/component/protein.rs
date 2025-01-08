#![allow(non_snake_case)]
use crate::core::routes::Route;
use crate::Layout;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use dioxus_logger::tracing;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct Protein {
    pub id: i32,
    pub entry: String,
    pub entry_name: String,
    pub frm: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct ProteinRowProps {
    pub protein: Protein,
}

#[component]
pub fn ProteinRow(props: ProteinRowProps) -> Element {
    rsx! {
        div { class: "flex min-w-0 gap-x-4 my-2", id: "pri:{props.protein.id}", key: "prk:{props.protein.id}",
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
    let content = rsx! {
        div { class:"divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow",
           div { class:"px-4 py-5 sm:px-6",
               "Protein Details for {props.id}"
           }
           div { class:"px-8 py-8 sm:p-6 m-4",
               "Details goes here"
           }
        }
    };
    rsx! { Layout { content }}
}

#[component]
pub fn ListProteins() -> Element {
    let mut proteins: Signal<Vec<Protein>> = use_signal(|| vec![]);

    let c1 = rsx! {
        button {
            class: "inline-flex items-center gap-x-1.5 rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
            onclick: move |_| async move {
                if let Ok(data) = get_proteins(proteins.len() as i32).await {
                    proteins.extend(data);
                }
            },
            "Load Proteins"
        }
    };
    let c2 = rsx! {
        div { role: "list", class: "divide-y divide-gray-300",

            for t in proteins.iter() {
                ProteinRow { protein: t.clone() }
            }
        }
    };
    let content = rsx! {
        {c2}
        {c1}
    };
    rsx! {
        Layout { content }
    }
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, sqlx::FromRow)]
pub struct ProteinQuery {
    pub id: i32,
    pub entry: String,
    pub entry_name: String,
    pub frm: String,
}

#[server(GetServerData)]
pub async fn get_proteins(offset: i32) -> Result<Vec<Protein>, ServerFnError> {
    let pool = crate::pool::POOL.get().await;
    let stmt = format!("SELECT * FROM ac ORDER BY id DESC LIMIT 25 OFFSET {offset}");
    let items: Vec<ProteinQuery> = sqlx::query_as::<_, ProteinQuery>(&stmt)
        .fetch_all(pool)
        .await?;
    let ret: Vec<Protein> = items.into_iter().map(|e| e.into()).collect();
    tracing::info!("get_proteins");
    Ok(ret)
}

#[cfg(feature = "server")]
impl Into<Protein> for ProteinQuery {
    fn into(self) -> Protein {
        Protein {
            id: self.id,
            entry: self.entry,
            entry_name: self.entry_name,
            frm: self.frm,
        }
    }
}
