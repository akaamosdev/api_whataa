-- Add migration script here
  CREATE TABLE IF NOT EXISTS unites(
      id TEXT PRIMARY KEY,
      code TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      name TEXT,
     synchronise boolean DEFAULT FALSE
    );