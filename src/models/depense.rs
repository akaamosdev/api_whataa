use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow,Serialize)]
pub struct Depense {
    pub id: String,
    pub code: String,
    pub type_depense: String,
    pub caisse_id: String,
    pub mode_paiement_id: String,
    pub type_depense_id: String,
    pub caisse: String,
    pub mode_paiement: String,
    pub ref_piece: String,
    pub user: String,
    pub montant: f32,
    pub comment: String,
    pub date_depense: NaiveDate,
}

#[derive(FromRow, Deserialize,Serialize)]
pub struct DepensePayload {
    pub id: String,
    pub code: String,
    pub type_depense_id: String,
    pub caisse_id: String,
    pub mode_paiement_id: String,
    pub ref_piece: String,
    pub user_id: String,
    pub montant: f32,
    pub comment: String,
    pub date_depense: NaiveDate,
    pub is_edited: bool,
}
// paginate depense response model
#[derive(Deserialize, FromRow,Serialize)]
pub struct PaginateDepense {
    pub offset: i64,
    pub search: Option<String>,
    pub limit: i64,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
}