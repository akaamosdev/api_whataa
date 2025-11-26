
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::{FromRow, PgPool};

use crate::errors::AppError;

pub async fn check_database(State(pool): State<PgPool>)-> Result<impl IntoResponse, AppError> {
    
    let count:i64 = sqlx::query_scalar("SELECT COUNT(id) FROM users")
    .fetch_one(&pool)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok((StatusCode::OK,Json(json!({
        "statut":count<0,
    }))))
}
#[derive(Serialize, FromRow)]
pub struct TiersData{
    pub id:String,
    pub denomination:String,

}
pub async fn all_tiers(State(pool): State<PgPool>,Path(table):Path<String>) 
-> Result<impl IntoResponse, AppError> {
    let sqlc = format!("SELECT id, denomination FROM {} ORDER BY denomination ASC",table);
    let familles: Vec<TiersData> =
        sqlx::query_as(&sqlc)
            .fetch_all(&pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(familles)))
}