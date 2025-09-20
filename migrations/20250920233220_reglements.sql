-- Add migration script here
 CREATE TABLE reglements(
      id TEXT PRIMARY KEY,
      user_id INTEGER,
      client_id TEXT,
      fournisseur_id TEXT,
      document_id TEXT,
      boutique_id TEXT,
      caisse_id TEXT,
      reglement_num TEXT,
      reglement_date TEXT,
      commentaire TEXT,
      montant REAL,
      mode_paiement_id TEXT, 
      reference TEXT, 
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise Integer,
      FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE,
      FOREIGN KEY (fournisseur_id) REFERENCES fournisseurs(id) ON DELETE CASCADE,
      FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
      FOREIGN KEY (caisse_id) REFERENCES caisses(id) ON DELETE CASCADE,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (mode_paiement_id) REFERENCES mode_paiements(id) ON DELETE CASCADE
    )
 
 