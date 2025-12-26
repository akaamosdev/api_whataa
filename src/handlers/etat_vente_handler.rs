use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{ json};
use sqlx::{PgPool};

use crate::{errors::AppError, models::{etats::{ArticleVenteAchat, EtatCreanceTier, EtatMvtTier, EtatReglementTier, EtatVenteCumuled, ParamsEtatDoc, VenteFacture}, tier}};



pub async fn etat_vente_facture(
    State(pool): State<PgPool>,
    Query(params): Query<ParamsEtatDoc>,
) -> Result<impl IntoResponse, AppError> {

    let ventes = sqlx::query_as!(
        VenteFacture,
        r#"
        SELECT
        d.id,
        t.denomination AS tier_name,
        d.document_num AS numero,
        d.document_date AS date,
        d.montant_net,
        d.montant_tva,
        d.montant_total,
        d.montant_remise,

        COALESCE(
            json_agg(
                json_build_object(
                    'id', l.id,
                    'article_id', l.article_id,
                    'code_bar', a.code_bar,
                    'designation', a.name,
                    'qte', l.qte,
                    'unite', u.name,
                    'prix_vente_ttc', l.prix_vente_ttc,
                    'montant_net', l.montant_net
                )
            ) FILTER (WHERE l.id IS NOT NULL),
            '[]'::json
        ) AS lignes
    FROM documents d
    INNER JOIN ligne_documents l ON l.document_id = d.id
    INNER JOIN articles a ON a.id = l.article_id
    INNER JOIN unites u ON u.id = a.unite_id
    INNER JOIN tiers t ON t.id = d.tier_id
    WHERE d.type_doc = $4 AND (NULLIF($1, '') IS NULL OR d.tier_id = $1) AND  document_date BETWEEN $2 AND $3 
    
    GROUP BY d.id, t.denomination
        "#,
        params.tier_id,
        params.date_start,
        params.date_end,
        params.type_doc
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?
;
    Ok((
        StatusCode::OK,
        Json(json!({ "ventes": ventes })),
    ))
}

pub async fn article_vente_achat(State(pool): State<PgPool>, 
Query(params): Query<ParamsEtatDoc>) -> 
Result<impl IntoResponse, AppError> {

    let vente_day = sqlx::query_as!(
        ArticleVenteAchat,
        r#"
        SELECT 
        a.code_bar,a.name as designation, 
        SUM(lg.qte) as total_qte,
        SUM(COALESCE(lg.prix_vente_ttc * lg.qte,0)) as total_vente,
        SUM(COALESCE(lg.prix_achat_ttc * lg.qte,0)) as total_achat,
        SUM(COALESCE((lg.prix_vente_ttc - lg.prix_achat_ttc) * lg.qte,0)-lg.montant_remise) as total_marge
        FROM ligne_documents lg
        INNER JOIN documents ON documents.id = lg.document_id 
        INNER JOIN articles a ON a.id = lg.article_id 
        WHERE document_date BETWEEN $1 AND $2 AND type_doc = $3
        GROUP BY a.id ORDER BY total_qte DESC
        "#,
        params.date_start,
        params.date_end,
        params.type_doc    
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::OK,
        Json(json!({ "articles": vente_day })),
    ))
    
}

