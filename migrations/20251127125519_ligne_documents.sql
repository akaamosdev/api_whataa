-- Add migration script here

 CREATE TABLE IF NOT EXISTS ligne_documents(
      id TEXT PRIMARY KEY,
      document_id TEXT,
      article_id TEXT,
      prix_achat_ttc REAL,
      qte REAL,
      qte_mvt_stock REAL DEFAULT 0,
      prix_vente_ttc REAL,
      montant_ttc REAL,
      montant_remise REAL DEFAULT 0,
      montant_net REAL,
      qte_last_stock REAL DEFAULT 0,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE,
      FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
      FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
    );
CREATE INDEX IF NOT EXISTS idx_ligne_documents_document_id ON ligne_documents(document_id);
CREATE INDEX IF NOT EXISTS idx_ligne_documents_article_id ON ligne_documents(article_id);