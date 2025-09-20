mod config;
mod db;
mod errors;
mod middleware;
mod auth;
mod models {
pub mod user;
}
mod handlers {
pub mod auth;
}
mod routes;


use crate::{config::Config, db::init_db, routes::create_router};
use tracing_subscriber;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
tracing_subscriber::fmt::init();


let config = Config::from_env();
let pool = init_db(&config.database_url).await;


let app = create_router(pool);


let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
println!("🚀 API disponible sur http://{}", addr);


axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
.await
.unwrap();
}