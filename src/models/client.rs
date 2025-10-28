use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Client{
    pub id: String,
    pub code: String,
    pub denomination: String,
    pub nb_commerce: String,
    pub nb_contribuable: String,
    pub address_phy: String,
    pub boite_postale: String,
    pub phone_fix: String,
    pub phone_mobil: String,
    pub faxe: String,
    pub address_mail: String,
    pub boutique_id: String,
    pub defaut: i64,
    pub synchronise: i8,
    pub solde:Option<f64>,
}

