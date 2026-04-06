use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Caisse{
    pub id: String,
    pub code: String,
    pub name: String,
    pub boutique_id: String,
    pub solde_caisse: Option<f32>,
}
#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct MvtCaissePlayload{
    pub caisse_debit_id: String,
    pub caisse_credit_id: String,
    pub montant: f32,
    pub comment: String,
    pub user_id: String,
    pub boutique_id: String,
}
#[derive(FromRow,Serialize)]
pub struct MvtCaisseShow{
    pub numero_mvt: String,
    pub caisse: String,
    pub montant: f32,
    pub commentaire: String,
    pub date_mvt: NaiveDate,
    pub type_mvt: String,
    pub user_by: String,
}
#[derive(Debug,Deserialize)]
pub struct PaginateCaisse {
    pub offset: i64,
    pub caisse_id: Option<String>,
    pub limit: i64,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
}