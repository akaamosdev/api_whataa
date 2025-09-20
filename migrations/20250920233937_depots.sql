-- Add migration script here
CREATE TABLE IF NOT EXISTS depots(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      defaut INTEGER,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )