use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, QueryBuilder};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        helper_model::PaginateReglement,
        reglement::{ReglementData, ReglementDetail},
    },
};

pub async fn regle_client(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateReglement>,
) -> Result<impl IntoResponse, AppError> {
    let offset = params.offset;

    let mut query_b = QueryBuilder::<Postgres>::new(
        "SELECT reglements.id, reglement_num, reglement_date,
        montant,denomination, caisses.name AS caisse,
        mode_paiements.name AS mode_pay, mode_paiement_id, tier_id, caisse_id,
        commentaire, reference
        FROM reglements 
        INNER JOIN tiers ON tiers.id=reglements.tier_id
        INNER JOIN caisses ON reglements.caisse_id=caisses.id
        INNER JOIN mode_paiements ON reglements.mode_paiement_id=mode_paiements.id
        WHERE tiers.type_tier = ",
    );
    query_b.push_bind(&params.type_tier);
    if let Some(search) = &params.search {
        let search_pattern = format!("%{}%", search);
        query_b.push(" AND (reglement_num ILIKE ");
        query_b.push_bind(search_pattern.clone());
        query_b.push(" OR denomination ILIKE ");
        query_b.push_bind(search_pattern.clone());
        query_b.push(" OR montant::text ILIKE ");
        query_b.push_bind(search_pattern);
        query_b.push(")");
    }
    if let Some(date_start) = &params.date_start {
        query_b.push(" AND reglement_date >= ");
        query_b.push_bind(date_start);
    }
    if let Some(date_end) = &params.date_end {
        query_b.push(" AND reglement_date <= ");
        query_b.push_bind(date_end);
    }

    query_b.push(" ORDER BY reglements.created_at DESC LIMIT ");
    query_b.push_bind(params.limit);
    query_b.push(" OFFSET ");
    query_b.push_bind(offset);

    let regles: Vec<ReglementDetail> = query_b
        .build_query_as()
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(regles)))
}

pub async fn store_reglement(
    State(pool): State<PgPool>,
    Json(regle): Json<ReglementData>,
) -> Result<impl IntoResponse, AppError> {
    let query_c = "
        INSERT INTO reglements(
            user_id, reglement_num, reglement_date, montant, 
            boutique_id, caisse_id, tier_id, mode_paiement_id,
            commentaire, reference, id
        )
        VALUES ($1, $2, $3::date, $4, $5, $6, $7, $8, $9, $10, $11)
    ";
    if regle.is_edit == Some(true) {
        sqlx::query!(
            r#"
            DELETE FROM reglements WHERE id=$1
        "#,
            regle.id
        )
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    sqlx::query(&query_c)
        .bind(&regle.user_id)
        .bind(regle.reglement_num)
        .bind(regle.reglement_date)
        .bind(regle.montant)
        .bind(regle.boutique_id)
        .bind(regle.caisse_id)
        .bind(&regle.tier_id)
        .bind(regle.mode_paiement_id)
        .bind(regle.commentaire)
        .bind(regle.reference)
        .bind(&regle.id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    // regle doc
    get_docs_client(&pool, &regle.tier_id, &regle.id, regle.montant).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true
        })),
    ))
}

async fn get_docs_client(
    pool: &PgPool,
    client_id: &str,
    reglement_id: &str,
    mut montant_total: f32,
) -> Result<(), AppError> {
    // Démarre une transaction pour assurer la cohérence des écritures
    let mut tx: Transaction<'_, Postgres> = pool
        .begin()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 1️⃣ Récupérer les documents non totalement réglés
    let regle_docs = "
        SELECT 
        documents.id AS doc_id,
        montant_net - COALESCE(regle_docs.montant_doc_regle, 0) AS reste
        FROM documents
        INNER JOIN tiers ON tiers.id = documents.tier_id
        LEFT JOIN (
            SELECT 
                document_id, 
                COALESCE(SUM(montant), 0) AS montant_doc_regle
            FROM reglement_documents
            GROUP BY document_id
        ) AS regle_docs ON regle_docs.document_id = documents.id
        WHERE (type_doc = 2 OR type_doc = 1)
        AND tiers.id = $1
        GROUP BY documents.id, montant_net, regle_docs.montant_doc_regle
        HAVING montant_net - COALESCE(regle_docs.montant_doc_regle, 0) > 0
        ORDER BY documents.created_at ASC;

    ";

    let docs: Vec<(String, f32)> = sqlx::query_as(regle_docs)
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
            VALUES ($1, $2, $3, $4)
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
    pub table_id: String,
    pub table_name: String,
}
// delete regle
pub async fn delete_regle(
    State(pool): State<PgPool>,
    Json(playbod): Json<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let query = "
    DELETE FROM reglement_documents WHERE reglement_id=$1
    ";
    sqlx::query(query)
        .bind(&playbod.table_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let query_c = "
    DELETE FROM reglements WHERE id=$1
    ";
    sqlx::query(query_c)
        .bind(&playbod.table_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK))
}
#[derive(Serialize, Deserialize)]
pub struct RegleDocAuto {
    pub tier_id: String,
    pub document_id: String,
    pub montant_doc: f32,
}

pub async fn get_regle_no_user(
     tx: &mut sqlx::Transaction<'_, Postgres>,
    playbod: RegleDocAuto,
) -> Result<(), AppError> {
    let mut montant_net = playbod.montant_doc;
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
        WHERE (r.montant - COALESCE(rd.montant_alloue, 0)) > 0 AND r.tier_id=$1
        
    ",
    );

    payments_sql.push_str("ORDER BY r.reglement_date ASC");
    let regle_nos = sqlx::query_as::<_, (String, f32)>(
        &payments_sql).bind(&playbod.tier_id);
    //
    let regles = regle_nos
        .fetch_all(&mut **tx)
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
            "INSERT INTO reglement_documents (
            id, reglement_id, document_id, montant)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(reglement_id)
        .bind(&playbod.document_id)
        .bind(montant_regle)
        // either of these two forms is fine:
        .execute(&mut **tx) // explicit: &mut *tx
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        montant_net -= montant_regle;
    }

    Ok(())
}
