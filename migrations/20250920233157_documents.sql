-- Add migration script here
CREATE TABLE IF NOT EXISTS documents(
      id TEXT PRIMARY KEY,
      document_num TEXT,
      fournisseur_id TEXT DEFAULT NULL,
      client_id TEXT DEFAULT NULL,
      document_date TEXT,
      depot_id TEXT,
      commentaire TEXT DEFAULT NULL,
      type_doc INTEGER,
      nombre_article REAL DEFAULT 0,
      montant_ttc REAL DEFAULT 0,
      taux_remise REAL DEFAULT NULL,
      montant_remise REAL DEFAULT NULL,
      montant_client REAL DEFAULT NULL,
      montant_net REAL DEFAULT 0,
      montant_tva REAL DEFAULT NULL,
      montant_airsi REAL DEFAULT NULL,
      boutique_id TEXT,
      attente INTEGER DEFAULT NULL,
      regler INTEGER DEFAULT NULL, 
      doc_parent_id TEXT,
      doc_fils_id TEXT,
      user_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     synchronise BOOLEAN DEFAULT 0,
      FOREIGN KEY (fournisseur_id) REFERENCES fournisseurs(id) ON DELETE SET NULL,
      FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL,
      FOREIGN KEY (depot_id) REFERENCES depots(id) ON DELETE SET NULL,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
    );

CREATE INDEX IF NOT EXISTS idx_documents_fournisseur_id ON documents(fournisseur_id);
CREATE INDEX IF NOT EXISTS idx_documents_client_id ON documents(client_id);
CREATE INDEX IF NOT EXISTS idx_documents_depot_id ON documents(depot_id);
CREATE INDEX IF NOT EXISTS idx_documents_boutique_id ON documents(boutique_id);
CREATE INDEX IF NOT EXISTS idx_documents_user_id ON documents(user_id);