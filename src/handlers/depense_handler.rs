use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::{PgPool, Postgres, QueryBuilder, query};

use crate::{errors::AppError, models::depense::{Depense, DepensePayload, PaginateDepense}};

pub async fn store_depense(
    State(pool): State<PgPool>,
    Json(payload): Json<DepensePayload>,
) -> Result<impl IntoResponse, AppError> {
    // Logique pour stocker une dépense dans la base de données
    // Utilisez `pool` pour interagir avec la base de données
    if payload.is_edited {
        sqlx::query!(
            r#"
        UPDATE depenses
        SET code = $2,
            type_depense_id = $3,
            caisse_id = $4,
            mode_paiement_id = $5,
            ref_piece = $6,
            user_id = $7,
            montant = $8,
            comment = $9,
            date_depense = $10::date
        WHERE id = $1
        "#,
            payload.id,
            payload.code,
            payload.type_depense_id,
            payload.caisse_id,
            payload.mode_paiement_id,
            payload.ref_piece,
            payload.user_id,
            payload.montant,
            payload.comment,
            payload.date_depense,
        )
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    } else {
        sqlx::query!(
            r#"
        INSERT INTO depenses (id, code, type_depense_id, caisse_id,
         mode_paiement_id, ref_piece, user_id, montant, comment, date_depense)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::date)
        "#,
            payload.id,
            payload.code,
            payload.type_depense_id,
            payload.caisse_id,
            payload.mode_paiement_id,
            payload.ref_piece,
            payload.user_id,
            payload.montant,
            payload.comment,
            payload.date_depense,
        )
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    Ok((
        StatusCode::CREATED,
        Json(json!({"message": "Dépense enregistrée avec succès","status": true})),
    ))
}

// get list of depense (with pagination)
pub async fn get_depenses(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateDepense>,
) -> Result<impl IntoResponse, AppError> {

    let mut query = QueryBuilder::<Postgres>::new(
        " SELECT depenses.*, type_depenses.name AS type_depense, caisses.name AS caisse,
         mode_paiements.name AS mode_paiement, users.name AS user
        FROM depenses 
        INNER JOIN type_depenses ON depenses.type_depense_id = type_depenses.id
        INNER JOIN caisses ON depenses.caisse_id = caisses.id
        INNER JOIN mode_paiements ON depenses.mode_paiement_id = mode_paiements.id
        INNER JOIN users ON depenses.user_id = users.id

        ",
    );
    let offset = params.offset;
    let search_pattern = params
        .search
        .as_ref()
        .map(|s| format!("%{}%", s))
        .unwrap_or("%".to_string());
    if &search_pattern != "%" {
        query.push("WHERE depenses.code ILIKE ");
    query.push_bind(&search_pattern);
    }
    if let Some(date_start) =params.date_start {
        if &search_pattern != "%" {
            query.push(" AND ");
        } else {
            query.push("WHERE ");
        }
        query.push("date_depense >= ");
        query.push_bind(date_start);
    }
    if let Some(date_end) = params.date_end {
        if search_pattern != "%" || params.date_start.is_some() {
            query.push(" AND ");
        } else {
            query.push("WHERE ");
        }
        query.push("date_depense <= ");
        query.push_bind(date_end);
    }
    query.push(" ORDER BY depenses.created_at DESC LIMIT ");
    query.push_bind(params.limit);
    query.push(" OFFSET ");
    query.push_bind(offset);    
    let depenses: Vec<Depense> = query
        .build_query_as()
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    // Fetch depenses from the database
    Ok((
        StatusCode::OK,
        Json(depenses),
    ))
}