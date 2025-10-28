use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Document{
    pub id: String,
    pub document_num: String,
    pub fournisseur_id: String,
    pub client_id: String,
    pub document_date: String,
    pub depot_id: String,
    pub commentaire: String,
    pub type_doc: i8,
    pub nombre_article: f64,
    pub montant_ttc: f64,
    pub taux_remise: f64,
    pub montant_remise: f64,
    pub montant_client: f64,
    pub montant_net: f64,
    pub montant_tva: f64,
    pub montant_airsi: f64,
    pub boutique_id: String,
    pub attente: i8,
    pub regler: i8,
    pub doc_parent_id: String,
    pub doc_fils_id: String,
    pub user_id: String,
    pub mont_ht: Option<f64>,
    pub nb_commerce: Option<String>,
    pub phone_mobil: Option<String>,
    pub address_mail: Option<String>,
}

