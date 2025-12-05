use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Article{
    pub id: String,
    pub code: String,
    pub code_bar: String,
    pub name: String,
    pub sous_famille_id: String,
    pub marque_id: String,
    pub unite_id: String,
    pub alert_stock: f32,
    pub is_stock: bool,
    pub boutique_id: String,
    pub price_buy: f32,
    pub price_seller: f32,
    pub stock: f32,
    pub doc_defaut_id: String,
    pub depot_defaut_id: Option<String>,
    pub user_id: Option<String>,
}

//show list
#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ArticleShow{
    pub id: String,
    pub code: String,
    pub code_bar: String,
    pub name: String,
    pub sous_famille_id: String,
    pub marque_id: String,
    pub unite_id: String,
    pub alert_stock: f32,
    pub is_stock: bool,
    pub boutique_id: String,
    pub price_buy: f32,
    pub price_seller: f32,
    pub stock: f32,
    pub image: Option<String>,
    pub famille_id: String,
    pub famille: String
}

// for pagination
#[derive(Debug,Deserialize,FromRow,Serialize)]
pub struct ArticleDocument{
    pub id: String,
    pub code: String,
    pub code_bar: String,
    pub name: String,
    pub price_seller: f32,
    pub price_buy: f32,
    pub stock: f32,
}