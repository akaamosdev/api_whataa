-- Add migration script here
CREATE TABLE IF NOT EXISTS boutiques(
      id TEXT PRIMARY KEY,
      compagny_id TEXT,
      code TEXT,
      name TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_boutiques_compagny_id ON boutiques(compagny_id);
-- Add migration script here