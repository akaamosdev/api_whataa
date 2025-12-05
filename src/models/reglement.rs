use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::reglement_document::ReglementDocument;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Reglement{
    pub id: String,
    pub user_id: String,
    pub tier_id: String,
    pub boutique_id: String,
    pub caisse_id: String,
    pub reglement_num: String,
    pub reglement_date: String,
    pub commentaire: Option<String>,
    pub montant: f64,
    pub mode_paiement_id: String,
    pub reference: Option<String>,
    pub regle_doc: Option<ReglementDocument>
}

