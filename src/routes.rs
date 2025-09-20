use axum::{Router, routing::{post,get}};
use sqlx::SqlitePool;
use crate::handlers::auth::{get_all_users, login, register};

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
       // .route("/users", get(get_all_users).post(handler))
        // .nest("pv",
        /* Router::new()
            .route("/product", get(handler)) */
        //)
        .with_state(pool)
}