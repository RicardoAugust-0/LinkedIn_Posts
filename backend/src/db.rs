use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::fs;
use tracing::info;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Certificar que o diretório do arquivo do banco existe
    if database_url.starts_with("sqlite:") {
        let path = database_url.trim_start_matches("sqlite:");
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).ok();
            }
        }
    }

    info!("Conectando ao banco de dados SQLite: {}", database_url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Criar as tabelas se não existirem
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            gemini_key TEXT,
            google_search_key TEXT,
            google_search_cx TEXT,
            linkedin_client_id TEXT,
            linkedin_client_secret TEXT,
            linkedin_access_token TEXT,
            linkedin_access_token_expires TEXT
        );"
    )
    .execute(&pool)
    .await?;

    // Inserir registro inicial de settings se não existir
    sqlx::query(
        "INSERT OR IGNORE INTO settings (id) VALUES (1);"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            topic TEXT NOT NULL,
            content TEXT NOT NULL,
            image_url TEXT,
            image_source TEXT NOT NULL,
            status TEXT NOT NULL,
            scheduled_at TEXT,
            published_at TEXT,
            created_at TEXT NOT NULL,
            linkedin_post_id TEXT
        );"
    )
    .execute(&pool)
    .await?;

    info!("Banco de dados SQLite inicializado com sucesso.");
    Ok(pool)
}
