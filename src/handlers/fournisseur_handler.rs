use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::fournisseur::Fournisseur;
use crate::models::helper_model::PaginateParam;




pub async fn fournisseur_paginates(
    State(pool): State<SqlitePool>,
    Query(params): Query<PaginateParam>,
) -> Result<impl IntoResponse, AppError> {
    let mut sqlc = format!(
        "
    SELECT 
    fou.*, 0.0 AS solde
    FROM fournisseurs fou
    "
    );
    let mut search_pattern: Option<String> = None;

    if let Some(search) = &params.search {
        sqlc.push_str(" WHERE code LIKE ? OR denomination LIKE ? OR phone_mobil LIKE ?");
    }

    sqlc.push_str(" ORDER BY created_at DESC LIMIT 25 OFFSET ?");

    let mut q = sqlx::query_as::<_, Fournisseur>(&sqlc);

    if let Some(search) = &params.search {
        search_pattern = Some(format!("%{}%", search));
        q = q
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern);
    }

    q = q.bind(params.offset);

    let fournisseurs: Vec<Fournisseur> = q
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(fournisseurs)))
}


pub async fn fournisseur_add(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Fournisseur>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    INSERT INTO fournisseurs (
        id, code, denomination,nb_commerce,nb_contribuable,address_phy,boite_postale,
        phone_fix,phone_mobil,faxe,address_mail,boutique_id,
        synchronise
    )
    VALUES (
        ?, ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?, ?,
        ?
    )
    ",
    );

    sqlx::query(&query)
        .bind(&payload.id)
        .bind(&payload.code)
        .bind(&payload.denomination)
        .bind(&payload.nb_commerce)
        .bind(&payload.nb_contribuable)
        .bind(&payload.address_phy)
        .bind(&payload.boite_postale)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(payload.faxe)
        .bind(payload.address_mail)
        .bind(payload.boutique_id)
        .bind(payload.synchronise)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "Fournisseur enregistré avec succès"
        })),
    ))
}
pub async fn fournisseur_update(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Fournisseur>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    UPDATE fournisseurs 
        SET code=?, denomination=?, nb_commerce=?, nb_contribuable=?, address_phy=?, boite_postale=?,
        phone_fix=?,phone_mobil=?,faxe=?,address_mail=?,boutique_id=?,
        synchronise=?
        WHERE id=?
    ",
    );

    sqlx::query(&query)
        
        .bind(&payload.code)
        .bind(&payload.denomination)
        .bind(&payload.nb_commerce)
        .bind(&payload.nb_contribuable)
        .bind(&payload.address_phy)
        .bind(&payload.boite_postale)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(&payload.faxe)
        .bind(&payload.address_mail)
        .bind(&payload.boutique_id)
        .bind(&payload.synchronise)
        .bind(&payload.id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "Fournisseur modifié avec succès"
        })),
    ))
}
//zz