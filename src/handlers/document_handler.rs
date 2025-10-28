use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::{SqlitePool, query};
use uuid::Uuid;

use crate::{errors::AppError, models::ligne_document::DocumentDto};

pub async fn store_document(
    State(pool): State<SqlitePool>,
    Json(doc): Json<DocumentDto>,
) -> Result<impl IntoResponse, AppError> {
    let mut tx = pool.begin().await.map_err(|e| AppError::SqlxError(e))?;

    let mut  query= String::from("
        INSERT INTO documents (
        document_num, fournisseur_id, client_id, document_date, depot_id, 
        commentaire, type_doc, nombre_article, montant_ttc, taux_remise, montant_remise,
        montant_client, montant_net, montant_tva, montant_airsi, boutique_id, user_id,
        id
    ) 
    VALUES(
     ?, ?, ?, ?, ?, ?,
     ?, ?, ?, ?, ?, ?,
     ?, ?, ?, ?, ?, ?
     )
    ");
    if doc.is_edit==Some(true) {
        delete_ligne_doc(&pool, &doc.id).await?;
        query= String::from("
        UPDATE documents SET
        document_num=?, fournisseur_id=?, client_id=?, document_date=?, depot_id=?, 
        commentaire=?, type_doc=?, nombre_article=?, montant_ttc=?, taux_remise=?, montant_remise=?,
        montant_client=?, montant_net=?, montant_tva=?, montant_airsi=?, boutique_id=?, user_id=?
        WHERE id=?
        ");
    }

    sqlx::query(&query)
        .bind(&doc.document_num)
        .bind(&doc.fournisseur_id)
        .bind(&doc.client_id)
        .bind(&doc.document_date)
        .bind(&doc.depot_id)
        .bind(&doc.commentaire)
        .bind(&doc.type_doc)
        .bind(&doc.nombre_article)
        .bind(&doc.montant_ttc)
        .bind(&doc.taux_remise)
        .bind(&doc.montant_remise)
        .bind(&doc.montant_client)
        .bind(&doc.montant_net)
        .bind(&doc.montant_tva)
        .bind(&doc.montant_airsi)
        .bind(&doc.boutique_id)
        .bind(&doc.user_id)
        .bind(&doc.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    // Insert des lignes
    for lig in doc.ligs {
        sqlx::query!(
            r#"
            INSERT INTO ligne_documents (
                id, document_id, article_id, prix_achat_ttc, prix_vente_ttc, 
                qte, qte_mvt_stock, taux_remise, montant_ttc, montant_net, 
                montant_remise, achever, synchronise
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            lig.id,
            lig.document_id,
            lig.article_id,
            lig.prix_achat_ttc,
            lig.prix_vente_ttc,
            lig.qte,
            lig.qte_mvt_stock,
            lig.taux_remise,
            lig.montant_ttc,
            lig.montant_net,
            lig.montant_remise,
            lig.achever,
            lig.synchronise,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
    }
    if let Some(reg) = &doc.reglement {
        let query_reg = r#"
        INSERT INTO reglements (
            id, user_id, client_id, fournisseur_id, document_id, boutique_id, caisse_id,
            reglement_num, reglement_date, commentaire, montant, mode_paiement_id, 
            reference, synchronise
        )
        VALUES(
            ?, ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?,
            ?, ?
        )
        "#;
        sqlx::query(query_reg)
            .bind(&reg.id)
            .bind(&reg.user_id)
            .bind(&reg.client_id)
            .bind(&reg.fournisseur_id)
            .bind(&reg.document_id)
            .bind(&reg.boutique_id)
            .bind(&reg.caisse_id)
            .bind(&reg.reglement_num)
            .bind(&reg.reglement_date)
            .bind(&reg.commentaire)
            .bind(&reg.montant)
            .bind(&reg.mode_paiement_id)
            .bind(&reg.reference)
            .bind(&reg.synchronise)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::SqlxError(e))?;

        let reg_doc_id = Uuid::new_v4().to_string();
        let rd_query = r#"
        INSERT INTO reglement_documents (
            id, reglement_id, document_id, montant, synchronise
        ) VALUES (
         ?, ?, ?, ?, ?
        )
        "#;
        sqlx::query(rd_query)
        .bind(reg_doc_id)
        .bind(&reg.id)
        .bind(&doc.id)
        .bind(&reg.montant)
        .bind(0)
        .execute(&mut *tx)
        .await.map_err(|e| AppError::SqlxError(e))?;

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
pub async fn delete_ligne_doc(
    pool: &SqlitePool,
    doc_id: &str,
) -> Result<bool, AppError> {
    let query = r#"
        DELETE FROM ligne_documents 
        WHERE document_id = ?
    "#;

    let row_affect  = sqlx::query(query)
        .bind(doc_id)
        .execute(pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok(row_affect.rows_affected()>0)
}