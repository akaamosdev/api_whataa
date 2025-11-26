use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Postgres, Transaction};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        helper_model::{PaginateDocument, PaginateReglement},
        reglement,
    },
};
#[derive(Serialize, FromRow)]
pub struct DocReste {
    doc_id: String,
    reste: f64,
}

#[derive(Serialize, FromRow)]
pub struct ReglementDetail {
    id: String,
    reglement_num: String,
    reglement_date: String,
    montant: f64,
    denomination: String,
    caisse: String,
    mode_pay: String,
    client_id: String,
    caisse_id: String,
    mode_paiement_id: String,
    commentaire: String,
    reference: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ReglementData {
    id: String,
    reglement_num: String,
    reglement_date: String,
    montant: f64,
    boutique_id: String,
    caisse_id: String,
    fournisseur_id: Option<String>,
    client_id: Option<String>,
    mode_paiement_id: String,
    commentaire: String,
    reference: String,
    is_edit: Option<bool>,
}

pub async fn regle_client(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateReglement>,
) -> Result<impl IntoResponse, AppError> {
    let offset = params.offset.unwrap_or(0);
    let mut sqlc = String::from(
        "SELECT reglements.id, reglement_num, reglement_date,
        montant,clients.denomination, caisses.name AS caisse,
        mode_paiements.name AS mode_pay, mode_paiement_id, client_id, caisse_id,
        commentaire, reference
        FROM reglements 
        INNER JOIN clients ON clients.id=reglements.client_id
        INNER JOIN caisses ON reglements.caisse_id=caisses.id
        INNER JOIN mode_paiements ON reglements.mode_paiement_id=mode_paiements.id
        ",
    );
    let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));
    if search_pattern.is_some() {
        sqlc.push_str(
            " AND (reglement_num LIKE ? 
                   OR reglement_date LIKE ? 
                   OR denomination LIKE ? 
                   OR montant LIKE ?) ",
        );
    }
    sqlc.push_str(
        "
    GROUP BY reglements.id ORDER BY reglements.reglement_num DESC LIMIT 25 OFFSET ?
    ",
    );

    let mut query = sqlx::query_as::<_, ReglementDetail>(&sqlc);
    if let Some(ref pattern) = search_pattern {
        query = query
            .bind(pattern)
            .bind(pattern)
            .bind(pattern)
            .bind(pattern)
    }
    query = query.bind(offset);
    let regles: Vec<ReglementDetail> = query
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok((StatusCode::OK, Json(regles)))
}

pub async fn store_reglement(
    State(pool): State<PgPool>,
    Json(regle): Json<ReglementData>,
) -> Result<impl IntoResponse, AppError> {
    let mut query_c = "
        INSERT INTO reglements(
            reglement_num, reglement_date, montant, 
            boutique_id, caisse_id, fournisseur_id, client_id, mode_paiement_id,
            commentaire, reference, id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ";
    if regle.is_edit == Some(true) {
        query_c = "
            UPDATE reglements SET reglement_num=?, reglement_date=?, montant=?, 
            boutique_id=?, caisse_id=?, fournisseur_id=?, client_id=?, mode_paiement_id=?,
            commentaire=?, reference=? WHERE id=?
        ";
    }

    sqlx::query(&query_c)
        .bind(regle.reglement_num)
        .bind(regle.reglement_date)
        .bind(regle.montant)
        .bind(regle.boutique_id)
        .bind(regle.caisse_id)
        .bind(regle.fournisseur_id)
        .bind(&regle.client_id)
        .bind(regle.mode_paiement_id)
        .bind(regle.commentaire)
        .bind(regle.reference)
        .bind(&regle.id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    // regle doc
    if let Some(client_id) = &regle.client_id {
        get_docs_client(&pool, client_id, &regle.id, regle.montant).await?;
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true
        })),
    ))
}

