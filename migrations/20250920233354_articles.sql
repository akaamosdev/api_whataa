-- Add migration script here
CREATE TABLE IF NOT EXISTS articles(
      id TEXT PRIMARY KEY,
      compagny_id TEXT,
      code TEXT,
      code_bar TEXT,
      name TEXT,
      sous_famille_id TEXT,
      marque_id TEXT,
      unite_id TEXT,
      alert_stock INTEGER,
      is_stock INTEGER,
      stock REAL DEFAULT 0,
      image TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )