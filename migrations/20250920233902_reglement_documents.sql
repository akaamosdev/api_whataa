-- Add migration script here
CREATE TABLE IF NOT EXISTS reglement_documents(
      id TEXT PRIMARY KEY,
      reglement_id TEXT,
      document_id TEXT,
      montant TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )