use axum::extract::{Query, State};
use sqlx::PgPool;
use crate::{errors::AppError, models::etats::{MvtStockArticle, ParamsStockArticle, ShowStockArticle}};
use axum::response::IntoResponse;

pub async fn stock_avaible(State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        ShowStockArticle,
        r#"
        SELECT code_bar, a.name AS designation, price_buy AS prix_achat, price_seller AS prix_vente,
        COALESCE(SUM(mv.qte_mvt_stock), 0) AS stock
        FROM articles a
        LEFT JOIN ligne_documents mv ON a.id = mv.article_id
        GROUP BY a.id
        ORDER BY stock
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    Ok((axum::http::StatusCode::OK, axum::Json(serde_json::json!({ "datas": result }))))
    
}
pub async fn stock_alert(State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        ShowStockArticle,
        r#"
        SELECT code_bar, a.name AS designation, price_buy AS prix_achat, price_seller AS prix_vente,
        COALESCE(SUM(mv.qte_mvt_stock), 0) AS stock
        FROM articles a
        LEFT JOIN ligne_documents mv ON a.id = mv.article_id
        GROUP BY a.id
        HAVING COALESCE(SUM(mv.qte_mvt_stock), 0) <= alert_stock
        ORDER BY stock
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    Ok((axum::http::StatusCode::OK, axum::Json(serde_json::json!({ "datas": result }))))
    
}
pub async fn mvt_stock_article(State(pool): State<PgPool>,
    Query(_params): Query<ParamsStockArticle>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        MvtStockArticle,
        r#"
        SELECT document_num, document_date, 
        CASE WHEN type_doc = 1 THEN 'Achat' 
             WHEN type_doc = 2 THEN 'Vente' 
             ELSE 'Autre' END AS type_mvt,
        qte_mvt_stock,
        SUM(qte_mvt_stock) OVER (
            ORDER BY document_date, lg.id
            ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        ) AS stock
        FROM ligne_documents lg
        INNER JOIN documents d ON lg.document_id = d.id
        WHERE qte_mvt_stock <> 0 AND lg.article_id = $1
        GROUP BY lg.id, d.document_num, d.document_date, d.type_doc
        ORDER BY document_date
        "#,
        _params.article_id
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::SqlxError)?;
    Ok((axum::http::StatusCode::OK, axum::Json(serde_json::json!({ "datas": result }))))
    
}