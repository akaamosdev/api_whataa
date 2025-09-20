
use sqlx::{SqlitePool, migrate::Migrator};



static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db(database_url: &str) -> SqlitePool {
/* if !Path::new("db").exists() {
std::fs::create_dir_all("db").unwrap();
} */
let pool = SqlitePool::connect(database_url).await.expect("Connexion SQLite échouée");
MIGRATOR.run(&pool).await.expect("Migrations échouées");
pool
}