pub async fn etat_vente_by_client(State(pool): State<PgPool>,
    Query(params): Query<ParamsEtatDoc>,
) -> Result<impl IntoResponse, AppError> {
    let data: Vec<_> = sqlx::query_as!(
        EtatVenteCumuled,
        r#"
        SELECT
        d.document_num AS numero,
        t.denomination AS tier_name,
        d.document_date AS date,
        d.montant_total,
        d.montant_remise,
        d.montant_tva,
        d.montant_net
    FROM documents d
    INNER JOIN tiers t ON t.id = d.tier_id
    WHERE d.type_doc = $4 AND (NULLIF($1, '') IS NULL OR d.tier_id = $1) AND  document_date BETWEEN $2 AND $3 
        "#,
        params.tier_id,
        params.date_start,
        params.date_end,
        params.type_doc
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}
pub async fn etat_paiement_tier(State(pool): State<PgPool>,
    Query(params): Query<ParamsEtatDoc>,
) -> Result<impl IntoResponse, AppError> {
    let type_tier = if params.type_doc == 2 {
    "CLIENT"
} else {
    "FOURNISSEUR"
};
    let data: Vec<_> = sqlx::query_as!(
        EtatReglementTier,
        r#"
        SELECT
        r.reglement_num AS numero,
        t.denomination AS tier_name,
        r.reglement_date::text AS date,
        r.montant,
        mp.name AS mode_paiement,
        r.commentaire AS comment
    FROM reglements r
    INNER JOIN tiers t ON t.id = r.tier_id
    INNER JOIN mode_paiements mp ON mp.id = r.mode_paiement_id
    WHERE t.type_tier = $4 AND (NULLIF($1, '') IS NULL OR r.tier_id = $1) AND  r.reglement_date::text BETWEEN $2 AND $3 
        "#,
        params.tier_id,
        params.date_start,
        params.date_end,
        type_tier
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

println!("Params: {:?}", params);
    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}
pub async fn etat_creance_tier(State(pool): State<PgPool>,
    Query(params): Query<ParamsEtatDoc>,
) -> Result<impl IntoResponse, AppError> {
    
    let data: Vec<_> = sqlx::query_as!(
        EtatCreanceTier,
        r#"
        SELECT
        t.code, t.denomination, t.phone_mobil,
        COALESCE(SUM(d.montant_net),0) AS total_vente,
        COALESCE(SUM(r.montant),0) AS total_regle,
        COALESCE(SUM(d.montant_net),0)-COALESCE(SUM(r.montant),0) AS solde
    FROM tiers t
    LEFT JOIN (
        SELECT tier_id, SUM(montant) AS montant FROM reglements r
        WHERE (NULLIF($1, '') IS NULL OR r.tier_id = $1)
        GROUP BY tier_id
    ) r ON r.tier_id = t.id
    LEFT JOIN (
        SELECT tier_id, SUM(montant_net) AS montant_net FROM documents d
        WHERE d.type_doc = $2 AND (NULLIF($1, '') IS NULL OR d.tier_id = $1) 
        GROUP BY tier_id
    ) d ON d.tier_id = t.id
    GROUP BY t.id
    HAVING COALESCE(SUM(d.montant_net),0)-COALESCE(SUM(r.montant),0) > 0
    ORDER BY solde DESC
        "#,
        params.tier_id,
        params.type_doc,
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}
pub async fn etat_mvt_tier(State(pool): State<PgPool>,
    Query(params): Query<ParamsEtatDoc>,
) -> Result<impl IntoResponse, AppError> {
    let data: Vec<_> = sqlx::query_as!(
        EtatMvtTier,
    r#"
    SELECT date_mvt, numero, type_mvt, montant,
    SUM(montant) OVER (
        ORDER BY date_mvt, ordre
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS solde 
    FROM (
        SELECT 
        d.document_date AS date_mvt,
        d.document_num AS numero,
        CASE 
            WHEN d.type_doc = 2 THEN 'VENTE' 
            WHEN d.type_doc = 1 THEN 'ACHAT' 
            ELSE 'AUTRE' 
        END AS type_mvt,
        d.montant_net AS montant, 1 AS ordre
        FROM documents d
        WHERE d.tier_id = $1 AND d.type_doc BETWEEN 1 AND 2
        
        UNION ALL

        SELECT 
        r.reglement_date::text AS date_mvt,
        r.reglement_num AS numero,
        'REGLEMENT' AS type_mvt,
       -r.montant AS montant, 2 AS ordre
        FROM reglements r
        WHERE r.tier_id = $1
    ) AS mouvements 
     WHERE date_mvt BETWEEN $2 AND $3
    ORDER BY date_mvt
        "#,
        params.tier_id,
        params.date_start,
        params.date_end,
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    Ok((StatusCode::OK, Json(json!({ "datas": data}))))
}