// backend/src/main.rs
mod config;
mod db;
mod domain;
mod services;
mod scheduler;
mod handlers {
    pub mod ai;
    pub mod auth;
    pub mod posts;
    pub mod search;
    pub mod settings;
}

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use std::fs;
use sqlx::SqlitePool;

use crate::config::AppConfig;
use crate::db::init_db;
use crate::scheduler::start_scheduler;

// Estado compartilhado do Axum contendo banco de dados e configurações do app
#[derive(Clone, Debug)]
struct AppState {
    pool: SqlitePool,
    config: AppConfig,
}

// Implementar FromRef para permitir extrair sub-estados nos handlers
impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logs
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Carregar configurações
    let config = AppConfig::load();
    info!("Configurações carregadas: Host={}, Port={}", config.host, config.port);

    // Inicializar diretório de uploads
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "uploads".to_string());
    fs::create_dir_all(&uploads_dir)?;

    // Inicializar banco de dados
    let pool = init_db(&config.database_url).await?;

    // Iniciar agendador em background
    let scheduler_pool = pool.clone();
    tokio::spawn(async move {
        start_scheduler(scheduler_pool).await;
    });

    // Definir estado compartilhado unificado
    let state = AppState {
        pool,
        config: config.clone(),
    };

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // Para portfólio local, permitir qualquer origem facilita
        .allow_methods(Any)
        .allow_headers(Any);

    // Definir as rotas
    let app = Router::new()
        // API Posts
        .route("/api/posts", get(handlers::posts::list_posts).post(handlers::posts::create_post).delete(handlers::posts::clear_all_posts))
        .route("/api/posts/stats", get(handlers::posts::get_dashboard_stats))
        .route("/api/posts/:id", get(handlers::posts::get_post).put(handlers::posts::update_post).delete(handlers::posts::delete_post))
        .route("/api/posts/:id/publish", post(handlers::auth::publish_post_now))
        
        // API AI e Search
        .route("/api/generate/text", post(handlers::ai::generate_text))
        .route("/api/generate/improve", post(handlers::ai::improve_post))
        .route("/api/generate/image", post(handlers::ai::generate_image))
        .route("/api/generate/topics", get(handlers::ai::suggest_topics))
        .route("/api/search/images", get(handlers::search::search_images))
        
        // API Configurações e LinkedIn Auth
        .route("/api/settings", get(handlers::settings::get_settings).put(handlers::settings::save_settings))
        .route("/api/settings/test-all", post(handlers::settings::test_all_credentials))
        .route("/api/auth/linkedin", get(handlers::auth::linkedin_login))
        .route("/api/auth/linkedin/callback", get(handlers::auth::linkedin_callback))
        .route("/api/auth/linkedin/status", get(handlers::auth::get_auth_status))
        .route("/api/auth/linkedin/disconnect", post(handlers::auth::linkedin_disconnect))

        
        // Servir imagens geradas localmente
        .nest_service("/uploads", ServeDir::new(&uploads_dir))
        
        // Middlewares
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // Estado compartilhado unificado
        .with_state(state);

    // Bind e Start do servidor HTTP
    let addr_str = format!("{}:{}", config.host, config.port);
    let addr: SocketAddr = addr_str.parse()?;
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Servidor rodando em http://{}", addr_str);
    
    axum::serve(listener, app).await?;

    Ok(())
}
