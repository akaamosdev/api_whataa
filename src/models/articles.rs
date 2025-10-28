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
    pub alert_stock: f64,
    pub is_stock: i8,
    pub boutique_id: String,
    pub price_buy: f64,
    pub price_seller: f64,
    pub stock: f64,
    pub synchronise: i8,
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
    pub alert_stock: u16,
    pub is_stock: i8,
    pub boutique_id: String,
    pub price_buy: f64,
    pub price_seller: f64,
    pub stock: f64,
    pub image: String,
    pub famille_id: String,
    pub famille: String,
    pub stock_mvt: f64,
}
