 -- Add migration script here
CREATE TABLE IF NOT EXISTS depenses(
      id TEXT PRIMARY KEY,
      code TEXT,
      date_depense DATE,
      comment TEXT,
      type_depense_id TEXT,
      user_id TEXT,
      caisse_id TEXT,
      montant REAL,
      mode_paiement_id TEXT,
      ref_piece TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_depenses_type_depense_id ON depenses(type_depense_id);
CREATE INDEX IF NOT EXISTS idx_depenses_user_id ON depenses(user_id);
CREATE INDEX IF NOT EXISTS idx_depenses_caisse_id ON depenses(caisse_id);
CREATE INDEX IF NOT EXISTS idx_depenses_mode_paiement_id ON depenses(mode_paiement_id);