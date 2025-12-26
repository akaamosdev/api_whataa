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
('Admin', 'Administrateur avec tous les droits'),
('Caissier', 'Utilisateur avec droits limités pour la gestion des ventes au comptoir');

INSERT INTO permissions (name, description) VALUES
('supplier.list', 'Lister fournisseurs'),
('supplier.create', 'Ajouter fournisseur'),
('supplier.update', 'Modifier fournisseur'),
('supplier.delete', 'Supprimer fournisseur'),

('client.list', 'Lister clients'),
('client.create', 'Ajouter client'),
('client.update', 'Modifier client'),
('client.delete', 'Supprimer client'),

('article.list', 'Lister articles'),
('article.create', 'Ajouter article'),
('article.update', 'Modifier article'),
('article.delete', 'Supprimer article'),
('article.import', 'Importer articles'),
('article.export', 'Exporter articles'),

('bc.list', 'Lister bons de commande'),
('bc.create', 'Créer bon de commande'),
('bc.update', 'Modifier bon de commande'),
('bc.delete', 'Supprimer bon de commande'),
('bc.approve', 'Approuver bon de commande'),
('bc.print', 'Imprimer bon de commande'),

('bl.list', 'Lister bons de livraison'),
('bl.create', 'Créer bon de livraison'),
('bl.update', 'Modifier bon de livraison'),
('bl.delete', 'Supprimer bon de livraison'),
('bl.approve', 'Approuver bon de livraison'),
('bl.print', 'Imprimer bon de livraison'),

('purchase.list', 'Lister achats'),
('purchase.create', 'Créer achat'),
('purchase.update', 'Modifier achat'),
('purchase.delete', 'Supprimer achat'),
('purchase.print', 'Imprimer achat'),

('sale.list', 'Lister ventes'),
('sale.create', 'Créer vente'),
('sale.update', 'Modifier vente'),
('sale.delete', 'Supprimer vente'),
('sale.print', 'Imprimer vente'),

('rt.list', 'Lister retours achats'),
('rt.create', 'Créer retour achat'),
('rt.update', 'Modifier retour achat'),
('rt.delete', 'Supprimer retour achat'),
('rt.approve', 'Approuver retour achat'),
('rt.print', 'Imprimer retour achat'),

('rf.list', 'Lister reglements fournisseur'),
('rf.create', 'Créer reglement fournisseur'),
('rf.update', 'Modifier reglement fournisseur'),
('rf.delete', 'Supprimer reglement fournisseur'),
('rf.print', 'Imprimer reglements fournisseur'),

('devis.list', 'Lister devis'),
('devis.create', 'Créer devis'),
('devis.update', 'Modifier devis'),
('devis.delete', 'Supprimer devis'),
('devis.print', 'Imprimer devis'),
('devis.approve', 'Approuver devis'),

('rtc.list', 'Lister les retours clients'),
('rtc.create', 'Créer retour client'),
('rtc.update', 'Modifier retour client'),
('rtc.delete', 'Supprimer retour client'),
('rtc.print', 'Imprimer retour client'),

('payment.list', 'Lister paiements clients'),
('payment.create', 'Créer paiement client'),
('payment.update', 'Modifier paiement client'),
('payment.delete', 'Supprimer paiement client'),
('payment.print', 'Imprimer paiements clients'),

('stock.in.list', 'Lister entrées stock'),
('stock.in.create', 'Ajouter entrée stock'),
('stock.in.update', 'Modifier entrée stock'),
('stock.in.delete', 'Supprimer entrée stock'),
('stock.in.print', 'Imprimer entrée stock'),

('stock.out.list', 'Lister sorties stock'),
('stock.out.create', 'Ajouter sortie stock'),
('stock.out.update', 'Modifier sortie stock'),
('stock.out.delete', 'Supprimer sortie stock'),
('stock.out.print', 'Imprimer sortie stock'),

('stock.inventory.list', 'Lister inventaire stock'),
('stock.inventory.create', 'Ajouter inventaire stock'),
('stock.inventory.update', 'Modifier inventaire stock'),
('stock.inventory.delete', 'Supprimer inventaire stock'),
('stock.inventory.print', 'Imprimer inventaire stock'),
('stock.inventory.ajust', 'Ajuster inventaire stock'),

('expense.list', 'Lister dépenses'),
('expense.create', 'Ajouter dépense'),
('expense.update', 'Modifier dépense'),
('expense.delete', 'Supprimer dépense'),

('compte.list', 'Lister des comptes tresorerie'),
('compte.create', 'Ajouter compte trésorerie'),
('compte.update', 'Modifier compte trésorerie'),
('compte.delete', 'Supprimer compte trésorerie'),
('compte.mouvement', 'Gérer mouvements compte trésorerie'),
('compte.virement', 'Effectuer virement entre comptes'),

('stats.view', 'Voir statistiques'),
('pos.sale', 'Vente au comptoir');


INSERT INTO permission_role (role_id, permission_id)
SELECT 1, id FROM permissions;

INSERT INTO permission_role (role_id, permission_id)
SELECT 2, id FROM permissions WHERE name = 'pos.sale';
---------------------end add privilege