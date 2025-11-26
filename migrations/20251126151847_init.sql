-- Add migration script here
CREATE TABLE IF NOT EXISTS articles(
      id TEXT PRIMARY KEY,
      code TEXT,
      code_bar TEXT,
      name TEXT,
      sous_famille_id TEXT,
      marque_id TEXT,
      unite_id TEXT,
      alert_stock INTEGER,
      is_stock INTEGER,
      image TEXT,
      boutique_id TEXT,
      price_buy REAL,
      price_seller REAL,
      stock REAL DEFAULT 0,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_articles_sous_famille_id ON articles(sous_famille_id);
CREATE INDEX IF NOT EXISTS idx_articles_marque_id ON articles(marque_id);
CREATE INDEX IF NOT EXISTS idx_articles_unite_id ON articles(unite_id);
CREATE INDEX IF NOT EXISTS idx_articles_boutique_id ON articles(boutique_id);
-- Add migration script here
 CREATE TABLE IF NOT EXISTS users(
      id TEXT PRIMARY KEY,
      name TEXT,
      phone TEXT,
      email TEXT,
      password TEXT,
      password_hash TEXT,
      boutique_id TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      role_id INTEGER,
      statut INTEGER,
      depot_id TEXT DEFAULT '',
      synchronise boolean DEFAULT FALSE
    );

CREATE INDEX IF NOT EXISTS idx_users_boutique_id ON users(boutique_id);
CREATE INDEX IF NOT EXISTS idx_users_depot_id ON users(depot_id);
-- Add migration script here
-- Add migration script here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
-- Add migration script here
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,           -- ex: create_user, delete_invoice
    description TEXT,                    -- optionnel, pour décrire l’action
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Add migration script here
CREATE TABLE role_user (
    user_id TEXT NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);
-- Add migration script here
CREATE TABLE permission_role (
    role_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

---------------------add privilege
INSERT INTO roles (name, description) VALUES
('admin', 'Administrateur avec tous les droits');

INSERT INTO permissions (name, description) VALUES
('create_user', 'Créer un nouvel utilisateur'),
('view_user', 'Voir les utilisateurs'),
('edit_user', 'Modifier un utilisateur'),
('delete_user', 'Supprimer un utilisateur'),

('create_product', 'Créer un produit'),
('view_product', 'Voir les produits'),
('edit_product', 'Modifier un produit'),
('delete_product', 'Supprimer un produit');

INSERT INTO permission_role (role_id, permission_id)
SELECT 1, id FROM permissions;
-- Add migration script here
CREATE TABLE IF NOT EXISTS boutiques(
      id TEXT PRIMARY KEY,
      compagny_id TEXT,
      code TEXT,
      name TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_boutiques_compagny_id ON boutiques(compagny_id);
-- Add migration script here
CREATE TABLE IF NOT EXISTS depots(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      defaut INTEGER,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
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
      boutique_id INTEGER,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
    -- Add migration script here
CREATE TABLE IF NOT EXISTS documents(
      id TEXT PRIMARY KEY,
      document_num TEXT,
      tier_id TEXT DEFAULT NULL,
      document_date TEXT,
      depot_id TEXT,
      commentaire TEXT DEFAULT NULL,
      type_doc INTEGER,
      nombre_article REAL DEFAULT 0,
      montant_ttc REAL DEFAULT 0,
      taux_remise REAL DEFAULT NULL,
      montant_remise REAL DEFAULT NULL,
      montant_client REAL DEFAULT NULL,
      montant_net REAL DEFAULT 0,
      montant_tva REAL DEFAULT NULL,
      montant_airsi REAL DEFAULT NULL,
      boutique_id TEXT,
      attente INTEGER DEFAULT NULL,
      regler INTEGER DEFAULT NULL, 
      doc_parent_id TEXT,
      doc_fils_id TEXT,
      user_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise BOOLEAN DEFAULT false,
      FOREIGN KEY (tier_id) REFERENCES tiers(id) ON DELETE SET NULL,
      FOREIGN KEY (depot_id) REFERENCES depots(id) ON DELETE SET NULL,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
    );

CREATE INDEX IF NOT EXISTS idx_documents_tier_id ON documents(tier_id);
CREATE INDEX IF NOT EXISTS idx_documents_depot_id ON documents(depot_id);
CREATE INDEX IF NOT EXISTS idx_documents_boutique_id ON documents(boutique_id);
CREATE INDEX IF NOT EXISTS idx_documents_user_id ON documents(user_id);
-- Add migration script here
CREATE TABLE IF NOT EXISTS mode_paiements(
      id TEXT PRIMARY KEY,
      name TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
    -- Add migration script here
  CREATE TABLE IF NOT EXISTS caisses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      statut INTEGER,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_caisses_boutique_id ON caisses(boutique_id);
-- Add migration script here
 CREATE TABLE reglements(
      id TEXT PRIMARY KEY,
      user_id INTEGER,
      tier_id TEXT,
      document_id TEXT,
      boutique_id TEXT,
      caisse_id TEXT,
      reglement_num TEXT,
      reglement_date TEXT,
      commentaire TEXT,
      montant REAL,
      mode_paiement_id TEXT, 
      reference TEXT, 
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE,
      FOREIGN KEY (tier_id) REFERENCES tiers(id) ON DELETE CASCADE,
      FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
      FOREIGN KEY (caisse_id) REFERENCES caisses(id) ON DELETE CASCADE,
      FOREIGN KEY (boutique_id) REFERENCES boutiques(id) ON DELETE CASCADE,
      FOREIGN KEY (mode_paiement_id) REFERENCES mode_paiements(id) ON DELETE CASCADE
    );
    
CREATE INDEX IF NOT EXISTS idx_reglements_tier_id ON reglements(tier_id);
CREATE INDEX IF NOT EXISTS idx_reglements_document_id ON reglements(document_id);
CREATE INDEX IF NOT EXISTS idx_reglements_caisse_id ON reglements(caisse_id);
CREATE INDEX IF NOT EXISTS idx_reglements_boutique_id ON reglements(boutique_id);
CREATE INDEX IF NOT EXISTS idx_reglements_mode_paiement_id ON reglements(mode_paiement_id);
CREATE INDEX IF NOT EXISTS idx_reglements_user_id ON reglements(user_id);
-- Add migration script here

 CREATE TABLE IF NOT EXISTS ligne_documents(
      id TEXT PRIMARY KEY,
      document_id TEXT,
      article_id TEXT,
      prix_achat_ttc REAL,
      qte REAL,
      qte_mvt_stock REAL,
      prix_vente_ttc REAL,
      taux_remise REAL,
      montant_ttc REAL,
      montant_remise REAL,
      montant_net REAL,
      prix_vente_standard REAL,
      prix_achat_standard REAL,
      achever INTEGER,
      qte_last_stock REAL,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE,
      FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
      FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
    );
CREATE INDEX IF NOT EXISTS idx_ligne_documents_document_id ON ligne_documents(document_id);
CREATE INDEX IF NOT EXISTS idx_ligne_documents_article_id ON ligne_documents(article_id);
-- Add migration script here
CREATE TABLE IF NOT EXISTS familles(
      id TEXT PRIMARY KEY,
      code TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      name TEXT,
      synchronise boolean DEFAULT FALSE
    );
    -- Add migration script here
CREATE TABLE IF NOT EXISTS sous_familles(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      famille_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
synchronise boolean DEFAULT FALSE
    );

CREATE INDEX IF NOT EXISTS idx_sous_familles_famille_id ON sous_familles(famille_id);
-- Add migration script here
 CREATE TABLE IF NOT EXISTS marques(
      id TEXT PRIMARY KEY,
      code TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      name TEXT,
      synchronise boolean DEFAULT FALSE
    );
    -- Add migration script here
  CREATE TABLE IF NOT EXISTS unites(
      id TEXT PRIMARY KEY,
      code TEXT,
      compagny_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      name TEXT,
     synchronise boolean DEFAULT FALSE
    );
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
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
CREATE INDEX IF NOT EXISTS idx_depenses_type_depense_id ON depenses(type_depense_id);
CREATE INDEX IF NOT EXISTS idx_depenses_user_id ON depenses(user_id);
CREATE INDEX IF NOT EXISTS idx_depenses_caisse_id ON depenses(caisse_id);
CREATE INDEX IF NOT EXISTS idx_depenses_mode_paiement_id ON depenses(mode_paiement_id);
-- Add migration script here
CREATE TABLE IF NOT EXISTS type_depenses(
      id TEXT PRIMARY KEY,
      code TEXT,
      name TEXT,
      boutique_id TEXT,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );
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
    -- Add migration script here
 CREATE TABLE IF NOT EXISTS admins(
      id TEXT PRIMARY KEY,
      name TEXT,
      email TEXT,
      password TEXT,
      phone TEXT,
      compagny_id TEXT,
      synchronise boolean DEFAULT FALSE
    );
    -- Add migration script here
CREATE TABLE IF NOT EXISTS reglement_documents(
      id TEXT PRIMARY KEY,
      reglement_id TEXT,
      document_id TEXT,
      montant TEXT,
     created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
      synchronise boolean DEFAULT FALSE
    );