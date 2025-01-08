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
    pub frm: String,
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
    pub frm: String,
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

#[server(GetProteins)]
pub async fn get_proteins(offset: i32) -> Result<Vec<Protein>, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();
    if !curr_user.permissions.contains("Category::View") {
        return Ok(vec![]);
    }

    let dbc = session.1;
    let items: Vec<ProteinQuery> = sqlx::query_as::<_, ProteinQuery>(
        "SELECT * FROM ac WHERE entry IN ('O14594', 'P84074', 'Q16650', 'Q9BQI7', 'P23515', 'P01160', 'P19429', 'Q01449', 'P45379', 'P20813', 'P54840', 'Q14032', 'P33261', 'P08684', 'P07988', 'P11686', 'Q8IWL2', 'Q8IWL1', 'Q9UBF9', 'A8MU46', 'Q13698', 'P02585', 'P01308', 'Q0VAF6', 'P48304', 'Q6GPI1', 'P04746', 'P16233', 'O60829', 'P07288', 'P15309', 'P04279', 'P04155', 'Q03403', 'Q3MIW9', 'Q86XP6', 'P07098', 'P0DJD9', 'P27352', 'Q9NS71', 'Q6PHW0', 'P01266', 'P07202', 'P16473', 'O00476', 'O15244', 'Q9NP85', 'Q8WZ55', 'Q96S37') ORDER BY id DESC LIMIT 25 OFFSET $1;", //  OFFSET {offset}
    )
    .bind(offset)
    .fetch_all(dbc.as_ref())
    .await?;
    let ret: Vec<Protein> = items.into_iter().map(|e| e.into()).collect();
    info!("{}", ret.len());
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
impl Into<QM> for QmQuery {
    fn into(self) -> QM {
        QM {
            tissue: self.tissue,
            value: self.value,
        }
    }
}

#[server(GetProteinQ)]
pub async fn get_protein_q(id: i32) -> Result<Vec<QM>, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();
    if !curr_user.permissions.contains("Category::View") {
        return Ok(vec![]);
    }

    let dbc = session.1;
    let items: Vec<QmQuery> = sqlx::query_as::<_, QmQuery>(
        "SELECT global_grp as tissue,norm_value as value FROM qm INNER JOIN qmatrix ON qmatrix.id = qmatrix_id INNER JOIN ann ON ann_id = ann.id WHERE ac_id = $1 AND qmatrix.name = 'haatlas';"
    )
    .bind(id)
    .fetch_all(dbc.as_ref())
    .await?;
    let ret: Vec<QM> = items.into_iter().map(|e| e.into()).collect();
    info!("{}", ret.len());
    Ok(ret)
}
