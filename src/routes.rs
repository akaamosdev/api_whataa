use crate::handlers::{
    auth::{get_all_users, get_data_default, login, register}, compagny_handler::{create_compagny, get_compagny, update_compagny}, document_handler::store_document, famille::{add_famille, delete_famille, get_familles, update_famille}, helpers::{get_last_counts, import_articles, upload_file}, mode_paiement::get_mode_paiement, product_handler::{article_add, article_by_id, article_check_stock, article_documents, article_paginates, article_update}, reglement_handler::{delete_regle, regle_client, store_reglement}, sous_famille::{
        sous_famille_add, sous_famille_delete, sous_famille_update, sous_familles_by_famille,
        sous_familles_get,
    }, sync_handler::send_data, tier_handler::{tier_add, tier_paginates, tier_update}, user_handler::{all_tiers, check_database}, vente_handler::{vente_by_id, vente_get}
};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{PgPool};
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
    let tier_routes = Router::new().route(
        "/",
        get(tier_paginates)
                        .post(tier_add)
            .put(tier_update),
    );
    let document_routes =
        Router::new().route("/", get(vente_get).post(store_document)
        // .put(client_update)
    );
    let reglement_routes = Router::new().route("/", 
    get(regle_client).post(store_reglement).delete(delete_regle));
    // Router principal
    Router::new()
        .route("/check_database", post(check_database))
        .route("/store_compagny", post(create_compagny))
        .route("/get_compagny/{compagny_id}", get(get_compagny))
        .route("/update_compagny", post(update_compagny))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/all_tiers/{type_tier}", get(all_tiers))
        .route("/users", get(get_all_users))
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
        //vente by id
        .route("/document/{doc_id}", get(vente_by_id))
        .route("/articles/doc", get(article_documents))
        .route("/articles/check/stock", post(article_check_stock))
        .with_state(pool)
}
