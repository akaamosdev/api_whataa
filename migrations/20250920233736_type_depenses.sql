-- Add migration script here
CREATE TABLE IF NOT EXISTS type_depenses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )