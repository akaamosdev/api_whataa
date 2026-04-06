#![windows_subsystem = "windows"]

mod config;
mod db;
mod errors;
mod middleware;
mod auth;
mod handlers;
mod models;
mod routes;

use crate::{config::Config, db::init_db, routes::create_router};
use std::net::SocketAddr;
use tracing::{info, error};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {

    // 📁 Création du dossier logs
    let log_dir = "C:\\Whataa\\logs";
    std::fs::create_dir_all(log_dir).ok();

    // 📝 Logger fichier
    let file_appender = rolling::daily(log_dir, "whataa.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(fmt::Layer::default().with_writer(non_blocking))
        .init();

    info!("🚀 Démarrage API Whataa...");

    let config = Config::from_env();

    let pool = init_db(&config.database_url).await;
    info!("✅ Connexion base de données OK");


    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("🌍 API disponible sur http://{}", addr);

    if let Err(e) = axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    ).await {
        error!("❌ Erreur serveur: {:?}", e);
    }
}
