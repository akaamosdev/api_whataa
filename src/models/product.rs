use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product{
    pub id: String,
    pub code: String,
    pub code_bar: String,
    pub name: String,
    pub sousFamille: String,
}