use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::{Execute, PgPool};
use uuid::Uuid;

use crate::{
    errors::AppError,
    handlers::reglement_handler::{DeletePayload, RegleDocAuto, get_regle_no_user},
    models::{
        document::{ ApprouveParam, StockList, StockParam},
        ligne_document::{DocumentDto, LigneResetStock},
    },
};

pub async fn store_document(
    State(pool): State<PgPool>,
    Json(doc): Json<DocumentDto>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = pool.begin().await.map_err(|e| AppError::SqlxError(e))?;

    let mut query = String::from(
        "
        INSERT INTO documents (
        document_num, tier_id, document_date, depot_id, 
        commentaire, type_doc, montant_ht, taux_remise, montant_remise,
        montant_client, montant_net, montant_tva, montant_airsi, boutique_id, user_id,
        montant_total, doc_parent_id, attente,
        id
        ) 
        VALUES(
        $1, $2, $3, $4,
        $5, $6, $7, $8, $9,
        $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
        )
    ",
    );
    if doc.is_edit == Some(true) {
        delete_ligne_doc(&pool, &doc.id).await?;
        query = String::from(
            "
        UPDATE documents SET
        document_num = $1, tier_id = $2, document_date = $3, depot_id = $4,
        commentaire = $5, type_doc = $6, montant_ht = $7, taux_remise = $8, montant_remise = $9,
        montant_client = $10, montant_net = $11, montant_tva = $12, montant_airsi = $13, boutique_id = $14,
        user_id = $15, montant_total= $16, doc_parent_id= $17, attente= $18 WHERE id = $19
        ",
        );
    }

    sqlx::query(&query)
        .bind(&doc.document_num)
        .bind(&doc.tier_id)
        .bind(&doc.document_date)
        .bind(&doc.depot_id)
        .bind(&doc.commentaire)
        .bind(&doc.type_doc)
        .bind(&doc.montant_ht)
        .bind(&doc.taux_remise)
        .bind(&doc.montant_remise)
        .bind(&doc.montant_client)
        .bind(&doc.montant_net)
        .bind(&doc.montant_tva)
        .bind(&doc.montant_airsi)
        .bind(&doc.boutique_id)
        .bind(&doc.user_id)
        .bind(&doc.montant_total)
        .bind(&doc.doc_parent_id)
        .bind(&doc.attente)
        .bind(&doc.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    // Insert des lignes
    for lig in doc.lignes {
        sqlx::query!(
            r#"
            INSERT INTO ligne_documents (
                id, document_id, article_id, prix_achat_ttc, prix_vente_ttc, 
                qte, qte_mvt_stock, montant_ttc, montant_net, 
                montant_remise, qte_last_stock
            ) 
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10, $11
            )
            "#,
            lig.id,
            lig.document_id,
            lig.article_id,
            lig.prix_achat_ttc,
            lig.prix_vente_ttc,
            lig.qte,
            lig.qte_mvt_stock,
            lig.montant_ttc,
            lig.montant_net,
            lig.montant_remise,
            lig.qte_last_stock
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

        sqlx::query!(
            r#"
        UPDATE articles SET 
        stock = stock + $1
        WHERE id = $2
        "#,
            &lig.qte_mvt_stock,
            &lig.article_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
    }
    if let Some(reg) = &doc.reglement {
        let query_reg = r#"
        INSERT INTO reglements (
            id, user_id, tier_id, boutique_id, caisse_id,
            reglement_num, reglement_date, commentaire, montant, mode_paiement_id, 
            reference
        )
        VALUES(
        $1, $2, $3, $4, $5,
        $6, $7::date, $8, $9, $10, $11
        )
        "#;

        sqlx::query(query_reg)
            .bind(&reg.id)
            .bind(&reg.user_id)
            .bind(&reg.tier_id)
            .bind(&reg.boutique_id)
            .bind(&reg.caisse_id)
            .bind(&reg.reglement_num)
            .bind(&reg.reglement_date)
            .bind(&reg.commentaire)
            .bind(&reg.montant)
            .bind(&reg.mode_paiement_id)
            .bind(&reg.reference)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::SqlxError(e))?;

        let reg_doc_id = Uuid::new_v4().to_string();
        let rd_query = r#"
        INSERT INTO reglement_documents (
            id, reglement_id, document_id, montant
        ) VALUES (
            $1, $2, $3, $4
        )
        "#;
        sqlx::query(rd_query)
            .bind(reg_doc_id)
            .bind(&reg.id)
            .bind(&doc.id)
            .bind(&reg.montant)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::SqlxError(e))?;
    }
    if doc.is_edit.is_none()&&(doc.type_doc==2||doc.type_doc==1) {
        get_regle_no_user(&mut tx, RegleDocAuto{
            document_id: doc.id,
            tier_id: doc.tier_id.clone().unwrap_or_default(),
            montant_doc: doc.montant_net
        }).await?;
    }

    tx.commit().await.map_err(|e| AppError::SqlxError(e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut":true,
            "message":"Document enregistré avec succès"
        })),
    ))
}

