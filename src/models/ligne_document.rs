

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct LigneDocument{
    pub id: String,
    pub document_id: String,
    pub article_id: String,
    pub prix_achat_ttc: f64,
    pub qte: f64,
    pub qte_mvt_stock: f64,
    pub prix_vente_ttc: f64,
    pub taux_remise: f64,
    pub montant_ttc: f64,
    pub montant_remise: f64,
    pub montant_net: f64,
    pub achever: bool,
    pub qte_last_stock: f64,
    pub synchronise: bool,
    pub create_at: String
}

