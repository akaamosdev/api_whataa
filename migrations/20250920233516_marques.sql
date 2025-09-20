-- Add migration script here
 CREATE TABLE IF NOT EXISTS marques(
      id TEXT PRIMARY KEY,
      code TEXT,
      compagny_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      name TEXT,
      synchronise INTEGER
    )