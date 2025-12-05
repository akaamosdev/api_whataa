use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct ReglementDocument{
    pub id: String,
    pub reglement_id: String,
    pub document_id: String,
    pub montant: f64,
}

