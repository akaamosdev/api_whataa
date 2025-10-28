-- Add migration script here
  CREATE TABLE IF NOT EXISTS caisses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      statut INTEGER,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    );
CREATE INDEX IF NOT EXISTS idx_caisses_boutique_id ON caisses(boutique_id);