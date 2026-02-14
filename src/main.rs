mod config;
mod db;
mod errors;
mod middleware;
mod auth;
mod handlers;
mod models;
mod routes;
mod service;

use crate::{config::Config, db::init_db, routes::create_router};
use tracing_subscriber;
use std::net::SocketAddr;
use std::env;
use windows_service::service::ServiceMainFunction;

#[allow(warnings)]
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for service control arguments
    if args.len() > 1 {
        match args[1].as_str() {
            "install" => {
                match service::install_service() {
                    Ok(_) => std::process::exit(0),
                    Err(e) => {
                        eprintln!("Failed to install service: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "uninstall" => {
                match service::uninstall_service() {
                    Ok(_) => std::process::exit(0),
                    Err(e) => {
                        eprintln!("Failed to uninstall service: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "--service" => {
                // Run as Windows service
                let dispatch_table = [
                    windows_service::service::ServiceTableEntry {
                        name: std::ffi::OsStr::new("MyRustApp"),
                        service_main: Some(service::ffi_service_main),
                    },
                    windows_service::service::ServiceTableEntry {
                        name: std::ffi::OsStr::new(""),
                        service_main: None,
                    },
                ];

                if let Err(e) = windows_service::service::start_service_control_dispatcher(&dispatch_table) {
                    eprintln!("Failed to start service: {}", e);
                }
                std::process::exit(0);
            }
            _ => {}
        }
    }

    // Run normally (not as service)
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = init_db(&config.database_url).await;

    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 API disponible sur http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}