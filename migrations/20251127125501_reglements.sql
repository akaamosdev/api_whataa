-- Add migration script here
CREATE TABLE reglements(
      id TEXT PRIMARY KEY,
      user_id TEXT,
      tier_id TEXT,
      boutique_id TEXT,
      caisse_id TEXT,
      reglement_num TEXT,
      reglement_date DATE,
      commentaire TEXT,
      montant REAL,
      mode_paiement_id TEXT, 
      reference TEXT, 
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE,
      FOREIGN KEY (tier_id) REFERENCES tiers(id) ON DELETE CASCADE,
      FOREIGN KEY (caisse_id) REFERENCES caisses(id) ON DELETE CASCADE,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (mode_paiement_id) REFERENCES mode_paiements(id) ON DELETE CASCADE
    );
    
CREATE INDEX IF NOT EXISTS idx_reglements_tier_id ON reglements(tier_id);
CREATE INDEX IF NOT EXISTS idx_reglements_caisse_id ON reglements(caisse_id);
CREATE INDEX IF NOT EXISTS idx_reglements_boutique_id ON reglements(boutique_id);
CREATE INDEX IF NOT EXISTS idx_reglements_mode_paiement_id ON reglements(mode_paiement_id);
CREATE INDEX IF NOT EXISTS idx_reglements_user_id ON reglements(user_id);