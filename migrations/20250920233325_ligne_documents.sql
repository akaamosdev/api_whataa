-- Add migration script here

 CREATE TABLE IF NOT EXISTS ligne_documents(
      id TEXT PRIMARY KEY,
      document_id TEXT,
      article_id INTEGER,
      prix_achat_ttc REAL,
      qte REAL,
      qte_mvt_stock REAL,
      prix_vente_ttc REAL,
      taux_remise REAL,
      montant_ttc REAL,
      montant_remise REAL,
      montant_net REAL,
      prix_vente_standard REAL,
      prix_achat_standard REAL,
      achever INTEGER,
      qte_last_stock REAL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise Integer,
      FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
      FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
    );
CREATE INDEX IF NOT EXISTS idx_ligne_documents_document_id ON ligne_documents(document_id);
CREATE INDEX IF NOT EXISTS idx_ligne_documents_article_id ON ligne_documents(article_id);