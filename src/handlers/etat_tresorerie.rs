use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{errors::AppError, models::etats::{ChiffreAffaire, MvtCompte, MvtDepense, ParamsTresorerie}};



pub async fn chiffre_affaire(State(pool): State<PgPool>,
    Query(params): Query<ParamsTresorerie>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        ChiffreAffaire,
        r#"
        SELECT date_mvt ,COALESCE(SUM(achat),0) AS s_achat, COALESCE(SUM(vente),0) AS s_vente,
        COALESCE(SUM(depense),0) AS s_depense, 
        COALESCE(SUM(vente),0)-COALESCE(SUM(achat),0)-COALESCE(SUM(depense),0) AS marge
        FROM (
            SELECT document_date::text AS date_mvt, 0 AS achat, montant_net AS vente, 0 AS depense
            FROM documents
            WHERE type_doc = 2 AND document_date::text BETWEEN $1 AND $2
            

            UNION ALL

            SELECT document_date::text AS date_mvt, montant_net AS achat, 0 AS vente, 0 AS depense
            FROM documents
            WHERE type_doc = 1 AND document_date::text BETWEEN $1 AND $2
             

            UNION ALL

            SELECT date_depense::text AS date_mvt, 0 AS achat, 0 AS vente, montant AS depense
            FROM depenses
            WHERE date_depense::text BETWEEN $1 AND $2
            AND (NULLIF($3, '') IS NULL OR caisse_id = $3) 
        ) AS mouvements GROUP BY date_mvt
        ORDER BY date_mvt
        "#,
        params.date_start,
        params.date_end,
        params.caisse_id
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(json!({ "datas": result}))))
    
}
pub async fn mvt_compte(State(pool): State<PgPool>,
    Query(params): Query<ParamsTresorerie>,
) -> Result<impl IntoResponse, AppError> {
    let data: Vec<_> = sqlx::query_as!(
        MvtCompte,
    r#"
    SELECT date_mvt ,label, debit, credit,
    SUM(credit - debit) OVER (
        ORDER BY date_mvt, ordere
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS solde 
    FROM (
        SELECT reglement_date::text AS date_mvt, 
        'PAIEMENT CLIENT' AS label, 
        0 AS debit, montant AS credit, 1 AS ordere
        FROM reglements
        INNER JOIN tiers ON reglements.tier_id = tiers.id
        WHERE tiers.type_tier = 'CLIENT' AND reglement_date::text BETWEEN $1 AND $2
        AND (NULLIF($3, '') IS NULL OR caisse_id = $3)

        UNION ALL

       SELECT reglement_date::text AS date_mvt, 
        'REGLEMENT FOURNISSEUR' AS label, 
        montant AS debit, 0 AS credit, 2 AS ordere
        FROM reglements
        INNER JOIN tiers ON reglements.tier_id = tiers.id
        WHERE tiers.type_tier = 'FOURNISSEUR' AND reglement_date::text BETWEEN $1 AND $2
        AND (NULLIF($3, '') IS NULL OR caisse_id = $3)

        UNION ALL

        SELECT date_depense::text AS date_mvt, 
        'DEPENSE' AS label,
         montant AS debit, 0 AS credit, 3 AS ordere
        FROM depenses
        WHERE date_depense::text BETWEEN $1 AND $2
        AND (NULLIF($3, '') IS NULL OR caisse_id = $3)

    ) AS mouvements ORDER BY date_mvt
        "#,
        params.date_start,
        params.date_end,
        params.caisse_id
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}
pub async fn mvt_depense(State(pool): State<PgPool>,
    Query(params): Query<ParamsTresorerie>,
) -> Result<impl IntoResponse, AppError> {
    let data: Vec<_> = sqlx::query_as!(
        MvtDepense,
    r#"
    SELECT date_depense::text, comment, montant, modes.name AS mode_paiement
    FROM depenses
    INNER JOIN mode_paiements AS modes ON depenses.mode_paiement_id = modes.id
    WHERE date_depense::text BETWEEN $1 AND $2
    AND (NULLIF($3, '') IS NULL OR caisse_id = $3)
        "#,
        params.date_start,
        params.date_end,
        params.caisse_id
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}   