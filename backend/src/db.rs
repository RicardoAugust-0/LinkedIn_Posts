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
    
    use std::str::FromStr;
    let connection_options = sqlx::sqlite::SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
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
            linkedin_access_token_expires TEXT,
            linkedin_refresh_token TEXT,
            linkedin_refresh_token_expires TEXT,
            pexels_key TEXT,
            user_context TEXT,
            campaign_active BOOLEAN NOT NULL DEFAULT 0,
            campaign_topic TEXT,
            campaign_quantity INTEGER DEFAULT 10,
            campaign_cadence TEXT,
            campaign_windows TEXT,
            campaign_tone TEXT
        );"
    )
    .execute(&pool)
    .await?;

    // Adicionar a coluna pexels_key se ela não existir em bancos existentes
    sqlx::query("ALTER TABLE settings ADD COLUMN pexels_key TEXT;")
        .execute(&pool)
        .await
        .ok();

    // Adicionar colunas do refresh token do LinkedIn em bancos existentes
    sqlx::query("ALTER TABLE settings ADD COLUMN linkedin_refresh_token TEXT;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN linkedin_refresh_token_expires TEXT;")
        .execute(&pool)
        .await
        .ok();

    // Adicionar a coluna user_context se ela não existir em bancos existentes
    sqlx::query("ALTER TABLE settings ADD COLUMN user_context TEXT;")
        .execute(&pool)
        .await
        .ok();

    // Adicionar colunas da campanha em settings se não existirem
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_active BOOLEAN NOT NULL DEFAULT 0;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_topic TEXT;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_quantity INTEGER DEFAULT 10;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_cadence TEXT;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_windows TEXT;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE settings ADD COLUMN campaign_tone TEXT;")
        .execute(&pool)
        .await
        .ok();

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
            linkedin_post_id TEXT,
            is_automated BOOLEAN NOT NULL DEFAULT 0,
            retry_count INTEGER NOT NULL DEFAULT 0,
            error_message TEXT
        );"
    )
    .execute(&pool)
    .await?;

    // Adicionar a coluna is_automated em posts se não existir em bancos existentes
    sqlx::query("ALTER TABLE posts ADD COLUMN is_automated BOOLEAN NOT NULL DEFAULT 0;")
        .execute(&pool)
        .await
        .ok();

    // Colunas de robustez de publicação (retry automático + motivo da falha)
    sqlx::query("ALTER TABLE posts ADD COLUMN retry_count INTEGER NOT NULL DEFAULT 0;")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("ALTER TABLE posts ADD COLUMN error_message TEXT;")
        .execute(&pool)
        .await
        .ok();

    info!("Banco de dados SQLite inicializado com sucesso.");
    Ok(pool)
}
