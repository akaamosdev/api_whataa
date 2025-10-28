use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Compagny{
    pub id: String,
    pub denomination: String,
    pub cigle: String,
    pub date_created: String,
    pub capital_so: String,
    pub statut_juridique_id: String,
    pub nb_contribuable: String,
    pub nb_commerce: String,
    pub secteur_act: String,
    pub responsable: String,
    pub address_phy: String,
    pub phone_fix: String,
    pub phone_mobil: String,
    pub taux_tva: String,
    pub taux_airsi: String,
    pub address_mail: String,
    pub logo: String,
    pub sale_negative: String,
    pub synchronise: i8,
}

