use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ModePaiement{
    pub id: String,
    pub name: String,
    pub compagny_id: String,
    pub synchronise: bool,
    pub create_at: String
}
#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ModePaiementShow{
    pub id: String,
    pub name: String,
}

