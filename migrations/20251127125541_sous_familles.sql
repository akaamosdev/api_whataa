-- Add migration script here
CREATE TABLE IF NOT EXISTS sous_familles(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      famille_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
synchronise boolean DEFAULT FALSE
    );

CREATE INDEX IF NOT EXISTS idx_sous_familles_famille_id ON sous_familles(famille_id);