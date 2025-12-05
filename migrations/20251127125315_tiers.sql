-- Add migration script here
CREATE TABLE IF NOT EXISTS tiers(
      id TEXT PRIMARY KEY,
      code TEXT,
      denomination TEXT,
      type_tier TEXT,
      nb_commerce TEXT,
      nb_contribuable TEXT,
      address_phy TEXT,
      boite_postale TEXT,
      phone_fix TEXT,
      phone_mobil TEXT,
      address_mail TEXT,
      boutique_id TEXT,
      defaut BOOLEAN DEFAULT FALSE,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );

CREATE INDEX IF NOT EXISTS idx_tiers_boutique_id ON tiers(boutique_id);
CREATE INDEX IF NOT EXISTS idx_tiers_type_tier ON tiers (type_tier);
CREATE INDEX IF NOT EXISTS idx_tiers_denomination ON tiers (denomination);
