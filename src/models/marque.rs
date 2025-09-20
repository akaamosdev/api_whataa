

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Marque{
    pub id: String,
    pub code: String,
    pub name: String,
    pub compagny_id: String,
    pub synchronise: bool,
    pub create_at: String
}

