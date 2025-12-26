use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ModePaiementShow{
    pub id: String,
    pub name: String,
}

