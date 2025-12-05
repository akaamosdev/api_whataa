use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{ FromRow, PgPool};

use crate::{errors::AppError, models::{document::{Document, DocumentEdit, LigneEdit}, helper_model::PaginateDocument}};

#[derive(Deserialize,FromRow,Serialize)]
pub struct VenteShow{
    pub id: String,
    pub document_num:String,
    pub document_date: String,
    pub denomination: String,
    pub montant_tva: f32,
    pub montant_total: f32,
    pub montant_ht: f32,
    pub montant_net: f32,
    pub montant_remise: f32,
    pub paye: f32,
    pub reste: f32,
    pub doc_fils_id: Option<String>
}

pub async fn vente_get(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateDocument>,
) -> Result<impl IntoResponse, AppError> {
    let offset = params.offset;

    let mut sqlc = String::from("
        SELECT d.id, d.document_num, d.document_date, denomination, d.montant_tva, d.montant_ht,d.montant_total, 
               d.montant_net, d.montant_remise, COALESCE(t_rd, 0) AS paye, d.montant_net - COALESCE(t_rd, 0) AS reste,
               doc_fs.id AS doc_fils_id
        FROM documents d
        INNER JOIN tiers ts ON ts.id = d.tier_id
        LEFT JOIN documents AS doc_fs ON doc_fs.doc_parent_id=d.id
        LEFT JOIN reglement_documents rd ON rd.document_id = d.id
        LEFT JOIN (
            SELECT document_id, SUM(montant) AS t_rd
            FROM reglement_documents rds
            GROUP BY document_id
        ) AS rds ON rds.document_id = d.id
        WHERE d.type_doc = $1 AND ts.type_tier = $2 
    ");

    let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));
    if search_pattern.is_some() {
        sqlc.push_str(
            " AND (denomination ILIKE $3
                   OR d.document_num ILIKE $4
                   OR d.document_date::text ILIKE $5 
                   OR d.montant_net::text ILIKE $6) ",
        );
        sqlc.push_str(" ORDER BY d.created_at DESC LIMIT 25 OFFSET $7");
    }else {
        sqlc.push_str(" ORDER BY d.created_at DESC LIMIT 25 OFFSET $3");
    }
    // Prépare la requête
    let mut query = sqlx::query_as::<_, VenteShow>(&sqlc);
    query = query.bind(params.type_doc).bind(params.type_tier);

    // Bind search si présent
    if let Some(ref pattern) = search_pattern {
        query = query
            .bind(pattern)
            .bind(pattern)
            .bind(pattern)
            .bind(pattern)
            .bind(offset);
    }else {
         query = query.bind(offset);
    }
    // Bind offset
    let ventes: Vec<VenteShow> = query
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(ventes)))
}

pub async fn vente_by_id(
    State(pool): State<PgPool>,
    Path(doc_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let query = r#"
        SELECT 
        docs.id,type_doc, document_num, document_date, depot_id,
        montant_tva, montant_ht, montant_total,
        montant_net, montant_remise, tier_id,taux_remise, doc_parent_id,
        denomination,address_mail,phone_mobil,phone_fix, type_tier, COALESCE(rg_docs.paye,0) AS paye
        FROM documents docs
        INNER JOIN tiers ts ON ts.id=tier_id
        LEFT JOIN (
            SELECT SUM(montant) AS paye, document_id FROM reglement_documents
            GROUP BY document_id
        ) AS rg_docs ON rg_docs.document_id=docs.id
        WHERE docs.id = $1 LIMIT 1
    "#;

    let vente_doc: DocumentEdit = sqlx::query_as::<_, DocumentEdit>(&query)
        .bind(&doc_id)
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    //
     let query_lig = r#"
        SELECT
        lgs.id AS id,
        article_id, code_bar, art.name AS designation, 
        qte AS quantite, prix_achat_ttc, prix_vente_ttc,
        montant_remise, montant_net, stock, uts.name AS unite
        FROM ligne_documents lgs
        INNER JOIN articles art ON art.id=lgs.article_id
        INNER JOIN unites uts ON uts.id=art.unite_id
        WHERE document_id = $1
    "#;

    let lignes: Vec<LigneEdit> = sqlx::query_as::<_, LigneEdit>(&query_lig)
        .bind(&doc_id)
        .fetch_all(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let datas = json!({
        "document":vente_doc,
        "lignes":lignes
    });
    // println!("{:?}",datas);

    Ok((StatusCode::OK, Json(datas)))
}
