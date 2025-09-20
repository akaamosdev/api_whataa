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
    pub is_stock: bool,
    pub stock: f64,
    pub image: String,
    pub synchronise: bool,
    pub create_at: String
}

