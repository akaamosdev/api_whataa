use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub role_id: i64,
    pub boutique_id: String,
    pub email: String,
    pub password_hash: String,
}