// backend/src/handlers/auth.rs
use axum::{
    extract::{Query, State, Path},
    response::Redirect,
    Json,
};
use sqlx::SqlitePool;
use serde::Deserialize;
use crate::config::AppConfig;
use crate::domain::errors::AppError;
use crate::services::linkedin::{
    get_auth_status as service_get_auth_status,
    get_authorization_url,
    handle_callback,
    disconnect,
    publish_post,
    AuthStatusResponse,
};

#[derive(Debug, Deserialize)]
pub struct AuthCallbackParams {
    pub code: Option<String>,
    pub error: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub redirect_url: Option<String>,
}

pub async fn get_auth_status(
    State(pool): State<SqlitePool>,
) -> Result<Json<AuthStatusResponse>, AppError> {
    let status = service_get_auth_status(&pool).await?;
    Ok(Json(status))
}

pub async fn linkedin_login(
    State(pool): State<SqlitePool>,
    State(config): State<AppConfig>,
    Query(params): Query<LoginParams>,
) -> Result<Redirect, AppError> {
    let state = params.redirect_url.unwrap_or_else(|| "linkedin_auth_state".to_string());
    
    match get_authorization_url(&pool, &config.public_url, &state).await? {
        Some(url) => Ok(Redirect::to(&url)),
        None => {
            tracing::info!("LinkedIn Client ID não configurado. Redirecionando para login simulado.");
            let mock_redirect = format!(
                "/api/auth/linkedin/callback?code=mock_code&state={}",
                urlencoding::encode(&state)
            );
            Ok(Redirect::to(&mock_redirect))
        }
    }
}

pub async fn linkedin_callback(
    State(pool): State<SqlitePool>,
    State(config): State<AppConfig>,
    Query(params): Query<AuthCallbackParams>,
) -> Result<Redirect, AppError> {
    let mut redirect_host = config.frontend_url.clone();
    if let Some(ref state) = params.state {
        if state.starts_with("http://") || state.starts_with("https://") {
            let mut host = state.clone();
            if host.ends_with('/') {
                host.pop();
            }
            redirect_host = host;
        }
    }

    if let Some(err) = params.error {
        tracing::error!("Erro de autenticação no LinkedIn: {}", err);
        return Ok(Redirect::to(&format!("{}/settings?auth=error", redirect_host)));
    }

    let code = params.code.ok_or_else(|| {
        AppError::BadRequest("Código de autorização ausente".to_string())
    })?;

    let frontend_redirect_url = format!("{}/settings?auth=success", redirect_host);

    if let Err(e) = handle_callback(&pool, &code, &config.public_url).await {
        tracing::error!("Erro ao processar callback do LinkedIn: {}", e);
        return Ok(Redirect::to(&format!("{}/settings?auth=error", redirect_host)));
    }

    Ok(Redirect::to(&frontend_redirect_url))
}

pub async fn publish_post_now(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let li_id = publish_post(&pool, &id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "linkedin_post_id": li_id
    })))
}

pub async fn linkedin_disconnect(
    State(pool): State<SqlitePool>,
) -> Result<Json<serde_json::Value>, AppError> {
    disconnect(&pool).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}
