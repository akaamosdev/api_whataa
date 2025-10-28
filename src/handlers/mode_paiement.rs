use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;

use crate::{errors::AppError, models::mode_paiement::{ ModePaiementShow}};



pub async fn get_mode_paiement(State(pool): State<SqlitePool>) -> Result<impl IntoResponse, AppError> {
    let sqlc = format!("SELECT id, name FROM mode_paiements ORDER BY created_at DESC");
    let familles: Vec<ModePaiementShow> =
        sqlx::query_as(&sqlc)
            .fetch_all(&pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(familles)))
}