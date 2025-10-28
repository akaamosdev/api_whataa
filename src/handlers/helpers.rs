use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use calamine::{Xlsx, XlsxError, open_workbook, open_workbook_from_rs};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

use axum_extra::extract::Multipart;
use calamine::{DataType, RangeDeserializerBuilder, Reader, open_workbook_auto};
use std::{fs::File, io::Cursor, vec};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{articles::Article, ligne_document::LigneDocumentDto},
};

#[derive(Deserialize, Serialize)]
pub struct TableGetData {
    pub table: String,
    pub type_doc: String,
}
pub async fn get_last_counts(
    State(pool): State<SqlitePool>,
    Json(tables): Json<TableGetData>,
) -> Result<impl IntoResponse, AppError> {
    let query_s: String = if tables.table == "documents" {
        format!(
            "SELECT COUNT(*) AS count FROM documents WHERE type_doc='{}'",
            tables.type_doc
        )
    } else {
        format!("SELECT COUNT(*) AS counts FROM {}", tables.table)
    };
    let count: i64 = sqlx::query_scalar(&query_s)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "table": tables.table,
            "count": count
        })),
    ))
}

//store image
pub async fn upload_file(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut id = String::new();
    let mut table = String::new();
    let mut url = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("id") => {
                id = field.text().await.unwrap();
            }
            Some("table") => {
                table = field.text().await.unwrap();
            }
            Some("upload_image") => {
                // let file_name = field.file_name().unwrap_or("upload.png").to_string();
                let data = field.bytes().await.unwrap();

                tokio::fs::write(format!("./uploads/{}image.png", id), &data)
                    .await
                    .unwrap();

                url = format!("/uploads/{}image.png", id);
            }
            _ => {}
        }
    }
    let column = if table == "articles" { "image" } else { "logo" };
    let query = format!("UPDATE {} SET {} = ? WHERE id = ?", table, column);

    let rows_affected = sqlx::query(&query)
        .bind(&url)
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .rows_affected();

    Ok((
        StatusCode::OK,
        Json(json!({
            "statut": true,
            "message": "image enregistrée jour avec succès"
        })),
    ))
}
//import article
#[derive(Serialize)]
struct ImportResponse {
    success: usize,
    failed: usize,
}
pub async fn import_articles(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut file_data = Vec::new();
    let mut boutique_id = String::new();
    let mut depot_id = String::new();
    let mut doc_id = String::new();

    // 1. Extraire les données du formulaire multipart
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                file_data = field.bytes().await.unwrap().to_vec();
            }
            "boutique_id" => {
                boutique_id = field.text().await.unwrap_or_default();
            }
            "depot_id" => {
                depot_id = field.text().await.unwrap_or_default();
            }
            _ => (),
        }
    }

    if file_data.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": false, "message": "Aucun fichier n'a été téléversé."})),
        ));
    }
    if boutique_id.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({"status": false, "message": "L'ID de la boutique est manquant."})),
        ));
    }

    // 2. Ouvrir le classeur Excel à partir des données en mémoire
    let cursor = Cursor::new(file_data);
    let mut workbook: calamine::Xlsx<std::io::Cursor<Vec<u8>>> =
        open_workbook_from_rs(cursor).map_err(|e: XlsxError| AppError::Internal(e.to_string()))?;

    let worksheet = workbook.worksheet_range("Sheet1").map_err(|e| {
        AppError::BadRequest(format!(
            "Impossible de trouver ou de lire la feuille 'Sheet1': {}",
            e
        ))
    })?;

    let unite_id: String =
        sqlx::query_scalar("SELECT id FROM unites ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| AppError::SqlxError(e))?;

    let marque_id: String =
        sqlx::query_scalar("SELECT id FROM marques ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| AppError::SqlxError(e))?;

    let mut success_count = 0;
    // 3. Démarrer une transaction de base de données
    let mut tx = pool.begin().await.map_err(AppError::SqlxError)?;

    // 4. Itérer sur les lignes (en sautant l'en-tête)
    for row in worksheet.rows().skip(1) {
        let new_id = uuid::Uuid::new_v4().to_string();
        let sous_famille = row.get(3).and_then(|c| c.as_string()).unwrap_or_default();
        let mut sous_famille_id: Option<String> = sqlx::query_scalar(
            "
                SELECT id FROM sous_familles 
                WHERE name=?",
        )
        .bind(&sous_famille)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::SqlxError(e))?;

        if sous_famille_id.is_none() {
            let famid: String = uuid::Uuid::new_v4().to_string();
            let code: String = format!("{}{}", &sous_famille[0..3], "01");
            sqlx::query("INSERT INTO familles (id, code, name ) VALUES (?, ?, ?)")
                .bind(&famid)
                .bind(code)
                .bind(&sous_famille)
                .execute(&mut *tx)
                .await
                .map_err(AppError::SqlxError)?;
            let new_sou_fami_id = uuid::Uuid::new_v4().to_string();
            let s_code: String = format!("S{}{}", &sous_famille[0..3], "01");
            sqlx::query(
                "INSERT INTO sous_familles (id, code, name, famille_id ) VALUES (?, ?, ?, ?)",
            )
            .bind(&new_sou_fami_id)
            .bind(s_code)
            .bind(&sous_famille)
            .bind(&famid)
            .execute(&mut *tx)
            .await
            .map_err(AppError::SqlxError)?;

            sous_famille_id = Some(new_sou_fami_id);
        }

        // 5. Mapper les cellules de la ligne à la structure Article
        // Assurez-vous que l'ordre des colonnes dans votre Excel correspond
        let article = Article {
            id: new_id,
            code: row.get(0).and_then(|c| c.as_string()).unwrap_or_default(),
            code_bar: row.get(1).and_then(|c| c.as_string()).unwrap_or_default(),
            name: row.get(2).and_then(|c| c.as_string()).unwrap_or_default(),
            sous_famille_id: sous_famille_id.unwrap(), // row.get(3).and_then(|c| c.as_string()).unwrap_or_default(),
            marque_id: marque_id.clone(), // row.get(4).and_then(|c| c.as_string()).unwrap_or_default(),
            unite_id: unite_id.clone(), // row.get(5).and_then(|c| c.as_string()).unwrap_or_default(),
            alert_stock: 5.0,           // row.get(6).and_then(|c| c.as_f64()).unwrap_or(0.0),
            is_stock: 1,                //row.get(7).and_then(|c| c.as_i64()).unwrap_or(1) as i8,
            boutique_id: boutique_id.clone(), // Utiliser l'ID de la boutique du formulaire
            price_buy: row.get(4).and_then(|c| c.as_f64()).unwrap_or(0.0),
            price_seller: row.get(5).and_then(|c| c.as_f64()).unwrap_or(0.0),
            stock: row.get(6).and_then(|c| c.as_f64()).unwrap_or(0.0),
            synchronise: 0,
        };

        //println!("Article {:?} ",article);
        // Ignorer les lignes où le nom de l'article est vide
        if article.name.is_empty() || article.code == "Ref" {
            continue;
        }
        // 6. Insérer l'article dans la base de données
        sqlx::query!(
            r#"
            INSERT INTO articles (
            id, code, code_bar, name, sous_famille_id, 
            marque_id, unite_id, 
            alert_stock, is_stock, boutique_id, price_buy, 
            price_seller, stock, synchronise
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            article.id,
            article.code,
            article.code_bar,
            article.name,
            article.sous_famille_id,
            article.marque_id,
            article.unite_id,
            article.alert_stock,
            article.is_stock,
            article.boutique_id,
            article.price_buy,
            article.price_seller,
            article.stock,
            article.synchronise
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SqlxError)?;

        if article.stock != 0.0 {
            if doc_id.is_empty() {
                doc_id = Uuid::new_v4().to_string();
                let doc_date = Local::now().date_naive().to_string();
                let query_doc = r#"
            INSERT INTO documents (
            id, document_num, document_date, depot_id, 
            commentaire, type_doc, boutique_id
            ) 
            VALUES(
            ?, ?, ?, ?,
            ?, ?, ?
            )
            "#;
                sqlx::query(query_doc)
                    .bind(&doc_id)
                    .bind("MVTIS0000")
                    .bind(&doc_date)
                    .bind(&depot_id)
                    .bind("stock initial")
                    .bind(0)
                    .bind(article.boutique_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| AppError::SqlxError(e))?;
            }
            let lig: LigneDocumentDto = LigneDocumentDto {
                id: Uuid::new_v4().to_string(),
                document_id: doc_id.clone(),
                article_id: article.id,
                prix_achat_ttc: article.price_buy * article.stock,
                prix_vente_ttc: article.price_seller * article.stock,
                qte: article.stock,
                qte_mvt_stock: article.stock,
                taux_remise: 0.0,
                montant_ttc: article.price_buy * article.stock,
                montant_net: article.price_buy * article.stock,
                montant_remise: 0.0,
                achever: 1,
                synchronise: 0,
            };
            sqlx::query!(
                r#"
            INSERT INTO ligne_documents (
                id, document_id, article_id, prix_achat_ttc, prix_vente_ttc, 
                qte, qte_mvt_stock, taux_remise, montant_ttc, montant_net, 
                montant_remise, achever, synchronise
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
                lig.id,
                lig.document_id,
                lig.article_id,
                lig.prix_achat_ttc,
                lig.prix_vente_ttc,
                lig.qte,
                lig.qte_mvt_stock,
                lig.taux_remise,
                lig.montant_ttc,
                lig.montant_net,
                lig.montant_remise,
                lig.achever,
                lig.synchronise,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::SqlxError(e))?;
        }

        success_count += 1;
    }

    // 7. Valider la transaction
    tx.commit().await.map_err(AppError::SqlxError)?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": true,
            "message": format!("{} articles importés avec succès.", success_count)
        })),
    ))
}
