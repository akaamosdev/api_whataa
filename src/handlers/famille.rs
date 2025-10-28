use axum::{
    Json,
    extract::{Path, State},
    http::{
        StatusCode,
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, SqlitePool};

use crate::errors::AppError;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Famille {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct FamilleRequest {
    pub id: String,
    pub code: String,
    pub name: String,
    pub table: String,
}


pub async fn get_familles(State(pool): State<SqlitePool>,Path(table):Path<String>) -> Result<impl IntoResponse, AppError> {
    let sqlc = format!("SELECT id, code, name FROM {} ORDER BY created_at DESC",table);
    let familles: Vec<Famille> =
        sqlx::query_as(&sqlc)
            .fetch_all(&pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(familles)))
}

pub async fn add_famille(
    State(pool): State<SqlitePool>,
    Json(famil_req): Json<FamilleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let query = format!("INSERT INTO {} (id, code, name) VALUES (?, ?, ?)",famil_req.table);

    sqlx::query(&query)
        .bind(&famil_req.id)
        .bind(&famil_req.code)
        .bind(&famil_req.name)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "Element enregistré avec succès"
        })),
    ))
}
//
pub async fn update_famille(
    State(pool): State<SqlitePool>,
    Json(famil_req): Json<FamilleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let query = format!("UPDATE {} SET code = ?, name = ? WHERE id = ?",famil_req.table);
    let rows_affected = sqlx::query(&query)
        .bind(&famil_req.code)
        .bind(&famil_req.name)
        .bind(&famil_req.id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": "Famille mise à jour avec succès"
        })),
    ))
}
pub async fn delete_famille(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let rows_affected = sqlx::query("DELETE FROM familles WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": "Famille supprimée avec succès"
        })),
    ))
}
