

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Depot{
    pub id: String,
    pub code: String,
    pub name: String,
    pub boutique_id: String,
    pub defaut: bool,
    pub synchronise: bool,
    pub create_at: String
}

