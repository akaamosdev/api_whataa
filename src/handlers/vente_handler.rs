use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{ FromRow, PgPool};

use crate::{errors::AppError, models::{document::Document, helper_model::PaginateDocument, ligne_document::{ LigneDocumentShow}}};

#[derive(Deserialize,FromRow,Serialize)]
pub struct VenteShow{
    pub id: String,
    pub document_num:String,
    pub document_date: String,
    pub denomination: String,
    pub montant_tva: f64,
    pub montant_ttc: f64,
    pub montant_net: f64,
    pub montant_remise: f64,
    pub paye: f64,
    pub reste: f64,
}

pub async fn vente_get(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateDocument>,
) -> Result<impl IntoResponse, AppError> {
    let offset = params.offset.unwrap_or(0);

    let mut sqlc = String::from("
        SELECT d.id, document_num, document_date, denomination, montant_tva, montant_ttc, 
               montant_net, montant_remise, COALESCE(t_rd, 0.0) AS paye, montant_net - COALESCE(t_rd, 0.0) AS reste
        FROM documents d
        INNER JOIN clients cls ON cls.id = d.client_id
        LEFT JOIN reglement_documents rd ON rd.document_id = d.id
        LEFT JOIN (
            SELECT document_id, SUM(montant) AS t_rd
            FROM reglement_documents rds
            GROUP BY document_id
        ) AS rds ON rds.document_id = d.id
        WHERE type_doc = ?
    ");

    let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));
    if search_pattern.is_some() {
        sqlc.push_str(
            " AND (denomination LIKE ? 
                   OR document_num LIKE ? 
                   OR document_date LIKE ? 
                   OR montant_net LIKE ?) ",
        );
    }

    sqlc.push_str(" ORDER BY d.created_at DESC LIMIT 25 OFFSET ?");

    // Prépare la requête
    let mut query = sqlx::query_as::<_, VenteShow>(&sqlc);
    query = query.bind(params.type_doc);

    // Bind search si présent
    if let Some(ref pattern) = search_pattern {
        query = query
            .bind(pattern)
            .bind(pattern)
            .bind(pattern)
            .bind(pattern);
    }

    // Bind offset
    query = query.bind(offset);

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
        SELECT documents.*, SUM(ligne_documents.montant_net) AS mont_ht,
        nb_commerce, phone_mobil, address_mail
        FROM documents 
        INNER JOIN clients ON clients.id = documents.client_id
        LEFT JOIN ligne_documents ON ligne_documents.document_id = documents.id
        WHERE documents.id = ?
        GROUP BY documents.id
    "#;

    let vente_doc: Document = sqlx::query_as::<_, Document>(&query)
        .bind(&doc_id)
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    //
     let query_lig = r#"
        SELECT lgs.*, art.code, name, code_bar,is_stock, COALESCE(stock_mvt,0.0) AS stock_mvt
        FROM ligne_documents lgs
        INNER JOIN articles art ON art.id=lgs.article_id
        LEFT JOIN (
            SELECT article_id, SUM(qte_mvt_stock) AS stock_mvt
            FROM ligne_documents s_ligs 
            WHERE document_id !=?
            GROUP BY article_id
        ) AS s_ligs ON s_ligs.article_id=art.id
        WHERE document_id = ?
    "#;

    let lignes: Vec<LigneDocumentShow> = sqlx::query_as::<_, LigneDocumentShow>(&query_lig)
        .bind(&doc_id)
        .bind(&doc_id)
        .fetch_all(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    

    Ok((StatusCode::OK, Json(json!({
        "document":vente_doc,
        "lignes":lignes
    }))))
}
