use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::reglement::Reglement;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct DocumentDto {
    pub id: String,
    pub user_id: String,
    pub document_num: String,
    pub type_doc: i32,
    pub nombre_article: f64,
    pub montant_net: f64,
    pub montant_ttc: f64,
    pub taux_remise: f64,
    pub montant_remise: f64,
    pub montant_airsi: f64,
    pub montant_tva: f64,
    pub fournisseur_id: Option<String>,
    pub client_id: Option<String>,
    pub document_date: String,
    pub depot_id: String,
    pub boutique_id: String,
    pub commentaire: String,
    pub montant_client: f64,
    pub synchronise: i32,
    pub ligs: Vec<LigneDocumentDto>, // on ajoute les lignes directement
    pub reglement: Option<Reglement>,
    pub is_edit: Option<bool>
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
    pub taux_remise: f32,
    pub montant_ttc: f32,
    pub montant_net: f32,
    pub montant_remise: f32,
    pub achever: i32,
    pub synchronise: bool,
}
//
#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct LigneDocumentShow {
    pub id: String,
    pub document_id: String,
    pub article_id: String,
    pub prix_achat_ttc: f64,
    pub prix_vente_ttc: f64,
    pub qte: f64,
    pub qte_mvt_stock: f64,
    pub taux_remise: f64,
    pub montant_ttc: f64,
    pub montant_net: f64,
    pub montant_remise: f64,
    pub code: String,
    pub name: String,
    pub code_bar: String,
    pub is_stock: i8,
    pub stock_mvt: f64,
}