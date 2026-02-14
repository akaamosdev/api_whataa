use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::{PgPool, Postgres, QueryBuilder, query};

use crate::{errors::AppError, models::{caisse::{Caisse, MvtCaissePlayload, MvtCaisseShow, PaginateCaisse}}};

pub async fn caisse_get(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let caisses = sqlx::query_as::<_, Caisse>(
        r#"
        SELECT id, code, name, boutique_id, COALESCE(entre_caisse, 0)-COALESCE(total_depense, 0) - 
        COALESCE(sortie_caisse, 0) + COALESCE(total_mvt, 0) AS solde_caisse
        FROM caisses
        LEFT JOIN (
            SELECT SUM(montant) AS entre_caisse, caisse_id
            FROM reglements
            INNER JOIN tiers ON reglements.tier_id = tiers.id
            WHERE tiers.type_tier = 'CLIENT'
            GROUP BY caisse_id  
        ) AS reglement_enter ON caisses.id = reglement_enter.caisse_id
        LEFT JOIN (
            SELECT SUM(montant) AS sortie_caisse, caisse_id
            FROM reglements
            INNER JOIN tiers ON reglements.tier_id = tiers.id
            WHERE tiers.type_tier = 'FOURNISSEUR'
            GROUP BY caisse_id  
        ) AS reglement_sortie ON caisses.id = reglement_sortie.caisse_id
        LEFT JOIN (
            SELECT SUM(montant) AS total_depense, caisse_id
            FROM depenses
            GROUP BY caisse_id  
        ) AS depense_totals ON caisses.id = depense_totals.caisse_id
        LEFT JOIN (
            SELECT SUM(montant) AS total_mvt, caisse_id
            FROM reglements WHERE tier_id IS NULL
            GROUP BY caisse_id  
        ) AS reglement_totals ON caisses.id = reglement_totals.caisse_id

        ORDER BY name
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::SqlxError(e))?;
    Ok((
        StatusCode::OK,
        Json(caisses),
    ))
}

// store a new caisse
pub async fn store_caisse(
    State(pool): State<PgPool>,
    Json(payload): Json<Caisse>,
) -> Result<impl IntoResponse, AppError> {
    query!(
        r#"
        INSERT INTO caisses (id, code, name, boutique_id)
        VALUES ($1, $2, $3, $4)
        "#,
        &payload.id, &payload.code, &payload.name, &payload.boutique_id   
    )
    .execute(&pool)
    .await
    .map_err(|e| AppError::SqlxError(e))?;
    Ok((
        StatusCode::CREATED,Json(json!({"message": "Caisse créée avec succès","status": true})),
    ))
}
// mvt caisse
pub async fn mvt_caisse_store(
    State(pool): State<PgPool>,
    Json(params): Json<MvtCaissePlayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut  tx = pool.begin().await.map_err(|e| AppError::SqlxError(e))?;
    // Implementation for caisse movements
    let date_mvt = chrono::Utc::now().naive_utc();
    sqlx::query!(
        r#"
        INSERT INTO reglements(
            user_id, reglement_date, montant, 
            boutique_id, caisse_id,
            commentaire, id
        )
        VALUES ($1, $2::date, $3, $4, $5, $6, gen_random_uuid()::text)
        "#,
        &params.user_id,
        &date_mvt.date(),
        &params.montant,
        &params.boutique_id,
        &params.caisse_credit_id,
        &params.comment,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    sqlx::query!(
        r#"
        INSERT INTO reglements(
            user_id, reglement_date, montant, 
            boutique_id, caisse_id,
            commentaire, id
        )
        VALUES ($1, $2::date, $3, $4, $5, $6, gen_random_uuid()::text)
        "#,
        &params.user_id,
        &date_mvt.date(),
        &params.montant*(-1.0),
        &params.boutique_id,
        &params.caisse_debit_id,
        &params.comment,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    tx.commit().await.map_err(|e| AppError::SqlxError(e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"message": "Caisse movements fetched","status": true})),
    ))
}  
pub async fn mvt_caisse_get(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateCaisse>,
) -> Result<impl IntoResponse, AppError> {
    let mut mvt_q = QueryBuilder::<Postgres>::new(
        r#"
        SELECT * FROM (
        SELECT reglement_num AS numero_mvt,reglement_date AS date_mvt, montant,c.name AS caisse,
        caisse_id,commentaire,
        'Paiement Client' AS type_mvt, u.name AS user_by
        FROM reglements 
        INNER JOIN caisses c ON reglements.caisse_id = c.id
        INNER JOIN tiers t ON reglements.tier_id = t.id
        INNER JOIN users u ON reglements.user_id = u.id
        WHERE t.type_tier = 'CLIENT'

        UNION ALL
        SELECT reglement_num AS numero_mvt,reglement_date AS date_mvt, -montant,c.name AS caisse,
        caisse_id,commentaire,
        'Paiement Fournisseur' AS type_mvt, u.name AS user_by
        FROM reglements 
        INNER JOIN caisses c ON reglements.caisse_id = c.id
        INNER JOIN tiers t ON reglements.tier_id = t.id
        INNER JOIN users u ON reglements.user_id = u.id
        WHERE t.type_tier = 'FOURNISSEUR'

        UNION ALL
        SELECT 'VIR00001' AS numero_mvt,reglement_date AS date_mvt, montant,c.name AS caisse,
        caisse_id,commentaire,
        'VIREMENT CAISSE' AS type_mvt, u.name AS user_by
        FROM reglements 
        INNER JOIN caisses c ON reglements.caisse_id = c.id
        INNER JOIN users u ON reglements.user_id = u.id
        WHERE tier_id IS NULL

        UNION ALL
        SELECT depenses.code AS numero_mvt, date_depense AS date_mvt, -montant, c.name AS caisse,
        caisse_id,comment AS commentaire,
        'depense' AS type_mvt, u.name AS user_by
        FROM depenses 
        INNER JOIN caisses c ON depenses.caisse_id = c.id
        INNER JOIN users u ON depenses.user_id = u.id
        ) AS mouvements
        
        "#,
    );

    if let Some(caisse_id) = &params.caisse_id {
        mvt_q.push(" WHERE caisse_id = ");
        mvt_q.push_bind(caisse_id);
    }
    if let Some(date_start) = &params.date_start {
        if params.caisse_id.is_some() {
            mvt_q.push(" AND date_mvt::date >= ");
        } else {
            mvt_q.push(" WHERE date_mvt::date >= ");
        }
        
        mvt_q.push_bind(date_start);
    }
    if let Some(date_end) = &params.date_end {
        if params.caisse_id.is_some() || params.date_start.is_some() {
            mvt_q.push(" AND date_mvt::date <= ");
        } else {
            mvt_q.push(" WHERE date_mvt::date <= ");
        }
        mvt_q.push_bind(date_end);
    }
    mvt_q.push(" ORDER BY date_mvt DESC ");
    mvt_q.push(" LIMIT ");
    mvt_q.push_bind(params.limit);
    mvt_q.push(" OFFSET ");
    mvt_q.push_bind(params.offset);

    let mouvements: Vec<MvtCaisseShow> = mvt_q
    .build_query_as()
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok((
        StatusCode::OK,
        Json(mouvements),
    ))
}