-- Add migration script here
CREATE TABLE IF NOT EXISTS abonnements(
      id SERIAL PRIMARY KEY,
      key_licence TEXT,
      email_compagny TEXT,
      phone_compagny TEXT,
      nom_compagny TEXT,
      date_debut TIMESTAMPTZ,
      date_fin TIMESTAMPTZ
    );