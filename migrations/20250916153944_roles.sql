-- Add migration script here
CREATE TABLE roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,           -- ex: admin, manager, user
    description TEXT,                    -- optionnel, ex: "Administrateur avec tous les droits"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

