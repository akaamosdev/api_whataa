use std::any::Any;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError, handlers::reglement_handler::get_regle_no_user,
    models::ligne_document::DocumentDto,
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
        montant_total, doc_parent_id,
        id
        ) 
        VALUES(
        $1, $2, $3, $4,
        $5, $6, $7, $8, $9,
        $10, $11, $12, $13, $14, $15, $16, $17, $18
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
        user_id = $15, montant_total= $16, doc_parent_id= $17 WHERE id = $18
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
                montant_remise
            ) 
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10
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
            lig.montant_remise
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
    // if doc.doc_parent_id.is_some() {
    //     sqlx::query!(r#"
    //     UPDATE documents SET doc
    //     "#)
    // }

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
