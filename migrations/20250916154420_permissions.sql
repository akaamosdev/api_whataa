-- Add migration script here
CREATE TABLE permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,           -- ex: create_user, delete_invoice
    description TEXT,                    -- optionnel, pour décrire l’action
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);