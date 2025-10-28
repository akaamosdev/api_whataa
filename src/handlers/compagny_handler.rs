use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, SqlitePool};
use tracing_subscriber::fmt::format::Format;
use uuid::Uuid;

use crate::auth::generate_token;
use crate::{errors::AppError, models::user::User};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CompagnyCreate {
    pub name: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}

pub async fn create_compagny(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CompagnyCreate>,
) -> Result<impl IntoResponse, AppError> {
    let compagny_id = Uuid::new_v4().to_string();
    let date_now = Local::now().date_naive().to_string();

    let mut tx = pool.begin().await.map_err(AppError::SqlxError)?;
    let query_comp: String = String::from(
        "
        INSERT INTO compagnies (
        id, denomination,cigle,date_created,capital_so,statut_juridique_id,
        nb_commerce,nb_contribuable,secteur_act,responsable,
        address_phy, phone_fix,phone_mobil,taux_tva,taux_airsi,address_mail,
        logo,sale_negative,synchronise
    )
    VALUES (
        ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?, ?, ?,
        ?, ?, ?
    )",
    );

    sqlx::query(&query_comp)
        .bind(&compagny_id)
        .bind(&payload.name)
        .bind(&payload.name[1..3])
        .bind(&date_now)
        .bind("FCFA")
        .bind(1)
        .bind("")
        .bind("")
        .bind("")
        .bind(&payload.username)
        .bind("")
        .bind("")
        .bind(&payload.phone)
        .bind(0)
        .bind(0)
        .bind("")
        .bind(0)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //create Admin
    let admin_query = r#"
    INSERT INTO admins (
        id, name, email, password, phone, compagny_id, synchronise
    ) VALUES (?, ?, ?, ?, ?, ?, ?)
    "#;
    let admin_id = Uuid::new_v4().to_string();
    sqlx::query(admin_query)
        .bind(admin_id)
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.password)
        .bind(&payload.phone)
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //boutique
    let boutiq_query = r#"
        INSERT INTO boutiques (
            id, compagny_id, code, name, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let boutiq_id = Uuid::new_v4().to_string();
    sqlx::query(boutiq_query)
        .bind(&boutiq_id)
        .bind(&compagny_id)
        .bind("B001")
        .bind(&payload.name)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //user gerant
    // Hasher le mot de passe
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user = User {
        id: Uuid::new_v4().to_string(),
        email: payload.email.clone(),
        password_hash,
        name: payload.username,
        role_id: 1,
        boutique_id: String::new(),
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
    .bind(&boutiq_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let _ = sqlx::query("INSERT INTO role_user (user_id, role_id) VALUES (?, ?)")
        .bind(&user.id)
        .bind(1)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()));

    //depot
    let depot_query = r#"
        INSERT INTO depots (
            id, code, name, boutique_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let depot_id = Uuid::new_v4().to_string();
    sqlx::query(depot_query)
        .bind(&depot_id)
        .bind("D0001")
        .bind("DEPÔT DE PRINCIPAL")
        .bind(&boutiq_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //client default
    let client_query = r#"
        INSERT INTO clients (
            id, code, denomination,  defaut,  boutique_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?, ?)
    "#;
    let client_id = Uuid::new_v4().to_string();
    sqlx::query(client_query)
        .bind(&client_id)
        .bind("C0001")
        .bind("CLIENT COMPTOIR")
        .bind(0)
        .bind(&boutiq_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //famille
    let famille_query = r#"
        INSERT INTO familles (
            id, code, name, compagny_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let fami_id = Uuid::new_v4().to_string();
    sqlx::query(famille_query)
        .bind(&fami_id)
        .bind("F0001")
        .bind("Defaut")
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //sous_famille
    let sous_fami_query = r#"
        INSERT INTO sous_familles (
            id, code, name, famille_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let sous_fami_id = Uuid::new_v4().to_string();
    sqlx::query(sous_fami_query)
        .bind(&sous_fami_id)
        .bind("SF0001")
        .bind("Defaut")
        .bind(&fami_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //marque
    let marque_query = r#"
        INSERT INTO marques (
            id, code, name, compagny_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let marque_id = Uuid::new_v4().to_string();
    sqlx::query(marque_query)
        .bind(&marque_id)
        .bind("SF0001")
        .bind("Defaut")
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //unité
    let unite_query = r#"
        INSERT INTO unites (
            id, code, name, compagny_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let unite_id = Uuid::new_v4().to_string();
    sqlx::query(unite_query)
        .bind(&unite_id)
        .bind("U0001")
        .bind("Defaut")
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //Caisse
    let caisse_query = r#"
        INSERT INTO caisses (
            id, code, name, boutique_id, statut, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?, ?)
    "#;
    let caisse_id = Uuid::new_v4().to_string();
    sqlx::query(caisse_query)
        .bind(&caisse_id)
        .bind("C0001")
        .bind("CAISSE BOUTIQUE")
        .bind(&boutiq_id)
        .bind(1)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //mode_paiements
    let mode_query = r#"
        INSERT INTO mode_paiements (
            id, name, compagny_id, synchronise
        ) 
        VALUES (?, ?, ?, ?)
    "#;
    let mode_paiement_id = Uuid::new_v4().to_string();
    sqlx::query(mode_query)
        .bind(&mode_paiement_id)
        .bind("ESPECE")
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //type_depenses
    let depense_type_query = r#"
        INSERT INTO type_depenses (
            id, code, name, boutique_id, synchronise
        ) 
        VALUES (?, ?, ?, ?, ?)
    "#;
    let depense_type_id = Uuid::new_v4().to_string();
    sqlx::query(depense_type_query)
        .bind(&depense_type_id)
        .bind("TD001")
        .bind("Charge Fixe")
        .bind(&compagny_id)
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    // 7. Valider la transaction
    tx.commit().await.map_err(AppError::SqlxError)?;
    let token = generate_token(&user.id, "supersecretkeychangeit");

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "statut": true,
            "token":token,
            "message": "Compte enregistré avec succès",
            "depot_id":&depot_id,
            "compagny_id":&compagny_id,
            "uniteId":&unite_id,
            "marqueId":&marque_id,
            "sousFamilleId":&sous_fami_id,
            "boutiqueId":&boutiq_id,
            "userId":&user.id,
            "caisse_id":&caisse_id,
            "modePaimentID":&mode_paiement_id,
            "userName":&user.name,
            "userRoleId":&user.role_id,
            "email":&payload.email,
        })),
    ))
}
