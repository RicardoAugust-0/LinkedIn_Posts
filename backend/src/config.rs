use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub host: String,
    pub public_url: String,
    pub frontend_url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        // Carregar do arquivo .env se existir
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:posts.db".to_string());
        
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        
        let public_url = env::var("PUBLIC_URL").unwrap_or_else(|_| format!("http://localhost:{}", port));

        let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

        Self {
            database_url,
            port,
            host,
            public_url,
            frontend_url,
        }
    }
}
