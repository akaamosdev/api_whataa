    -- Add migration script here
CREATE TABLE IF NOT EXISTS reglement_documents(
      id TEXT PRIMARY KEY,
      reglement_id TEXT,
      document_id TEXT,
      montant REAL,
     created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );