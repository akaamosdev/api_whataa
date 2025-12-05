use std::collections::HashMap;

use argon2::password_hash::rand_core::le;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::articles::{Article, ArticleDocument, ArticleShow};
use crate::models::helper_model::PaginateParam;

pub async fn article_paginates(
    State(pool): State<PgPool>,
    Query(params): Query<PaginateParam>,
) -> Result<impl IntoResponse, AppError> {
    let depot_id = params.depot_id.unwrap_or_default();
    let offset = params.offset.unwrap_or(0);

    let mut base_sql = String::from(
        "SELECT
              arts.*,
              fa.name As famille,
              fa.id AS famille_id
          FROM articles arts
          INNER JOIN sous_familles sfa ON sfa.id = arts.sous_famille_id
          INNER JOIN familles fa ON fa.id = sfa.famille_id ",
    );

    let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));

    // Use WHERE for the first condition, AND for subsequent ones
    if search_pattern.is_some() {
        base_sql.push_str(
            " WHERE (arts.code ILIKE $1 OR arts.code_bar ILIKE $2 OR arts.name ILIKE $3)",
        );
        base_sql.push_str(" ORDER BY arts.created_at DESC LIMIT 50 OFFSET $4");
    } else {
        base_sql.push_str(" ORDER BY arts.created_at DESC LIMIT 50 OFFSET $1");
    }

    let mut query = sqlx::query_as::<_, ArticleShow>(&base_sql);

    // 2. Bind search parameters if they exist
    if let Some(ref pattern) = search_pattern {
        query = query.bind(pattern).bind(pattern).bind(pattern).bind(offset);
    } else {
        query = query.bind(offset);
    }

    // 3. Bind offset for pagination

    let articles: Vec<ArticleShow> = query
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(articles)))
}

pub async fn article_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let sqlc = format!("SELECT * FROM articles WHERE id=$1");
    let article: Article = sqlx::query_as(&sqlc)
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(article)))
}

pub async fn article_add(
    State(pool): State<PgPool>,
    Json(payload): Json<Article>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    INSERT INTO articles (
        id, code, code_bar, name, sous_famille_id, 
        marque_id, unite_id, alert_stock, 
        is_stock, boutique_id, price_buy,
        price_seller, stock
    )
    VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8,
        $9, $10, $11, $12, $13
    )
    ",
    );

    sqlx::query(&query)
        .bind(&payload.id)
        .bind(&payload.code)
        .bind(&payload.code_bar)
        .bind(&payload.name)
        .bind(&payload.sous_famille_id)
        .bind(&payload.marque_id)
        .bind(&payload.unite_id)
        .bind(&payload.alert_stock)
        .bind(&payload.is_stock)
        .bind(&payload.boutique_id)
        .bind(&payload.price_buy)
        .bind(&payload.price_seller)
        .bind(&payload.stock)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    // Initialize stock document if stock is provided
    let mut doc_default_id = payload.doc_defaut_id;
    if payload.stock != 0.0 {
        if doc_default_id.is_empty() {
            let date_now = chrono::Utc::now().date_naive().to_string();
            let query = "
        INSERT INTO documents (
            document_num, tier_id, document_date, depot_id,
            commentaire, type_doc, nombre_article, montant_ttc,
            montant_net, boutique_id, user_id, id
        )
        VALUES (
            $1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12
        )
    ";
            doc_default_id = Uuid::new_v4().to_string();
            sqlx::query(&query)
                .bind("MVIS000")
                .bind(None::<String>)
                .bind(&date_now)
                .bind(&payload.depot_defaut_id.unwrap_or_default())
                .bind("Initial stock document")
                .bind(31) // assuming 2 is the type for stock initialization
                .bind(1) // one article
                .bind(&payload.price_seller)
                .bind(&payload.price_seller)
                .bind(&payload.boutique_id)
                .bind(&payload.user_id)
                .bind(&doc_default_id)
                .execute(&pool)
                .await
                .map_err(AppError::SqlxError)?;
        }

        // Insert des lignes
        sqlx::query!(
            r#"
        INSERT INTO ligne_documents (
            id, document_id, article_id, prix_achat_ttc, prix_vente_ttc, 
            qte, qte_mvt_stock, montant_ttc, montant_net
        ) 
        VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9
        )
        "#,
            Uuid::new_v4().to_string(),
            &doc_default_id,
            &payload.id,
            &payload.price_buy,
            &payload.price_seller,
            &payload.stock,
            &payload.stock,
            &payload.price_buy * &payload.stock,
            &payload.price_buy * &payload.stock
        )
        .execute(&pool)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
    }

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "article enregistré avec succès",
            "doc_defaut_id": &doc_default_id
        })),
    ))
}

pub async fn article_update(
    State(pool): State<PgPool>,
    Json(payload): Json<Article>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    UPDATE articles SET code=$1, code_bar=$2, name=$3, sous_famille_id=$4,
    marque_id=$5, unite_id=$6, alert_stock=$7, is_stock=$8, price_buy=$9,
    price_seller=$10
        WHERE id=$11
    ",
    );

    sqlx::query(&query)
        .bind(&payload.code)
        .bind(&payload.code_bar)
        .bind(&payload.name)
        .bind(&payload.sous_famille_id)
        .bind(&payload.marque_id)
        .bind(&payload.unite_id)
        .bind(&payload.alert_stock)
        .bind(&payload.is_stock)
        .bind(payload.price_buy)
        .bind(payload.price_seller)
        .bind(&payload.id)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": "article modifié avec succès"
        })),
    ))
}
#[derive(serde::Deserialize)]
pub struct ArticleSearchParams {
    pub name: Option<String>,
    pub code_bar: Option<String>,
}
//
#[derive(serde::Deserialize)]
pub struct ArticleInfo {
    pub id: String,
    pub designation: String,
    pub stock: f32,
}
#[derive(serde::Deserialize)]
pub struct ArticleCheckStock {
    pub article: Vec<ArticleInfo>,
}
// get article documents
pub async fn article_documents(
    State(pool): State<PgPool>,
    Query(params): Query<ArticleSearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let documents = if let Some(name) = params.name {
        sqlx::query_as::<_, ArticleDocument>(
            "SELECT id, code, code_bar, name, price_seller, price_buy, stock
             FROM articles
             WHERE name ILIKE $1
             ORDER BY name
             LIMIT 20",
        )
        .bind(format!("%{}%", name))
        .fetch_all(&pool)
        .await?
    } else if let Some(code_bar) = params.code_bar {
        sqlx::query_as::<_, ArticleDocument>(
            "SELECT id, code, code_bar, name, price_seller, price_buy, stock
             FROM articles
             WHERE code_bar = $1",
        )
        .bind(code_bar) // code_bar est une String → OK
        .fetch_all(&pool)
        .await?
    } else {
        vec![] // aucun param → retourne vide
    };

    Ok((StatusCode::OK, Json(documents)))
}
//
pub async fn article_check_stock(
    State(pool): State<PgPool>,
    Json(params): Json<ArticleCheckStock>,
) -> Result<impl IntoResponse, AppError> {
    for art in params.article {
        let article = sqlx::query_as::<_, ArticleDocument>(
            "SELECT name, stock
             FROM articles
             WHERE id=$1 AND stock>$2",
        )
        .bind(&art.id)
        .bind(&art.stock)
        .fetch_optional(&pool)
        .await?;
        if article.is_none() {
            return Ok((
                StatusCode::OK,
                Json(json!({
                    "statut":false,
                    "designation":art.designation,
                    "stock":art.stock
                })),
            ));
        }
    }

    Ok((StatusCode::OK, Json(json!({}))))
}
