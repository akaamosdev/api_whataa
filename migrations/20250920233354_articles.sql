-- Add migration script here
CREATE TABLE IF NOT EXISTS articles(
      id TEXT PRIMARY KEY,
      code TEXT,
      code_bar TEXT,
      name TEXT,
      sous_famille_id TEXT,
      marque_id TEXT,
      unite_id TEXT,
      alert_stock INTEGER,
      is_stock INTEGER,
      image TEXT,
      boutique_id TEXT,
      price_buy REAL,
      price_seller REAL,
      stock REAL DEFAULT 0,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    );
CREATE INDEX IF NOT EXISTS idx_articles_sous_famille_id ON articles(sous_famille_id);
CREATE INDEX IF NOT EXISTS idx_articles_marque_id ON articles(marque_id);
CREATE INDEX IF NOT EXISTS idx_articles_unite_id ON articles(unite_id);
CREATE INDEX IF NOT EXISTS idx_articles_boutique_id ON articles(boutique_id);