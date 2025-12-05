use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Local};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::auth::generate_token;
use crate::models::compagnie::Compagny;
use crate::{errors::AppError, models::user::User};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CompagnyCreate {
    pub name: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub logo: Option<String>,
}

pub async fn create_compagny(
    State(pool): State<PgPool>,
    Json(payload): Json<CompagnyCreate>,
) -> Result<impl IntoResponse, AppError> {
    let compagny_id = Uuid::new_v4().to_string();
    let date_now = Local::now().date_naive().to_string();

    let mut tx = pool.begin().await.map_err(AppError::SqlxError)?;
    let query_comp: String = String::from(
        "
        INSERT INTO compagnies (
        id, denomination,cigle,date_created,statut_juridique_id,
        responsable, phone_fix,phone_mobil,taux_tva,taux_airsi,address_mail,
        logo,sale_negative
    )
    VALUES (
        $1, $2, $3, $4, $5, $6,
        $7, $8, $9, $10,
        $11, $12, $13
    )",
    );

    sqlx::query(&query_comp)
        .bind(&compagny_id)
        .bind(&payload.name)
        .bind(&payload.name[0..3])
        .bind(&date_now)
        .bind(1)
        .bind(&payload.username)
        .bind("")
        .bind(&payload.phone)
        .bind(0)
        .bind(0)
        .bind(&payload.email)
        .bind("")
        .bind(0)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //create Admin
    let admin_query = r#"
    INSERT INTO admins (
        id, name, email, password, phone, compagny_id
    ) VALUES ($1, $2, $3, $4, $5, $6)
    "#;
    let admin_id = Uuid::new_v4().to_string();
    sqlx::query(admin_query)
        .bind(admin_id)
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.password)
        .bind(&payload.phone)
        .bind(&compagny_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //boutique
    let boutiq_query = r#"
        INSERT INTO boutiques (
            id, compagny_id, code, name
        ) 
        VALUES ($1, $2, $3, $4)
    "#;
    let boutiq_id = Uuid::new_v4().to_string();
    sqlx::query(boutiq_query)
        .bind(&boutiq_id)
        .bind(&compagny_id)
        .bind("B001")
        .bind(&payload.name)
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
    };

    sqlx::query(
        "INSERT INTO users (id, email, 
        password_hash,name,role_id,boutique_id) 
        VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.name)
    .bind(&user.role_id)
    .bind(&boutiq_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let _ = sqlx::query("INSERT INTO role_user (user_id, role_id) VALUES ($1, $2)")
        .bind(&user.id)
        .bind(1)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()));

    //depot
    let depot_query = r#"
        INSERT INTO depots (
            id, code, name, boutique_id
        ) 
        VALUES ($1, $2, $3, $4)
    "#;
    let depot_id = Uuid::new_v4().to_string();
    sqlx::query(depot_query)
        .bind(&depot_id)
        .bind("D0001")
        .bind("DEPÔT DE PRINCIPAL")
        .bind(&boutiq_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //client default
    let client_query = r#"
        INSERT INTO tiers (
            id, code, denomination,  defaut,  boutique_id, type_tier
        ) 
        VALUES ($1, $2, $3, $4, $5, $6)
    "#;
    let client_id = Uuid::new_v4().to_string();
    sqlx::query(client_query)
        .bind(&client_id)
        .bind("C0001")
        .bind("CLIENT COMPTOIR")
        .bind(true)
        .bind(&boutiq_id)
        .bind("CLIENT")
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //famille
    let famille_query = r#"
        INSERT INTO familles (
            id, code, name, compagny_id
        ) 
        VALUES ($1, $2, $3, $4)
    "#;
    let fami_id = Uuid::new_v4().to_string();
    sqlx::query(famille_query)
        .bind(&fami_id)
        .bind("F0001")
        .bind("Famille Defaut")
        .bind(&compagny_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //sous_famille
    let sous_fami_query = r#"
        INSERT INTO sous_familles (
            id, code, name, famille_id
        ) 
        VALUES ($1, $2, $3, $4)
    "#;
    let sous_fami_id = Uuid::new_v4().to_string();
    sqlx::query(sous_fami_query)
        .bind(&sous_fami_id)
        .bind("SF0001")
        .bind("Sous Famille Defaut")
        .bind(&fami_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //marque
    let marque_query = r#"
        INSERT INTO marques (
            id, code, name, compagny_id
        ) 
        VALUES ($1,$2,$3,$4)
    "#;
    let marque_id = Uuid::new_v4().to_string();
    sqlx::query(marque_query)
        .bind(&marque_id)
        .bind("SF0001")
        .bind("Marque Defaut")
        .bind(&compagny_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //unité
    let unite_query = r#"
        INSERT INTO unites (
            id, code, name, compagny_id
        ) 
        VALUES ($1,$2,$3,$4)
    "#;
    let unite_id = Uuid::new_v4().to_string();
    sqlx::query(unite_query)
        .bind(&unite_id)
        .bind("U0001")
        .bind("Unite Defaut")
        .bind(&compagny_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //Caisse
    let caisse_query = r#"
        INSERT INTO caisses (
            id, code, name, boutique_id, statut
        ) 
        VALUES ($1,$2,$3,$4,$5)
    "#;
    let caisse_id = Uuid::new_v4().to_string();
    sqlx::query(caisse_query)
        .bind(&caisse_id)
        .bind("C0001")
        .bind("CAISSE PRINCIPALE")
        .bind(&boutiq_id)
        .bind(true)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //mode_paiements
    let mode_query = r#"
        INSERT INTO mode_paiements (
            id, name, compagny_id
        ) 
        VALUES ($1,$2,$3)
    "#;
    let mode_paiement_id = Uuid::new_v4().to_string();
    sqlx::query(mode_query)
        .bind(&mode_paiement_id)
        .bind("ESPECE")
        .bind(&compagny_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

    //type_depenses
    let depense_type_query = r#"
        INSERT INTO type_depenses (
            id, code, name, boutique_id
        ) 
        VALUES ($1,$2,$3,$4)
    "#;
    let depense_type_id = Uuid::new_v4().to_string();
    sqlx::query(depense_type_query)
        .bind(&depense_type_id)
        .bind("TD001")
        .bind("Charge Fixe")
        .bind(&compagny_id)
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
pub async fn get_compagny(
    State(pool): State<PgPool>,
    Path(compagny_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let compagnies = sqlx::query_as::<_, Compagny>("SELECT * FROM compagnies")
        .bind(&compagny_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok((StatusCode::OK, Json(compagnies)))
}
// 
pub async fn update_compagny(
    State(pool): State<PgPool>,
    Json(payload): Json<Compagny>,
) -> Result<impl IntoResponse, AppError> {

    let query_comp = "
        UPDATE compagnies SET 
            denomination = $2,
            cigle = $3,
            date_created = $4,
            statut_juridique_id = $5,
            responsable = $6,
            phone_fix = $7,
            phone_mobil = $8,
            taux_tva = $9,
            taux_airsi = $10,
            address_mail = $11,
            sale_negative = $12,
            nb_contribuable = $13,
            nb_commerce = $14,
            secteur_act = $15,
            address_phy = $16
            
        WHERE id = $1
    ";

    sqlx::query(query_comp)
        .bind(&payload.id)
        .bind(&payload.denomination)
        .bind(&payload.cigle)
        .bind(&payload.date_created)
        .bind(&payload.statut_juridique_id)
        .bind(&payload.responsable)
        .bind(&payload.phone_fix)
        .bind(&payload.phone_mobil)
        .bind(&payload.taux_tva)
        .bind(&payload.taux_airsi)
        .bind(&payload.address_mail)
        .bind(&payload.sale_negative)
        .bind(&payload.nb_contribuable)
        .bind(&payload.nb_commerce)
        .bind(&payload.secteur_act)
        .bind(&payload.address_phy)
        .execute(&pool)
        .await
        .map_err(AppError::SqlxError)?;   // OK

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": "Compagnie mise à jour avec succès",
        })),
    ))
}
