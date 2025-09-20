

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Reglement{
    pub id: String,
    pub user_id: i64,
    pub client_id: String,
    pub fournisseur_id: String,
    pub document_id: String,
    pub boutique_id: String,
    pub caisse_id: String,
    pub reglement_num: String,
    pub reglement_date: String,
    pub commentaire: String,
    pub montant: String,
    pub mode_paiement_id: String,
    pub reference: String,
    pub synchronise: bool,
    pub create_at: String
}