async fn get_docs_client(
    pool: &PgPool,
    client_id: &str,
    reglement_id: &str,
    mut montant_total: f64,
) -> Result<(), AppError> {
    // Démarre une transaction pour assurer la cohérence des écritures
    let mut tx: Transaction<'_, Postgres> = pool
        .begin()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 1️⃣ Récupérer les documents non totalement réglés
    let regle_docs = "
        SELECT documents.id AS doc_id,
               documents.montant_net - COALESCE(regle_docs.montant_doc_regle, 0) AS reste
        FROM documents
        INNER JOIN clients ON clients.id = documents.client_id
        LEFT JOIN (
            SELECT reglement_documents.document_id,
                   COALESCE(SUM(reglement_documents.montant), 0) AS montant_doc_regle
            FROM reglement_documents
            GROUP BY reglement_documents.document_id
        ) AS regle_docs ON regle_docs.document_id = documents.id
        WHERE type_doc = 2 AND clients.id = ?
        GROUP BY documents.id
        HAVING reste > 0
        ORDER BY documents.created_at ASC
    ";

    let docs: Vec<(String, f64)> = sqlx::query_as(regle_docs)
        .bind(client_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 2️⃣ Parcourir les factures et les régler partiellement ou totalement
    for (doc_id, reste) in docs {
        if montant_total <= 0.0 {
            break;
        }

        let montant_regle = if montant_total >= reste {
            reste
        } else {
            montant_total
        };

        // 3️⃣ Insérer la ligne dans reglement_documents
        sqlx::query(
            "
            INSERT INTO reglement_documents ( id, reglement_id, document_id, montant)
            VALUES (?, ?, ?, ?)
        ",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(reglement_id)
        .bind(doc_id)
        .bind(montant_regle)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        montant_total -= montant_regle;
    }

    // 4️⃣ Validation de la transaction
    tx.commit()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(())
}
#[derive(Deserialize)]
pub struct DeletePayload {
    pub regle_id: String,
}
// delete regle
pub async fn delete_regle(
    State(pool): State<PgPool>,
    Json(playbod): Json<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let query = "
    DELETE FROM reglement_documents WHERE reglement_id=?
    ";
    sqlx::query(query)
        .bind(&playbod.regle_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let query_c = "
    DELETE FROM reglements WHERE id=?
    ";
    sqlx::query(query_c)
        .bind(&playbod.regle_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK))
}
#[derive(Serialize)]
pub struct RegleDocAuto {
    client_id: Option<String>,
    fournisseur_id: Option<String>,
    document_id: String,
    montant_doc: f64,
}

pub async fn get_regle_no_user(
    State(pool): State<PgPool>,
    Json(playbod): Json<RegleDocAuto>,
) -> Result<(), AppError> {
    let mut montant_net = playbod.montant_doc;
    let mut client_id = String::new();
    let mut fournisseur_id = String::new();
    let mut payments_sql = String::from(
        "
        SELECT r.id AS reglement_id,
               r.montant - COALESCE(rd.montant_alloue, 0) AS reste
        FROM reglements r
        LEFT JOIN (
            SELECT reglement_id, COALESCE(SUM(montant), 0) AS montant_alloue
            FROM reglement_documents
            GROUP BY reglement_id
        ) rd ON rd.reglement_id = r.id
        WHERE (r.montant - COALESCE(rd.montant_alloue, 0)) > 0
        
    ",
    );
    
    if let Some(client) = playbod.client_id {
        client_id=client;
        payments_sql.push_str(" AND r.client_id = ?");
       
    }
    if let Some(fournisseur) = playbod.fournisseur_id {
        fournisseur_id=fournisseur;
        payments_sql.push_str(" AND r.fournisseur_id = ?");
    }

    payments_sql.push_str("ORDER BY r.reglement_date ASC");
    let mut regle_nos = sqlx::query_as::<_, (String, f64)>(&payments_sql);
    
    if !client_id.is_empty() {
        regle_nos=regle_nos.bind(client_id);
    }
    //
   let regles  = regle_nos
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    for (reglement_id, reste) in regles {
        if montant_net <= 0.0 {
            break;
        }

        let montant_regle = if montant_net >= reste {
            reste
        } else {
            montant_net
        };

        sqlx::query(
            "INSERT INTO reglement_documents (id, reglement_id, document_id, montant)
             VALUES (?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(reglement_id)
        .bind(&playbod.document_id)
        .bind(montant_regle)
        // either of these two forms is fine:
        .execute(&pool) // explicit: &mut *tx
        //.execute(tx)      // or just pass tx (a &mut Transaction)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        montant_net -= montant_regle;
    }

    Ok(())
}
