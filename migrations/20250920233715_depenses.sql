-- Add migration script here
CREATE TABLE IF NOT EXISTS depenses(
      id TEXT PRIMARY KEY,
      code TEXT,
      date_depense TEXT,
      comment TEXT,
      type_depense_id TEXT,
      user_id TEXT,
      caisse_id TEXT,
      montant TEXT,
      mode_paiement_id TEXT,
      ref_piece TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      synchronise INTEGER
    )