-- Add migration script here
 CREATE TABLE IF NOT EXISTS article_boutiques(
      id TEXT PRIMARY KEY,
      article_id TEXT,
      boutique_id TEXT,
      price_buy REAL,
      price_seller REAL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )