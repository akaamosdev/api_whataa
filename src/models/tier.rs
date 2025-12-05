use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Tier{
    pub id: String,
    pub type_tier: String,
    pub code: String,
    pub denomination: String,
    pub nb_commerce: Option<String>,
    pub nb_contribuable: Option<String>,
    pub address_phy: Option<String>,
    pub boite_postale: Option<String>,
    pub phone_fix: Option<String>,
    pub phone_mobil: Option<String>,
    pub address_mail: Option<String>,
    pub boutique_id: String,
    pub defaut: Option<bool>,
    pub synchronise: Option<bool>,
    pub solde:Option<f32>,
}

