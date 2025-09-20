

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ArticleBoutique{
    pub id: String,
    pub article_id: String,
    pub boutique_id: String,
    pub price_buy: f64,
    pub price_seller: f64,
    pub synchronise: bool,
    pub create_at: String
}

