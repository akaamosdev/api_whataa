

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Document{
    pub id: String,
    pub document_num: String,
    pub fournisseur_id: String,
    pub client_id: String,
    pub document_date: String,
    pub depot_id: String,
    pub commentaire: String,
    pub type_doc: i64,
    pub nombre_article: i64,
    pub montant_ttc: f64,
    pub taux_remise: f64,
    pub montant_remise: bool,
    pub montant_client: f64,
    pub montant_net: f64,
    pub montant_tva: f64,
    pub montant_airsi: f64,
    pub boutique_id: String,
    pub attente: bool,
    pub regler: bool,
    pub doc_parent_id: String,
    pub doc_fils_id: String,
    pub user_id: String,
    pub synchronise: bool,
    pub create_at: String
}

