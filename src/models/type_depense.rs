

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct TypeDepense{
    pub id: String,
    pub code: String,
    pub name: String,
    pub boutique_id: String,
    pub synchronise: bool,
    pub create_at: String
}

