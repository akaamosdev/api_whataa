-- Add migration script here
 CREATE TABLE IF NOT EXISTS admins(
      id TEXT PRIMARY KEY,
      name TEXT,
      email TEXT,
      password TEXT,
      phone TEXT,
      compagny_id TEXT,
      synchronise INTEGER
    )