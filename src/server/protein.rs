#[cfg(feature = "server")]
use crate::auth::Session;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    println!("{stmt}");

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
