-- Add migration script here
 CREATE TABLE IF NOT EXISTS users(
      id TEXT PRIMARY KEY,
      name TEXT,
      phone TEXT,
      email TEXT,
      password TEXT,
      password_hash TEXT,
      boutique_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      role_id INTEGER,
      statut INTEGER,
      depot_id TEXT DEFAULT "",
      synchronise INTEGER
    );

CREATE INDEX IF NOT EXISTS idx_users_boutique_id ON users(boutique_id);
CREATE INDEX IF NOT EXISTS idx_users_depot_id ON users(depot_id);