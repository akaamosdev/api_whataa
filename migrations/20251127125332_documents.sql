-- Add migration script here
CREATE TABLE IF NOT EXISTS documents(
      id TEXT PRIMARY KEY,
      document_num TEXT,
      tier_id TEXT DEFAULT NULL,
      document_date TEXT,
      depot_id TEXT,
      commentaire TEXT DEFAULT NULL,
      type_doc INTEGER,
      montant_ht REAL DEFAULT 0,
      taux_remise INTEGER DEFAULT 0,
      montant_remise REAL DEFAULT 0,
      montant_client REAL DEFAULT 0,
      montant_total REAL DEFAULT 0,--ht+remise
      montant_net REAL DEFAULT 0,
      montant_tva REAL DEFAULT NULL,
      montant_airsi REAL DEFAULT NULL,
      boutique_id TEXT,
      attente BOOLEAN DEFAULT FALSE,
      regler BOOLEAN DEFAULT FALSE, 
      doc_parent_id TEXT,
      doc_fils_id TEXT,
      user_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise BOOLEAN DEFAULT false,
      FOREIGN KEY (tier_id) REFERENCES tiers(id) ON DELETE SET NULL,
      FOREIGN KEY (depot_id) REFERENCES depots(id) ON DELETE SET NULL,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
    );

CREATE INDEX IF NOT EXISTS idx_documents_tier_id ON documents(tier_id);
CREATE INDEX IF NOT EXISTS idx_documents_depot_id ON documents(depot_id);
CREATE INDEX IF NOT EXISTS idx_documents_boutique_id ON documents(boutique_id);
CREATE INDEX IF NOT EXISTS idx_documents_user_id ON documents(user_id);