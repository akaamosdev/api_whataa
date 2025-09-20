-- Add migration script here
CREATE TABLE IF NOT EXISTS boutiques(
      id TEXT PRIMARY KEY,
      compagny_id TEXT,
      code TEXT,
      name TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )