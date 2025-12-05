use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::reglement::Reglement;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct DocumentDto {
    pub id: String,
    pub user_id: String,
    pub tier_id: String,
    pub document_num: String,
    pub type_doc: i32,
    pub montant_net: f64,
    pub montant_ht: f64,
    pub taux_remise: f64,
    pub montant_remise: f64,
    pub montant_airsi: Option<f64>,
    pub montant_tva: f64,
    pub document_date: String,
    pub depot_id: String,
    pub boutique_id: String,
    pub commentaire: String,
    pub montant_client: f64,
    pub montant_total: f64,
    pub lignes: Vec<LigneDocumentDto>, // on ajoute les lignes directement
    pub reglement: Option<Reglement>,
    pub is_edit: Option<bool>,
    pub doc_parent_id: Option<String>
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct LigneDocumentDto {
    pub id: String,
    pub document_id: String,
    pub article_id: String,
    pub prix_achat_ttc: f32,
    pub prix_vente_ttc: f32,
    pub qte: f32,
    pub qte_mvt_stock: f32,
    pub montant_ttc: f32,
    pub montant_net: f32,
    pub montant_remise: f32,
}
//
