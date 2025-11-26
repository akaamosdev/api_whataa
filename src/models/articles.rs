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
    pub alert_stock: i32,
    pub is_stock: i32,
    pub boutique_id: String,
    pub price_buy: f32,
    pub price_seller: f32,
    pub stock: f32,
    pub synchronise: bool,
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
    pub alert_stock: i64,
    pub is_stock: i8,
    pub boutique_id: String,
    pub price_buy: f64,
    pub price_seller: f32,
    pub stock: f32,
    pub image: String,
    pub famille_id: String,
    pub famille: String,
    pub stock_mvt: f32,
}
