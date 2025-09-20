-- Add migration script here
  CREATE TABLE IF NOT EXISTS caisses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      statut INTEGER,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )