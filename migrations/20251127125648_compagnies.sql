-- Add migration script here
CREATE TABLE IF NOT EXISTS compagnies(
      id TEXT PRIMARY KEY,
      denomination TEXT,
      cigle TEXT,
      date_created TEXT,
      capital_so TEXT DEFAULT 'FCFA',
      statut_juridique_id INTEGER,
      nb_contribuable TEXT,
      nb_commerce TEXT, 
      secteur_act TEXT,
      responsable TEXT, 
      address_phy TEXT,
      phone_fix TEXT,
      phone_mobil TEXT, 
      taux_tva INTEGER,
      taux_airsi INTEGER,
      address_mail TEXT,
      logo TEXT,
      sale_negative INTEGER,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );