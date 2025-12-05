-- Add migration script here
CREATE TABLE IF NOT EXISTS articles(
      id TEXT PRIMARY KEY,
      code TEXT,
      code_bar TEXT,
      name TEXT,
      sous_famille_id TEXT,
      marque_id TEXT,
      unite_id TEXT,
      alert_stock REAL DEFAULT 0,
      is_stock boolean DEFAULT TRUE,
      image TEXT,
      boutique_id TEXT,
      price_buy REAL,
      price_seller REAL,
      stock REAL DEFAULT 0,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_articles_sous_famille_id ON articles(sous_famille_id);
CREATE INDEX IF NOT EXISTS idx_articles_marque_id ON articles(marque_id);
CREATE INDEX IF NOT EXISTS idx_articles_unite_id ON articles(unite_id);
CREATE INDEX IF NOT EXISTS idx_articles_boutique_id ON articles(boutique_id);
CREATE INDEX IF NOT EXISTS idx_articles_code ON articles(code);
CREATE INDEX IF NOT EXISTS idx_articles_code_bar ON articles(code_bar);
CREATE INDEX IF NOT EXISTS idx_articles_name ON articles(name);