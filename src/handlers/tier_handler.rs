use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::tier::Tier;
use crate::models::helper_model::PaginateParam;




pub async fn tier_paginates(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateParam>,
) -> Result<impl IntoResponse, AppError> {
    let mut sqlc = format!(
        "
    SELECT 
    fou.*, CAST (0.0 AS FLOAT4) AS solde
    FROM Tiers fou
    WHERE type_tier= $1
    "
    );
    let mut search_pattern: Option<String> = None;

    if params.search.is_some() {
        sqlc.push_str(" AND (code ILIKE $2 OR denomination ILIKE $3 OR phone_mobil LIKE $4)");
        sqlc.push_str(" ORDER BY created_at DESC LIMIT 25 OFFSET $5");

    }else {
        sqlc.push_str(" ORDER BY created_at DESC LIMIT 25 OFFSET $2");
    }

    
    let mut q = sqlx::query_as::<_, Tier>(&sqlc);

    q = q.bind(&params.type_tier);
    if let Some(search) = &params.search {
        search_pattern = Some(format!("%{}%", search));
        q = q
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&params.offset);
    } else {
        q = q.bind(&params.offset);
    }

    let tiers: Vec<Tier> = q
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(tiers)))
}

pub async fn tier_add(
    State(pool): State<PgPool>,
    Json(payload): Json<Tier>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        " INSERT INTO tiers (
        id,type_tier, code, denomination,nb_commerce,nb_contribuable,address_phy,boite_postale,
        phone_fix,phone_mobil,address_mail,boutique_id
    )
    VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
    )
    ",
    );

    sqlx::query(&query)
        .bind(&payload.id)
        .bind(&payload.type_tier)
        .bind(&payload.code)
        .bind(&payload.denomination)
        .bind(&payload.nb_commerce)
        .bind(&payload.nb_contribuable)
        .bind(&payload.address_phy)
        .bind(&payload.boite_postale)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(&payload.address_mail)
        .bind(&payload.boutique_id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    //

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": format!("{} enregistré avec succès", &payload.type_tier)
        })),
    ))
}
pub async fn tier_update(
    State(pool): State<PgPool>,
    Json(payload): Json<Tier>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    UPDATE tiers 
        SET code = $1,
            denomination = $2,
            nb_commerce = $3,
            nb_contribuable = $4,
            address_phy = $5,
            boite_postale = $6,
            phone_fix = $7,
            phone_mobil = $8,
            address_mail = $9,
            boutique_id = $10
    WHERE id = $11
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
        .bind(&payload.address_mail)
        .bind(&payload.boutique_id)
        .bind(&payload.id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": format!("{} modifié avec succès", &payload.denomination)
        })),
    ))
}
//zz