-- Add migration script here
CREATE TABLE IF NOT EXISTS mode_paiements(
      id TEXT PRIMARY KEY,
      name TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );