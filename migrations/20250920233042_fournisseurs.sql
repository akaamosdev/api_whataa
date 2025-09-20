-- Add migration script here
   CREATE TABLE IF NOT EXISTS fournisseurs(
      id TEXT PRIMARY KEY,
      code TEXT,
      denomination TEXT,
      nb_commerce TEXT,
      nb_contribuable TEXT,
      address_phy TEXT,
      boite_postale TEXT,
      phone_fix TEXT,
      phone_mobil TEXT,
      faxe TEXT,
      address_mail TEXT,
      boutique_id INTEGER,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise Integer
    )