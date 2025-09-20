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
