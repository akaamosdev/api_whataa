-- Add migration script here
  CREATE TABLE IF NOT EXISTS caisses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      statut boolean DEFAULT TRUE,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_caisses_boutique_id ON caisses(boutique_id);