use std::f32;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, pool, prelude::FromRow};

use crate::{errors::AppError, models::depense};
//structs and models
#[derive(FromRow, Serialize)]
pub struct StatistisResponse {
    pub vente_count: Option<i64>,
    pub vente_total: Option<f32>,
    pub marge_total: Option<f32>,
}
#[derive(FromRow, Serialize)]
pub struct StockGlobalResponse {
    pub total_stock: Option<f32>,
    pub stock_value: Option<f32>,
    pub stock_value_seller: Option<f32>,
    pub potential_marge: Option<f32>,
    pub total_article: Option<i64>,
}
//handler function
#[derive(Deserialize, FromRow, Serialize)]
pub struct GraphicData {
    pub mois: Option<String>,
    pub mois_num: Option<i32>,
    pub total_vente: Option<f32>,
}

pub async fn statistis_handler(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    // Your handler logic here
    let vent_day = sqlx::query_as!(
        StatistisResponse,
        r#"
        SELECT COALESCE(COUNT(*),0) as vente_count, COALESCE(SUM(montant_net),0) as vente_total,
        COALESCE(SUM(marge),0) as marge_total
        FROM documents 
        INNER JOIN (
            SELECT document_id, SUM((prix_vente_ttc-prix_achat_ttc)*qte) as marge
            FROM ligne_documents
            GROUP BY document_id
        ) as ld ON ld.document_id = documents.id
        WHERE DATE(created_at) = CURRENT_DATE AND type_doc = 2
        "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    let depense_day = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(montant),0) as total_depense
        FROM depenses 
        WHERE DATE(created_at) = CURRENT_DATE
        "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    let achat_day = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(montant_net),0) as total_achat
        FROM documents 
        WHERE DATE(created_at) = CURRENT_DATE AND type_doc = 1
        "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    // ----------------------------------------------
    let stock_global = sqlx::query_as!(
        StockGlobalResponse,
        r#"
    SELECT COALESCE(SUM(stock),0) as total_stock, COALESCE(SUM(price_buy * stock),0) as stock_value,
    COALESCE(SUM(price_seller * stock),0) as stock_value_seller,
    COALESCE(SUM((price_seller - price_buy) * stock),0) as potential_marge,
    COUNT(*) as total_article
    FROM articles 
    "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    // ----------------------------------------------
    let client_solde = sqlx::query_scalar!(
        r#"
    SELECT COALESCE(SUM(montant_vente),0) - COALESCE(SUM(montant_regle),0) as total_solde
    FROM tiers 
    LEFT JOIN (
        SELECT tier_id, SUM(montant_net) as montant_vente
        FROM documents
        WHERE type_doc = 2
        GROUP BY tier_id
    ) as ventes ON ventes.tier_id = tiers.id
    LEFT JOIN (
        SELECT tier_id, SUM(montant) as montant_regle
        FROM reglements
        GROUP BY tier_id
    ) as regles ON regles.tier_id = tiers.id
    WHERE type_tier = 'CLIENT'
    "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    // ----------------------------------------------
    let fournisseur_solde = sqlx::query_scalar!(
        r#"
    SELECT COALESCE(SUM(montant_achat),0) - COALESCE(SUM(montant_regle),0) as total_solde
    FROM tiers 
    LEFT JOIN (
        SELECT tier_id, SUM(montant_net) as montant_achat
        FROM documents
        WHERE type_doc = 1
        GROUP BY tier_id
    ) as ventes ON ventes.tier_id = tiers.id
    LEFT JOIN (
        SELECT tier_id, SUM(montant) as montant_regle
        FROM reglements
        GROUP BY tier_id
    ) as regles ON regles.tier_id = tiers.id
    WHERE type_tier = 'FOURNISSEUR'
    "#
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    // ----------------------------------------------get data graphique ventes depenses achats
    let graphic_data = sqlx::query_as!(
        GraphicData,
        r#"
       WITH months AS (
        SELECT generate_series(
            date_trunc('year', CURRENT_DATE),
            date_trunc('year', CURRENT_DATE) + interval '11 months',
            interval '1 month'
        ) AS mois
        )
        SELECT
        TO_CHAR(m.mois, 'TMMonth') AS mois,
        EXTRACT(MONTH FROM m.mois)::INT AS mois_num,
        COALESCE(SUM(d.montant_net), 0) AS total_vente
        FROM months m
        LEFT JOIN documents d
        ON date_trunc('month', d.created_at) = m.mois
        AND d.type_doc = 2
        GROUP BY m.mois
        ORDER BY mois_num;
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::OK,
        Json(json!({"vente_day": vent_day, "depense_day": depense_day, 
            "achat_day": achat_day, "stock_global": stock_global, 
            "client_solde": client_solde, 
            "fournisseur_solde": fournisseur_solde,
            "graphic_data": graphic_data})),
    ))
}
