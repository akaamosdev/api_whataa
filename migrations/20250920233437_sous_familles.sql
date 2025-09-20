-- Add migration script here
CREATE TABLE IF NOT EXISTS sous_familles(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      famille_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )