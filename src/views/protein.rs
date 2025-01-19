//use dsrv::server::functions::{get_permissions, get_user_name, login, logout};
//use crate::server::{get_protein, get_protein_q, get_proteins, Protein, ProteinDetail, QM};
use crate::routes::USR;
use crate::Route;
use dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
#[cfg(feature = "server")]
use dsrv::auth::Session;
use serde::{Deserialize, Serialize};
use std::fmt;

use charming::{
    component::{Axis, Title},
    element::{AxisLabel, AxisType},
    series::Bar,
    Chart, WasmRenderer,
};

pub fn ProteinList() -> Element {
    let mut items: Signal<Vec<Protein>> = use_signal(std::vec::Vec::new);
    let mut searchterm: Signal<String> = use_signal(String::new);

    let data =
        use_resource(move || get_proteins(items.read().len() as i32, searchterm.read().clone()));

    let ren = match &*data.read_unchecked() {
        Some(Ok(dtas)) => rsx! {
            div { class: "bg-white rounded-md w-full min-w-44",
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
    let se = rsx! {

        div { class: "p-4",
            input {
                placeholder: "Search",
                oninput: move |event| async move {
                    let data = event.value();
                    searchterm.set(data);
                },
            }
        }
    };
    let c1 = rsx! {
        button {
            class: "inline-flex items-center gap-x-1.5 rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
            onclick: move |_| async move {
                if let Ok(data) = get_proteins(items.len() as i32, searchterm.read().clone())
                    .await
                {
                    items.extend(data);
                }
            },
            "Load more data"
        }
    };
    rsx! {
        div { class: "bg-gray-100",
            {se}
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
        div {
            key: "prk:{props.protein.id}",
            class: "flex min-w-20 w-full shadow-sm pb-4 m-2",
            id: "pri:{props.protein.id}",
            div { class: "min-w-0 flex-auto",
                p { class: "text-black text-lg font-bold",
                    "{props.protein.entry} - {props.protein.entry_name}"
                }
                p { class: "text-black text-sm",
                    "{props.protein.symbol} ({props.protein.ensg}): {props.protein.name}"
                }
            }
            div { class: "hidden shrink-0 sm:flex sm:flex-col sm:items-end overflow-x-clip",
                p { class: "text-black text-sm", "gls{props.protein.gls}: {props.protein.label}" }
                Link {
                    key: "protein{props.protein.id}",
                    class: "inline-flex items-center gap-x-1.5 rounded-md bg-indigo-600 px-2.5 py-1.5 my-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    id: "proteinid{props.protein.id}",
                    to: Route::ProteinDetails {
                        id: props.protein.id,
                    },

                    "View details"
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

    let details = use_resource(move || get_protein(*iid.read() as i32));

    let d_ren = match &*details.read_unchecked() {
        Some(Ok(details)) => rsx! {
            div {
                table {
                    tr {
                        th { class: "p-1", "AC" }
                        td { class: "p-1",
                            p {
                                {details.entry.clone()}
                                " - "
                                {details.entry_name.clone()}
                            }
                        }
                    }
                    tr {
                        th { class: "p-1", "Gene" }
                        td { class: "p-1", {details.ensg.clone()} }
                    }
                    tr {
                        th { class: "p-1", "Symbol" }
                        td { class: "p-1", {details.symbol.clone()} }
                    }
                    tr {
                        th { class: "p-1", "Name" }
                        td { class: "p-1", {details.name.clone()} }
                    }
                    tr {
                        th { class: "p-1", "BioType" }
                        td { class: "p-1", {details.biotype.clone()} }
                    }
                    tr {
                        th { class: "p-1", "Label" }
                        td { class: "p-1", {details.label.clone()} }
                    }
                    tr {
                        th { class: "p-1", "GLS" }
                        td { class: "p-1", {details.gls.clone()} }
                    }
                }
            }
        },
        Some(Err(err)) => rsx! { "An error occurred while fetching the data: {err}" },
        None => rsx! {
            p { class: "p-4", "Loading items" }
        },
    };

    let t_ren = match &*details.read_unchecked() {
        Some(Ok(details)) => rsx! {
            div {
                h3 {

                    {details.entry_name.clone()}
                    " ("
                    {details.entry.clone()}
                    "): "
                    {details.name.clone()}
                }
            }
        },
        Some(Err(err)) => rsx! { "An error occurred while fetching the data: {err}" },
        None => rsx! {
            p { class: "p-4", "Loading items" }
        },
    };

    // FIXME: refactor graphs to components
    let ha_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "haatlas".into()).await.unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("HA Tissue"));
        renderer.render("ha_chart", &chart)
    });
    let ha_status = match &*ha_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    let hc_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "hacells".into()).await.unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("HA Cells"));
        renderer.render("hc_chart", &chart)
    });
    let hc_status = match &*hc_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    let ec_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "emblcells".into())
            .await
            .unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("EMBL Cells"));
        renderer.render("ec_chart", &chart)
    });
    let ec_status = match &*ec_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    let ea_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "emblatlas".into())
            .await
            .unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("EMBL Tissue"));
        renderer.render("ea_chart", &chart)
    });
    let ea_status = match &*ea_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    let pa_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "mspatlas".into()).await.unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("MSP Tissue"));
        renderer.render("pa_chart", &chart)
    });
    let pa_status = match &*pa_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    let ra_rslt = use_resource(move || async move {
        let mut x = vec![];
        let mut y = vec![];
        let ds = get_protein_q(*iid.read(), "msratlas".into()).await.unwrap();
        for d in ds {
            x.push(d.tissue.clone());
            y.push(d.value);
        }
        let renderer: WasmRenderer = match x.len() {
            0 => WasmRenderer::new(0, 0),
            _ => WasmRenderer::new(600, 400),
        };
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(x)
                    .axis_label(AxisLabel::new().rotate(50).interval(0)),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Bar::new().data(y))
            .title(Title::new().text("MSR Tissue"));
        renderer.render("ra_chart", &chart)
    });
    let ra_status = match &*ra_rslt.read_unchecked() {
        Some(Ok(o)) => {
            rsx! {}
        }
        Some(Err(e)) => {
            rsx! {
                p { "err: {e}" }
            }
        }
        _ => {
            rsx! {
                p { "loading..." }
            }
        }
    };
    rsx! {
        div { class: "divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow",
            div { class: "px-4 py-5 sm:px-6", {t_ren} }
            div { class: "px-8 py-8 sm:p-6 m-4", {d_ren} }
            div { id: "ha_chart", style: "display: inline-block;" }
            div { id: "ea_chart", style: "display: inline-block;" }
            div { id: "pa_chart", style: "display: inline-block;" }
            div { id: "ra_chart", style: "display: inline-block;" }
            div { id: "hc_chart", style: "display: inline-block;" }
            div { id: "ec_chart", style: "display: inline-block;" }
            {ha_status}
            {ea_status}
            {pa_status}
            {ra_status}
            {hc_status}
            {ec_status}
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct Protein {
    pub id: i32,
    pub entry: String,
    pub entry_name: String,
    pub ensg: String,
    pub symbol: String,
    pub name: String,
    pub biotype: String,
    pub gls: String,
    pub label: String,
    pub rank: f64,
}

impl fmt::Display for Protein {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.entry,)
    }
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, sqlx::FromRow)]
pub struct ProteinQuery {
    pub id: i32,
    pub entry: String,
    pub entry_name: String,
    pub ensg: String,
    pub symbol: String,
    pub name: String,
    pub biotype: String,
    pub gls: String,
    pub label: String,
    pub rank: f64,
}

#[cfg(feature = "server")]
impl From<ProteinQuery> for Protein {
    fn from(val: ProteinQuery) -> Self {
        Protein {
            id: val.id,
            entry: val.entry,
            entry_name: val.entry_name,
            ensg: val.ensg,
            symbol: val.symbol,
            name: val.name,
            biotype: val.biotype,
            gls: val.gls,
            label: val.label,
            rank: val.rank,
        }
    }
}

#[server(GetProteins)]
pub async fn get_proteins(offset: i32, search: String) -> Result<Vec<Protein>, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();
    info!("Current user: {curr_user:?}");
    if !curr_user.permissions.contains("Category::View") {
        return Ok(vec![]);
    }

    let stmt = match search.as_ref() {
        "" => String::from("SELECT ac.id,entry,entry_name,ensg,symbol,name,biotype,gls::text,global_label_string as label,0.0::float as rank FROM ac INNER JOIN target ON frm = ensg INNER JOIN gls ON ac.id = gls.ac::int WHERE entry IN ('O14594', 'P84074', 'Q16650', 'Q9BQI7', 'P23515', 'P01160', 'P19429', 'Q01449', 'P45379', 'P20813', 'P54840', 'Q14032', 'P33261', 'P08684', 'P07988', 'P11686', 'Q8IWL2', 'Q8IWL1', 'Q9UBF9', 'A8MU46', 'Q13698', 'P02585', 'P01308', 'Q0VAF6', 'P48304', 'Q6GPI1', 'P04746', 'P16233', 'O60829', 'P07288', 'P15309', 'P04279', 'P04155', 'Q03403', 'Q3MIW9', 'Q86XP6', 'P07098', 'P0DJD9', 'P27352', 'Q9NS71', 'Q6PHW0', 'P01266', 'P07202', 'P16473', 'O00476', 'O15244', 'Q9NP85', 'Q8WZ55', 'Q96S37') ORDER BY id DESC LIMIT 25 OFFSET $1;"),
        _ => {
            let parts: Vec<String> = search.split_whitespace().map(String::from).collect();
            let first = match parts.first() {
                Some(e) => e.clone(),
                None => String::new(),
            };
            format!("SELECT id,entry,entry_name,ensg,symbol,name,biotype,gls,label,ts_rank(full_text, to_tsquery('{first}:*'))::float AS rank FROM glsv WHERE full_text @@ to_tsquery('{first}:*') order by rank desc LIMIT 25 OFFSET $1;")},
    };
    info!("{stmt}");

    let dbc = session.1;
    let items: Vec<ProteinQuery> = sqlx::query_as::<_, ProteinQuery>(&stmt)
        .bind(offset)
        .fetch_all(dbc.as_ref())
        .await?;
    let ret: Vec<Protein> = items.into_iter().map(|e| e.into()).collect();
    info!("{}", ret.len());
    Ok(ret)
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct ProteinDetail {
    pub entry: String,
    pub entry_name: String,
    pub ensg: String,
    pub symbol: String,
    pub name: String,
    pub biotype: String,
    pub gls: String,
    pub label: String,
}

impl fmt::Display for ProteinDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.entry,)
    }
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, sqlx::FromRow)]
pub struct ProteinDetailQuery {
    pub entry: String,
    pub entry_name: String,
    pub ensg: String,
    pub symbol: String,
    pub name: String,
    pub biotype: String,
    pub gls: String,
    pub label: String,
}

#[cfg(feature = "server")]
impl From<ProteinDetailQuery> for ProteinDetail {
    fn from(val: ProteinDetailQuery) -> Self {
        ProteinDetail {
            entry: val.entry,
            entry_name: val.entry_name,
            ensg: val.ensg,
            symbol: val.symbol,
            name: val.name,
            biotype: val.biotype,
            gls: val.gls,
            label: val.label,
        }
    }
}

#[server(GetProtein)]
pub async fn get_protein(id: i32) -> Result<ProteinDetail, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();
    if !curr_user.permissions.contains("Category::View") {
        return Ok(ProteinDetail::default());
    }

    let dbc = session.1;
    let item: ProteinDetailQuery = sqlx::query_as::<_, ProteinDetailQuery>(
        "select entry,entry_name,ensg,symbol,name,biotype,gls::text,global_label_string as label from ac INNER JOIN target ON frm = ensg INNER JOIN gls ON ac.id = gls.ac::int WHERE ac.id = $1;"
    )
    .bind(id)
    .fetch_one(dbc.as_ref())
    .await?;
    let ret: ProteinDetail = item.into();
    Ok(ret)
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct QM {
    pub tissue: String,
    pub value: f64,
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, sqlx::FromRow)]
pub struct QmQuery {
    pub tissue: String,
    pub value: f64,
}

#[cfg(feature = "server")]
impl From<QmQuery> for QM {
    fn from(val: QmQuery) -> Self {
        QM {
            tissue: val.tissue,
            value: val.value,
        }
    }
}

#[server(GetProteinQ)]
pub async fn get_protein_q(id: i32, atlas: String) -> Result<Vec<QM>, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();
    if !curr_user.permissions.contains("Category::View") {
        return Ok(vec![]);
    }

    let dbc = session.1;
    let items: Vec<QmQuery> = sqlx::query_as::<_, QmQuery>(
        "SELECT global_grp as tissue,norm_value as value FROM qm INNER JOIN qmatrix ON qmatrix.id = qmatrix_id INNER JOIN ann ON ann_id = ann.id WHERE ac_id = $1 AND qmatrix.name = $2 ORDER BY global_grp;"
    )
    .bind(id)
    .bind(&atlas)
    .fetch_all(dbc.as_ref())
    .await?;
    let ret: Vec<QM> = items.into_iter().map(|e| e.into()).collect();
    info!("{}: {}", atlas, ret.len());
    Ok(ret)
}
