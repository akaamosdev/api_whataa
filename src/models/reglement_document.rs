

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Caisse{
    pub id: String,
    pub reglement_id: String,
    pub document_id: String,
    pub montant: f64,
    pub synchronise: bool,
    pub create_at: String
}

