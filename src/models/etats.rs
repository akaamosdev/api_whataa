use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct ParamsEtatDoc {
    pub tier_id: Option<String>,
    pub date_start: String,
    pub date_end: String,
    pub type_doc: i32,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VenteFacture {
    pub id: Option<String>,
    pub tier_name: Option<String>,
    pub numero: Option<String>,
    pub date: Option<String>,
    pub montant_net: Option<f32>,
    pub montant_tva: Option<f32>,
    pub montant_total: Option<f32>,
    pub montant_remise: Option<f32>,
    pub lignes: Option<Value>, // json_agg
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ArticleVenteAchat {
    pub code_bar: Option<String>,
    pub designation: Option<String>,
    pub total_qte: Option<f32>,
    pub total_vente: Option<f32>,
    pub total_achat: Option<f32>,
    pub total_marge: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EtatVenteCumuled {
    pub numero: Option<String>,
    pub date: Option<String>,
    pub tier_name: Option<String>,
    pub montant_net: Option<f32>,
    pub montant_tva: Option<f32>,
    pub montant_total: Option<f32>,
    pub montant_remise: Option<f32>,
}
#[derive(Deserialize, FromRow, Serialize)]
pub struct EtatReglementTier {
    pub numero: Option<String>,
    pub date: Option<String>,
    pub tier_name: Option<String>,
    pub montant: Option<f32>,
    pub mode_paiement: Option<String>,
    pub comment: Option<String>,
}
#[derive(Deserialize, FromRow, Serialize)]
pub struct EtatCreanceTier {
    pub denomination: Option<String>,
    pub code: Option<String>,
    pub phone_mobil: Option<String>,
    pub total_vente: Option<f32>,
    pub total_regle: Option<f32>,
    pub solde: Option<f32>,
}
#[derive(Deserialize, FromRow, Serialize)]
pub struct EtatMvtTier{
    pub numero: Option<String>,
    pub date_mvt: Option<String>,
    pub type_mvt: Option<String>,
    pub montant: Option<f32>,
    pub solde: Option<f32>,
}