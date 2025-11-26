use sqlx::{postgres::PgPoolOptions, PgPool, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db(database_url: &str) -> PgPool {
    // Connexion au pool Postgres
    let pool = PgPoolOptions::new()
        .max_connections(10) // tu peux augmenter selon ton besoin
        .connect(database_url)
        .await
        .expect("Connexion PostgreSQL échouée");

    // Exécuter les migrations SQL
    MIGRATOR
        .run(&pool)
        .await
        .expect("Échec des migrations PostgreSQL");

    pool
}
