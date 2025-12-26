use serde::{Serialize, Deserialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub role_id: i32,
    pub boutique_id: String,
    pub email: String,
    pub password_hash: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserLogin {
   pub id: String,
    pub name: String,
    pub role_id: i32,
    pub boutique_id: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserShow {
    pub id: Option<String>,
    pub name: Option<String>,
    pub role_id: Option<i64>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Roles {
    pub id: Option<i64>,
    pub name: Option<String>
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserPlayload {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub role_id: i32,
    pub boutique_id: String,
    pub password: String,
    pub new_role : Option<String>,
    pub privileges : Option<Vec<i32>>,
    pub id_edit : String,

}