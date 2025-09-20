

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct SousFamille{
    pub id:String,
    pub code:String,
    pub name: String,
    pub famille_id: String,
    pub synchronise: bool,
    pub create_at: String
}

