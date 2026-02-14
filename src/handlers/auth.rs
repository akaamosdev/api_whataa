use std::collections::HashMap;

use argon2::password_hash::rand_core::le;
use argon2::password_hash::{PasswordHash, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, extract::State};
use calamine::Table;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use sqlx::{PgPool, pool};
use uuid::Uuid;

use crate::models::user::UserLogin;
use crate::{auth::generate_token, errors::AppError, models::user::User};

#[derive(Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role_id: i32,
    pub boutique_id: String,
    pub phone: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterInput>,
) -> Result<Json<User>, AppError> {
    // Générer un sel aléatoire
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hasher le mot de passe
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user = User {
        id: Uuid::new_v4().to_string(),
        email: payload.email.clone(),
        password_hash,
        name: payload.name,
        phone: payload.phone,
        role_id: payload.role_id,
        boutique_id: payload.boutique_id,
    };

    sqlx::query(
        "INSERT INTO users (
        id, email, password_hash, created_at,name,role_id,boutique_id) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.name)
    .bind(&user.role_id)
    .bind(&user.boutique_id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let _ = sqlx::query("INSERT INTO role_user (user_id, role_id) VALUES (?, ?)")
        .bind(&user.id)
        .bind(1)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()));

    Ok(Json(user))
}

#[derive(Deserialize, Debug)]
pub struct LoginInput {
    pub name: String,
    pub password: String,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginInput>,
) -> Result<impl IntoResponse, AppError> {
    let user: UserLogin = sqlx::query_as("SELECT * FROM users WHERE LOWER(name) = LOWER($1)")
        .bind(&payload.name)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::Unauthorized)?;

    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        let token = generate_token(&user.id, "supersecretkeychangeit");
        Ok((
            StatusCode::OK,
            Json(json!({
                "token": token,
                "user": user,
                "privileges": get_privileges(&pool, user.role_id).await?,
                "default_ids": get_default_datas(&pool).await?,
            })),
        ))
    } else {
        Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ))
    }
}

pub async fn get_privileges(pool: &PgPool, role_id: i32) -> Result<Vec<i32>, AppError> {
    let privileges: Vec<i32> = sqlx::query_scalar(
        "SELECT permission_id FROM permission_role
         WHERE role_id = $1",
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(privileges)
}
pub async fn get_default_datas(pool: &PgPool) -> Result<HashMap<String, String>, AppError> {
    let tables_defauts = [
        "unites",
        "depots",
        "marques",
        "sous_familles",
        "caisses",
        "mode_paiements",
        "compagnies",
        "boutiques",
    ];
    let mut default_ids: HashMap<String, String> = HashMap::new();
    for tab in tables_defauts {
        let ids: String = sqlx::query_scalar(format!("SELECT id FROM {} LIMIT 1", tab).as_str())
            .fetch_one(pool)
            .await
            .map_err(AppError::SqlxError)?;
        default_ids.insert(tab.to_string(), ids);
    }
    Ok(default_ids)
}

#[derive(Serialize)]
pub struct DefaultData {
    unite_id: String,
    depot_id: String,
    marque_id: String,
    sous_famille_id: String,
    caisse_id: String,
    mode_paiment_id: String,
    compagnies: CompagnieInfos,
}
#[derive(FromRow, Debug, Serialize)]
pub struct CompagnieInfos {
    id: String,
    taux_tva: i8,
    taux_airsi: i8,
}

pub async fn get_data_default(State(pool): State<PgPool>) -> Result<Json<DefaultData>, AppError> {
    // fetch one default value for each table
    let unite_id: String = sqlx::query_scalar("SELECT id FROM unites LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let depot_id: String = sqlx::query_scalar("SELECT id FROM depots LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let marque_id: String = sqlx::query_scalar("SELECT id FROM marques LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let sous_famille_id: String = sqlx::query_scalar("SELECT id FROM sous_familles LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let caisse_id: String = sqlx::query_scalar("SELECT id FROM caisses LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;

    let mode_paiment_id: String = sqlx::query_scalar("SELECT id FROM mode_paiements LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(AppError::SqlxError)?;
    let compagnies: CompagnieInfos = sqlx::query_as::<_, CompagnieInfos>(
        "SELECT id, taux_tva, taux_airsi FROM compagnies LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .map_err(AppError::SqlxError)?;

    let data = DefaultData {
        unite_id,
        depot_id,
        marque_id,
        sous_famille_id,
        caisse_id,
        mode_paiment_id,
        compagnies,
    };

    Ok(Json(data))
}
