use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Compagny{
    pub id: String,
    pub denomination: String,
    pub cigle: String,
    pub date_created: String,
    pub capital_so: String,
    pub statut_juridique_id: i32,
    pub nb_contribuable: Option<String>,
    pub nb_commerce: Option<String>,
    pub secteur_act: Option<String>,
    pub responsable: Option<String>,
    pub address_phy: Option<String>,
    pub phone_fix: Option<String>,
    pub phone_mobil: String,
    pub taux_tva: i32,
    pub taux_airsi: i32,
    pub address_mail: Option<String>,
    pub logo: Option<String>,
    pub sale_negative: Option<i32>,
    pub synchronise: Option<bool>,
}

