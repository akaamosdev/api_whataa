use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use chrono::Local;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::tier::Tier;
use crate::models::document::Document;
use crate::models::helper_model::PaginateParam;

pub async fn client_paginates(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateParam>,
) -> Result<impl IntoResponse, AppError> {
    let mut sqlc = format!(
        "
SELECT 
    cls.*, 
    CAST(COALESCE(montant_net,0) AS REAL) 
    - CAST(COALESCE(sum_retour,0) AS REAL) 
    - CAST(COALESCE(montant,0) AS REAL) AS solde,
     0.0 AS solde_initial
    FROM clients cls
LEFT JOIN (
    SELECT client_id, SUM(montant_net) AS montant_net FROM documents
    WHERE documents.type_doc = 2
    GROUP BY client_id
  ) AS t_documents ON t_documents.client_id=cls.id 

LEFT JOIN (
    SELECT client_id, SUM(montant_net) AS sum_retour FROM documents retours
    WHERE retours.type_doc = 23
    GROUP BY client_id
  ) AS retours ON retours.client_id=cls.id 

LEFT JOIN (
    SELECT client_id, SUM(montant) AS montant
    FROM reglements
    GROUP BY client_id
  ) AS t_reglements ON t_reglements.client_id=cls.id 
"
    );
    let mut search_pattern: Option<String> = None;

    if let Some(search) = &params.search {
        sqlc.push_str(" WHERE code LIKE ? OR denomination LIKE ? OR phone_mobil LIKE ?");
    }

    sqlc.push_str(" GROUP BY cls.id ORDER BY created_at DESC LIMIT 25 OFFSET ?");

    let mut q = sqlx::query_as::<_, Tier>(&sqlc);

    if let Some(search) = &params.search {
        search_pattern = Some(format!("%{}%", search));
        q = q
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern);
    }

    q = q.bind(params.offset);

    let fournisseurs: Vec<Tier> = q
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(fournisseurs)))
}

pub async fn client_add(
    State(pool): State<PgPool>,
    Json(payload): Json<Tier>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        " INSERT INTO clients (
        id, code, denomination,nb_commerce,nb_contribuable,address_phy,boite_postale,
        phone_fix,phone_mobil,faxe,address_mail,boutique_id,
        synchronise
    )
    VALUES (
        ?, ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?, ?,
        ?
    )
    ",
    );

    sqlx::query(&query)
        .bind(&payload.id)
        .bind(&payload.code)
        .bind(&payload.denomination)
        .bind(&payload.nb_commerce)
        .bind(&payload.nb_contribuable)
        .bind(&payload.address_phy)
        .bind(&payload.boite_postale)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(&payload.faxe)
        .bind(&payload.address_mail)
        .bind(&payload.boutique_id)
        .bind(payload.synchronise)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    //

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "Client enregistré avec succès"
        })),
    ))
}
pub async fn store_solde_initial(
     State(pool): State<PgPool>,
    Json(payload): Json<Document>,
) -> Result<impl IntoResponse, AppError> {

    let query = r#"
        INSERT INTO documents (
            id, document_num, client_id, type_doc, nombre_article, montant_net, montant_ttc,
            document_date, depot_id, boutique_id, synchronise
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

    sqlx::query(query)
        .bind(&payload.id)
        .bind(&payload.document_num)
        .bind(&payload.client_id)
        .bind(2)
        .bind(1.0)
        .bind(&payload.montant_net)
        .bind(&payload.montant_ttc)
        .bind(&payload.document_date) // ✅ pass as string
        .bind(&payload.depot_id)
        .bind(&payload.boutique_id) // example: synchronise field, adjust as needed
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    
    Ok((StatusCode::OK,Json(json!({
        "statut":true,
        "message":"Solde initial"
    }))))
}

pub async fn client_update(
    State(pool): State<PgPool>,
    Json(payload): Json<Tier>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    UPDATE clients 
        SET code=?, denomination=?, nb_commerce=?, nb_contribuable=?, address_phy=?, boite_postale=?,
        phone_fix=?,phone_mobil=?,faxe=?,address_mail=?,boutique_id=?,
        synchronise=?
        WHERE id=?
    ",
    );

    sqlx::query(&query)
        .bind(&payload.code)
        .bind(&payload.denomination)
        .bind(&payload.nb_commerce)
        .bind(&payload.nb_contribuable)
        .bind(&payload.address_phy)
        .bind(&payload.boite_postale)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(&payload.faxe)
        .bind(&payload.address_mail)
        .bind(&payload.boutique_id)
        .bind(&payload.synchronise)
        .bind(&payload.id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "Client modifié avec succès"
        })),
    ))
}
//zz
