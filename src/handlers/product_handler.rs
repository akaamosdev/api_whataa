use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::articles::{Article, ArticleShow};
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
              fa.id AS famille_id,
              COALESCE(ld.stock_mvt, 0.0) AS stock_mvt
          FROM articles arts
          INNER JOIN sous_familles sfa ON sfa.id = arts.sous_famille_id
          INNER JOIN familles fa ON fa.id = sfa.famille_id
          LEFT JOIN (
              SELECT
                  ld.article_id,
                  SUM(ld.qte_mvt_stock) AS stock_mvt
              FROM ligne_documents ld
              INNER JOIN documents ds ON ds.id=ld.document_id
              WHERE ds.depot_id=?
              GROUP BY ld.article_id
          ) AS ld ON ld.article_id = arts.id"
      );

      let search_pattern = params.search.as_ref().map(|s| format!("%{}%", s));

      // Use WHERE for the first condition, AND for subsequent ones
      if search_pattern.is_some() {
          base_sql.push_str(" WHERE (arts.code LIKE ? OR arts.code_bar LIKE ? OR arts.name LIKE ?)");
      }

      base_sql.push_str("GROUP BY arts.id ORDER BY arts.created_at DESC LIMIT 25 OFFSET ?");

      let mut query = sqlx::query_as::<_, ArticleShow>(&base_sql);

      // 1. Bind depot_id for the subquery
      query = query.bind(depot_id);

      // 2. Bind search parameters if they exist
      if let Some(ref pattern) = search_pattern {
          query = query.bind(pattern).bind(pattern).bind(pattern);
      }

      // 3. Bind offset for pagination
      query = query.bind(offset);

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
    let sqlc = format!("SELECT * FROM articles WHERE id=?");
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
        ?, ?, ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?, ?, 
        ?
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
        .bind(payload.boutique_id)
        .bind(payload.price_buy)
        .bind(payload.price_seller)
        .bind(payload.stock)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "message": "article enregistré avec succès"
        })),
    ))
}
//
pub async fn article_update(
    State(pool): State<PgPool>,
    Json(payload): Json<Article>,
) -> Result<impl IntoResponse, AppError> {
    let query: String = String::from(
        "
    UPDATE articles SET code=?, code_bar = ?, name=?, sous_famille_id=?, 
        marque_id=?, unite_id=?, alert_stock=?, 
        is_stock=?, price_buy=?,price_seller=?
        WHERE id=?
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
