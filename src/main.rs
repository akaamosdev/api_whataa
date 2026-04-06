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
use tracing::{error, info};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // logs -> fichier
    let log_dir = "./logs";
    std::fs::create_dir_all(log_dir).ok();

    let file_appender = rolling::daily(log_dir, "whataa.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // IMPORTANT: désactiver ANSI (évite les [0m dans les fichiers)
    tracing_subscriber::registry()
        .with(fmt::Layer::default().with_ansi(false).with_writer(non_blocking))
        .init();

    if let Err(e) = run().await {
        error!("❌ Erreur fatale: {:#}", e);

        // utile même sans console
        std::fs::write("./logs/fatal.txt", format!("{:#}\n", e)).ok();
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Démarrage API Whataa...");

    // si Config::from_env() peut échouer, faites-la retourner Result.
    // Sinon, au minimum logguez ce qu'elle lit.
    let config = Config::from_env();

    let pool = init_db(&config.database_url).await;
    info!("✅ Connexion base de données OK");

    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("🌍 API disponible sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}