
use sqlx::{SqlitePool, migrate::Migrator};



static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db(database_url: &str) -> SqlitePool {
let pool = SqlitePool::connect(database_url).await.expect("Connexion SQLite échouée");


    // Activer WAL pour de meilleures performances en concurrence
    sqlx::query("PRAGMA journal_mode=WAL;")
        .execute(&pool)
        .await
        .expect("Impossible d'activer WAL");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("Impossible d'activer WAL");

    // Ajouter un délai d'attente en cas de verrouillage (ici 5 secondes)
    sqlx::query("PRAGMA busy_timeout = 15000;")
        .execute(&pool)
        .await
        .expect("Impossible de définir busy_timeout");

    // (Optionnel) Activer la synchronisation plus rapide (fsync moins agressif)
    // Peut légèrement augmenter les performances en sacrifiant un peu de sécurité
    sqlx::query("PRAGMA synchronous=NORMAL;")
        .execute(&pool)
        .await
        .expect("Impossible de définir synchronous");
MIGRATOR.run(&pool).await.expect("Migrations échouées");
pool
}