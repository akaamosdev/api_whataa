

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Caisse{
    pub id: String,
    pub code: String,
    pub date_depense: String,
    pub comment: String,
    pub type_depense_id: String,
    pub user_id: String,
    pub caisse_id: String,
    pub montant: f64,
    pub mode_paiement_id: String,
    pub ref_piece: String,
    pub create_at: String
}

