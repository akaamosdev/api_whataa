use std::vec;

use argon2::password_hash::{PasswordHash, SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use axum::{Json, extract::State};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{auth::generate_token, errors::AppError, models::user::User};

#[derive(Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role_id: i64,
    pub boutique_id: String,
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
        role_id: payload.role_id,
        boutique_id: payload.boutique_id,
        created_at: Utc::now().to_rfc3339(),
    };

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, created_at,name,role_id,boutique_id) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.created_at)
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

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub privileges: Vec<String>,
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<LoginResponse>, AppError> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::Unauthorized)?;

    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        let token = generate_token(&user.id, "supersecretkeychangeit");
        let privileges: Vec<String> = sqlx::query_scalar(
            "SELECT p.name FROM permissions p
             JOIN permission_role pr ON p.id = pr.permission_id
             JOIN role_user ur ON pr.role_id = ur.role_id
             WHERE ur.user_id = ?",
        )
        .bind(&user.id)
        .fetch_all(&pool)
        .await
        .map_err(|e| AppError::SqlxError(e))?;
        Ok(Json(LoginResponse {
            token,
            user,
            privileges,
        }))
    } else {
        Err(AppError::Unauthorized)
    }
}

pub async fn get_all_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, role_id, boutique_id, email, password_hash, created_at FROM users",
    )
    .fetch_all(&pool)
    .await
    .map_err(AppError::from)?;

    Ok(Json(users))
}
//create
pub struct UserCreate {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct DefaultData {
    unite_id: String,
    depot_id: String,
    marque_id: String,
    sous_famille_id: String,
    caisse_id: String,
    mode_paiment_id: String,
    compagnies: CompagnieInfos
}
#[derive(FromRow,Debug,Serialize)]
pub struct CompagnieInfos{
    id: String,
    taux_tva: i8,
    taux_airsi: i8
}

pub async fn get_data_default(
    State(pool): State<PgPool>,
) -> Result<Json<DefaultData>, AppError> {
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
    let compagnies: CompagnieInfos =
        sqlx::query_as::<_,CompagnieInfos>("SELECT id, taux_tva, taux_airsi FROM compagnies LIMIT 1")
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
