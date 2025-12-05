use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct SendDataDto {
    pub user_id: String,
    pub table: String,
    // Expect datas to be a JSON array in the request body
    pub datas: Vec<Value>,
}

// Whitelist of tables allowed to be synced. This prevents SQL injection via table names.
const ALLOWED_TABLES: &[&str] = &[
    "articles", "users", "roles", "boutiques", "depots", "tiers", "documents",
    "mode_paiements", "caisses", "reglements", "ligne_documents", "familles",
    "sous_familles", "marques", "unites", "depenses", "type_depenses",
    "compagnies", "admins", "reglement_documents",
];

pub async fn send_data(
    State(pool): State<PgPool>,
    Json(payload): Json<SendDataDto>,
) -> Result<impl IntoResponse, AppError> {
    // Validate table name
    let table = payload.table.trim();
    if !ALLOWED_TABLES.contains(&table) {
        return Err(AppError::BadRequest(format!("Table '{}' is not allowed", table)));
    }

    let items = payload.datas;
    if items.is_empty() {
        return Err(AppError::BadRequest("Empty datas array".to_string()));
    }

    // Start transaction
    let mut tx = pool.begin().await.map_err(|e| AppError::SqlxError(e))?;

    // Get columns for the table from information_schema
    let cols: Vec<String> = sqlx::query_scalar(
        r#"SELECT column_name FROM information_schema.columns WHERE table_schema='public' AND table_name=$1"#,
    )
    .bind(table)
    .fetch_all(&mut *tx)
    .await
    .map_err(AppError::SqlxError)?;

    if cols.is_empty() {
        return Err(AppError::BadRequest(format!("Table '{}' not found or has no columns", table)));
    }

    // Build update set excluding id column
    let update_cols: Vec<String> = cols.iter().filter(|c| *c != "id").cloned().collect();
    let update_set = if !update_cols.is_empty() {
        update_cols
            .iter()
            .map(|c| format!("{} = EXCLUDED.{}", c, c))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        // nothing to update
        "id = EXCLUDED.id".to_string()
    };

    // For each item, insert using json_populate_record to map JSON keys to columns and upsert on id
    for mut item in items {
        // Ensure id exists; if not, generate one
        if !item.get("id").is_some() || item.get("id").unwrap().is_null() {
            let new_id = Uuid::new_v4().to_string();
            if let Value::Object(ref mut m) = item {
                m.insert("id".to_string(), Value::String(new_id));
            }
        }

    let json_str = serde_json::to_string(&item).map_err(|e| AppError::BadRequest(e.to_string()))?;

        let insert_sql = format!(
            "INSERT INTO {table} SELECT * FROM json_populate_record(NULL::public.{table}, $1::json) ON CONFLICT (id) DO UPDATE SET {update}",
            table = table,
            update = update_set
        );

        sqlx::query(&insert_sql)
            .bind(json_str)
            .execute(&mut *tx)
            .await
            .map_err(AppError::SqlxError)?;
    }

    tx.commit().await.map_err(AppError::SqlxError)?;

    Ok((StatusCode::OK, "Data synchronized"))
}
