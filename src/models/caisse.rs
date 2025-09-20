

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Caisse{
    pub id: String,
    pub code: String,
    pub name: String,
    pub boutique_id: String,
    pub statut: i64,
    pub synchronise: bool,
    pub create_at: String
}