pub async fn delete_ligne_doc(pool: &PgPool, doc_id: &str) -> Result<bool, AppError> {
    let query = r#"
        DELETE FROM ligne_documents 
        WHERE document_id = $1
    "#;

    let row_affect = sqlx::query(query)
        .bind(doc_id)
        .execute(pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok(row_affect.rows_affected() > 0)
}

// -----------------------------stock functions-----------------------------
pub async fn stock_get(
    State(pool): State<PgPool>,
    Query(params): Query<StockParam>,
) -> Result<impl IntoResponse, AppError> {
    let mut query = String::from(
        r#"
        SELECT 
        docs.id,type_doc, document_num, document_date, depot_id,
         montant_total,montant_net, qte_total, doc_fils_id
        FROM documents docs
        LEFT JOIN (
            SELECT document_id, SUM(qte) AS qte_total
            FROM ligne_documents
            GROUP BY document_id
        ) AS lignes ON docs.id = lignes.document_id
        WHERE type_doc=$1
        
    "#,
    );
    // date filter
    let mut idx = 2;
    let search_clean = params.search.as_ref().and_then(|s| {
        let s = s.trim();
        if s.is_empty() {
            None
        } else {
            Some(format!("%{}%", s))
        }
    });
    if params.date_start.is_some() {
        query += &format!(" AND document_date >= ${} ", idx);
        idx += 1;
    }
    if params.date_end.is_some() {
        query += &format!(" AND document_date <= ${} ", idx);
        idx += 1;
    }
    if search_clean.is_some() {
        query += &format!(" AND (document_num ILIKE ${} ) ", idx);
        idx += 1;
    }
    query += &format!(
        " ORDER BY document_date DESC LIMIT ${} OFFSET ${} ",
        idx + 1,
        idx
    );

    let mut query_ex = sqlx::query_as::<_, StockList>(&query);
    query_ex = query_ex.bind(params.type_doc);

    if let Some(date_start) = params.date_start {
        query_ex = query_ex.bind(date_start);
    }
    if let Some(date_end) = params.date_end {
        query_ex = query_ex.bind(date_end);
    }
    if let Some(search) = search_clean {
        let pattern = format!("%{}%", search);
        query_ex = query_ex.bind(pattern);
    }
    query_ex = query_ex.bind(params.offset).bind(params.limit);

    let stock_docs: Vec<StockList> = query_ex
        .fetch_all(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, Json(stock_docs)))
}
// -----------------------------vente functions-----------------------------
pub async fn doc_delete(
    State(pool): State<PgPool>,
    Json(param): Json<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let query = format!("
        DELETE FROM {} 
        WHERE id = $1",param.table_name);

    let row_affect = sqlx::query(&query)
        .bind(&param.table_id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    if row_affect.rows_affected() == 0 {
        return Err(AppError::Internal("Document not found".to_string()));
    }


    Ok((
        StatusCode::OK,
        Json(json!({
            "statut":true,
            "message":"Document supprimé avec succès"
        })),
    ))
}
// -----------------------------approuve stock-----------------------------

pub async fn stock_ajuste(
    State(pool): State<PgPool>,
    Json(param): Json<ApprouveParam>
) -> Result<impl IntoResponse, AppError> {
    let mut tx = pool.begin().await.map_err(|e| AppError::SqlxError(e))?;  
    let lignes =sqlx::query_as::<_, LigneResetStock>(r#"
        SELECT article_id, qte, qte_last_stock
        FROM ligne_documents
        WHERE document_id = $1;    
    "#)
    .bind(&param.document_id)
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    for lig in lignes {
        sqlx::query!(r#"
        UPDATE articles SET 
        stock = $1
        WHERE id = $2
        "#,
        &lig.qte,   
        &lig.article_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
    //reset qte_last_stock to qte
        sqlx::query!(r#"
        UPDATE ligne_documents SET 
        qte_mvt_stock = qte_mvt_stock - $1 + $2
        WHERE article_id = $3 AND document_id = $4
        "#,
        &lig.qte_last_stock,   
        &lig.qte,
        &lig.article_id,
        &param.document_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
        
    }
    //approve the document reset
     sqlx::query!(r#"
        UPDATE documents SET 
        doc_fils_id = $1
        WHERE id = $2
        "#,
        &param.document_id,   
        &param.document_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    tx.commit().await.map_err(|e| AppError::SqlxError(e))?;

    Ok((StatusCode::OK, Json(json!({
        "statut":true,
        "message":"Stock approuvé avec succès"
    }))))
   
}


