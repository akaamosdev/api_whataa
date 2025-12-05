use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Document{
    pub id: String,
    pub document_num: String,
    pub tier_id: String,
    pub document_date: String,
    pub depot_id: String,
    pub commentaire: String,
    pub type_doc: i8,
    pub montant_ht: f64,
    pub taux_remise: f64,
    pub montant_remise: f64,
    pub montant_total: f64,
    pub montant_client: f64,
    pub montant_net: f64,
    pub montant_tva: f64,
    pub montant_airsi: f64,
    pub boutique_id: String,
    pub attente: bool,
    pub regler: bool,
    pub doc_parent_id: String,
    pub doc_fils_id: String,
    pub user_id: String,
    pub mont_ht: Option<f64>,
    pub nb_commerce: Option<String>,
    pub phone_mobil: Option<String>,
    pub address_mail: Option<String>,
}
// edit Document
#[derive(Serialize,FromRow)]
pub struct DocumentEdit{
    pub id: String,
    pub type_doc: i32,
    pub document_num: String,
    pub tier_id: String,
    pub document_date: String,
    pub depot_id: String,
    pub montant_total: f32,
    pub montant_remise: f32,
    pub montant_ht: f32,
    pub montant_tva: f32,
    pub montant_net: f32,
    pub taux_remise: i32,
    pub denomination: String,
    pub address_mail: Option<String>,
    pub phone_mobil: Option<String>,
    pub phone_fix: Option<String>,
    pub doc_parent_id: Option<String>,
    pub type_tier: String,
    pub paye: Option<f32>,

}
#[derive(Serialize,FromRow)]
pub struct LigneEdit{
    pub id:String,
    pub article_id:String,
    pub code_bar:String,
    pub designation : String,
    pub quantite : f32,
    pub prix_achat_ttc : f32,
    pub prix_vente_ttc : f32,
    pub montant_remise : f32,
    pub montant_net : f32,
    pub stock : f32,
    pub unite : String,

}

