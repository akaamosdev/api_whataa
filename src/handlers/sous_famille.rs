use axum::{
    Json,
    extract::{Path, State},
    http::{
        StatusCode,
        header::{CONTENT_TYPE, HeaderMap},
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, prelude::FromRow, query};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Deserialize, Serialize, FromRow)]
pub struct SousFamille {
    pub id: String,
    pub code: String,
    pub name: String,
    pub famille_id: String,
}
#[derive(Deserialize, Serialize, FromRow)]
pub struct SousFamilleShow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub famille_id: String,
    pub famille : String,
}
//
#[derive(Deserialize, Serialize, FromRow)]
pub struct SousFamilleByFamille {
    pub id: String,
    pub code: String,
    pub name: String,
    pub famille_id: String,
    pub famille: String,
}

pub async fn sous_familles_get(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let sqlc =
        format!("
        SELECT sous_familles.*, fam.name AS famille FROM sous_familles
        INNER JOIN familles fam ON fam.id=famille_id
         ORDER BY created_at DESC");
    let sous_familles: Vec<SousFamilleByFamille> = sqlx::query_as(&sqlc)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(sous_familles)))
}

pub async fn sous_famille_add(
    State(pool): State<PgPool>,
    Json(famil_req): Json<SousFamille>,
) -> Result<impl IntoResponse, AppError> {
    let query =
        format!("INSERT INTO sous_familles (id, code, name, famille_id) VALUES ($1,$2,$3,$4)");

    sqlx::query(&query)
        .bind(&famil_req.id)
        .bind(&famil_req.code)
        .bind(&famil_req.name)
        .bind(&famil_req.famille_id)
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
pub async fn sous_famille_update(
    State(pool): State<PgPool>,
    Json(famil_req): Json<SousFamille>,
) -> Result<impl IntoResponse, AppError> {
    let query = format!("UPDATE sous_familles SET code = $1, name = $2, famille_id = $3 WHERE id = $4");
    let rows_affected = sqlx::query(&query)
        .bind(&famil_req.code)
        .bind(&famil_req.name)
        .bind(&famil_req.famille_id)
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
pub async fn sous_famille_delete(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let rows_affected = sqlx::query("DELETE FROM sous_familles WHERE id = $1")
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
            "message": "Sous Famille supprimée avec succès"
        })),
    ))
}
//sous_famille by famille
pub async fn sous_familles_by_famille(
    State(pool): State<PgPool>,
    Path(famille_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let sqlc = format!(
        "
    SELECT sous_familles.*, familles.name AS famille FROM sous_familles 
    INNER JOIN familles ON familles.id=sous_familles.famille_id
    WHERE famille_id=$1"
    );
    let sous_familles: Vec<SousFamilleByFamille> = sqlx::query_as(&sqlc)
        .bind(&famille_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(sous_familles)))
}
