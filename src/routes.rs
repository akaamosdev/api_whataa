use crate::handlers::etat_article::{mvt_stock_article, stock_alert, stock_avaible};
use crate::handlers::user_handler::{all_users, role_users, store_new_user};
use crate::handlers::{
    auth::{ get_data_default, login, register},
    caisse_handler::{caisse_get, mvt_caisse_get, mvt_caisse_store, store_caisse},
    compagny_handler::{create_compagny, get_compagny, update_compagny},
    depense_handler::{get_depenses, store_depense},
    document_handler::{doc_delete, stock_ajuste, stock_get, store_document},
    etat_tresorerie::{chiffre_affaire, mvt_compte, mvt_depense},
    etat_vente_handler::{
        article_vente_achat, etat_creance_tier, etat_mvt_tier, etat_paiement_tier,
        etat_vente_by_client, etat_vente_facture,
    },
    famille::{add_famille, delete_famille, get_familles, update_famille},
    helpers::{get_last_counts, import_articles, upload_file},
    mode_paiement::get_mode_paiement,
    product_handler::{
        article_add, article_by_id, article_check_stock, article_documents, article_paginates,
        article_update,
    },
    reglement_handler::{delete_regle, regle_client, store_reglement},
    sous_famille::{
        sous_famille_add, sous_famille_delete, sous_famille_update, sous_familles_by_famille,
        sous_familles_get,
    },
    statistis_handler::statistis_handler,
    sync_handler::send_data,
    tier_handler::{tier_add, tier_paginates, tier_update},
    user_handler::{all_tiers, check_database},
    vente_handler::{doc_attente_lignes, documents_attente, vente_by_id, vente_get},
};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::services::ServeDir;

pub fn create_router(pool: PgPool) -> Router {
    // Routes pour les familles
    let famille_routes = Router::new().route(
        "/",
        post(add_famille).put(update_famille).delete(delete_famille),
    );
    let sous_famille_routes = Router::new().route(
        "/",
        get(sous_familles_get)
            .post(sous_famille_add)
            .put(sous_famille_update)
            .delete(sous_famille_delete),
    );
    let article_routes = Router::new().route(
        "/",
        get(article_paginates).post(article_add).put(article_update),
    );
    let tier_routes = Router::new().route("/", get(tier_paginates).post(tier_add).put(tier_update));
    let document_routes = Router::new().route(
        "/",
        get(vente_get).post(store_document).delete(doc_delete), // .put(client_update)
    );
    let reglement_routes = Router::new().route(
        "/",
        get(regle_client).post(store_reglement).delete(delete_regle),
    );

    let stock_routes = Router::new().route(
        "/",
        get(stock_get).post(store_document), // .put(client_update)
    );
    let depense_routes = Router::new().route("/", get(get_depenses).post(store_depense));
    let solde_caisse = Router::new().route("/", get(caisse_get).post(store_caisse));
    let mvt_caisse = Router::new().route("/", get(mvt_caisse_get).post(mvt_caisse_store));
    let user_route: Router<sqlx::Pool<sqlx::Postgres>> =
        Router::new().route("/", get(all_users).post(store_new_user));
    // Router principal
    Router::new()
        .route("/check_database", get(check_database))
        .route("/store_compagny", post(create_compagny))
        .route("/get_compagny/{compagny_id}", get(get_compagny))
        .route("/update_compagny", post(update_compagny))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/all_tiers/{type_tier}", get(all_tiers))
        .route("/last_count", post(get_last_counts))
        .route("/params/{table}", get(get_familles))
        .route("/mode_paiement", get(get_mode_paiement))
        .route("/sous_famille/by/{famille}", get(sous_familles_by_famille))
        .route("/article/{id}", get(article_by_id))
        .route("/upload", post(upload_file))
        .route("/send_data", post(send_data))
        .route("/import_articles", post(import_articles))
        // .route("/solde_initial", post(store_solde_initial))
        .route("/default/data", get(get_data_default))
        .nest_service("/uploads", ServeDir::new("./uploads"))
        .nest("/familles", famille_routes)
        .nest("/sous_familles", sous_famille_routes)
        .nest("/articles", article_routes)
        .nest("/tiers", tier_routes)
        .nest("/documents", document_routes)
        .nest("/reglements", reglement_routes)
        .nest("/stocks", stock_routes)
        .nest("/depenses", depense_routes)
        .nest("/caisses", solde_caisse)
        .nest("/mouvements_caisses", mvt_caisse)
        .nest("/users", user_route)
        
        //vente by id
        .route("/roles", get(role_users))
        .route("/document/{doc_id}", get(vente_by_id))
        .route("/articles/doc", get(article_documents))
        .route("/articles/check/stock", post(article_check_stock))
        .route("/stock/ajuste", post(stock_ajuste))
        .route("/documents_attente", get(documents_attente))
        .route(
            "/documents_attente/lignes/{doc_id}",
            get(doc_attente_lignes),
        )//routes etat
        .route("/statistic_data", get(statistis_handler))
        .route("/etat/vente/facture", get(etat_vente_facture))
        .route("/etat/vente/article", get(article_vente_achat))
        .route("/etat/vente/tier", get(etat_vente_by_client))
        .route("/etat/paiement/tier", get(etat_paiement_tier))
        .route("/etat/creance/tier", get(etat_creance_tier))
        .route("/etat/mouvement/tier", get(etat_mvt_tier))
        .route("/etat/tresorerie/chiffre_affaire", get(chiffre_affaire))
        .route("/etat/tresorerie/mouvement_compte", get(mvt_compte))
        .route("/etat/tresorerie/mouvement_depense", get(mvt_depense))
        .route("/etat/article/stock", get(stock_avaible))
        .route("/etat/article/stock_alert", get(stock_alert))
        .route("/etat/article/mouvement_stock", get(mvt_stock_article))//famille handlers
        .route("/sous-famille/by/{famille_id}", get(sous_familles_by_famille))

        .with_state(pool)
}
