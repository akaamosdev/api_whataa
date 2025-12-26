use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{FromRow, PgPool};

use crate::{
    errors::AppError,
    models::user::{self, User, UserPlayload, UserShow},
};

pub async fn check_database(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(id) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut":count>0,
        })),
    ))
}
#[derive(Serialize, FromRow)]
pub struct TiersData {
    pub id: String,
    pub name: String,
}
pub async fn all_tiers(
    State(pool): State<PgPool>,
    Path(type_tier): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let sqlc = format!(
        "
    SELECT id, denomination AS name FROM tiers 
    WHERE type_tier=$1
    ORDER BY denomination ASC"
    );
    let familles: Vec<TiersData> = sqlx::query_as(&sqlc)
        .bind(type_tier)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(familles)))
}
pub async fn all_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let users = sqlx::query_as!(
        UserShow,
        "
        SELECT us.id, us.name, email, phone, role_id::bigint, roles.name AS role FROM users us
        INNER JOIN roles ON us.role_id = roles.id
        ORDER BY us.created_at ASC
        "
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(users)))
}
pub async fn role_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let roles = sqlx::query_as!(
        user::Roles,
        "
        SELECT id::bigint, name FROM roles 
        ORDER BY created_at DESC
        "
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(roles)))
}
pub async fn store_new_user(
    State(pool): State<PgPool>,
    Json(play): Json<UserPlayload>,
) -> Result<impl IntoResponse, AppError> {
    let argon2 = Argon2::default();

    // Hasher le mot de passe
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(play.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();
    let mut role_id: i32 = play.role_id;
    if let Some(new_role) = play.new_role {
        println!("Creating new role: {}", new_role);
        role_id = sqlx::query_scalar(
            "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id",
        )
        .bind(&new_role)
        .bind(&new_role)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        if play.privileges.is_some() {
            sqlx::query(
                "INSERT INTO permission_role (
                role_id, permission_id) 
                VALUES ($1, UNNEST($2::bigint[])
                )",
            )
            .bind(role_id)
            .bind(play.privileges.unwrap())
            .execute(&pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        }
    }
    if play.id_edit.is_empty() {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: play.email,
            password_hash,
            name: play.name,
            phone: play.phone,
            role_id,
            boutique_id: play.boutique_id,
        };
        sqlx::query(
            "INSERT INTO users (
        id, email, password_hash, 
        created_at,name,role_id,boutique_id, phone) 
        VALUES ($1, $2, $3, NOW(), $4, $5, $6, $7)",
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.role_id)
        .bind(&user.boutique_id)
        .bind(&user.phone)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    } else {
        sqlx::query(
            "UPDATE users SET 
            email=$1, name=$2, role_id=$3, boutique_id=$4, phone=$5
            WHERE id=$6",
        )
        .bind(&play.email)
        .bind(&play.name)
        .bind(role_id)
        .bind(&play.boutique_id)
        .bind(&play.phone)
        .bind(&play.id_edit)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        if !play.password.is_empty() {
            sqlx::query(
                "UPDATE users SET 
                password_hash=$1
                WHERE id=$2",
            )
            .bind(password_hash)
            .bind(&play.id_edit)
            .execute(&pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        }
    }

    Ok((StatusCode::CREATED, Json(json!({"statut":"success"}))))
}
