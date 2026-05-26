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
#[allow(dead_code)]
pub struct AuthCallbackParams {
    pub code: Option<String>,
    pub error: Option<String>,
    pub state: Option<String>,
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
) -> Result<Redirect, AppError> {
    match get_authorization_url(&pool, &config.public_url).await? {
        Some(url) => Ok(Redirect::to(&url)),
        None => {
            tracing::info!("LinkedIn Client ID não configurado. Redirecionando para login simulado.");
            Ok(Redirect::to("/api/auth/linkedin/callback?code=mock_code"))
        }
    }
}

pub async fn linkedin_callback(
    State(pool): State<SqlitePool>,
    State(config): State<AppConfig>,
    Query(params): Query<AuthCallbackParams>,
) -> Result<Redirect, AppError> {
    if let Some(err) = params.error {
        tracing::error!("Erro de autenticação no LinkedIn: {}", err);
        return Ok(Redirect::to("/settings?auth=error"));
    }

    let code = params.code.ok_or_else(|| {
        AppError::BadRequest("Código de autorização ausente".to_string())
    })?;

    let frontend_redirect_url = "/settings?auth=success";

    if let Err(e) = handle_callback(&pool, &code, &config.public_url).await {
        tracing::error!("Erro ao processar callback do LinkedIn: {}", e);
        return Ok(Redirect::to("/settings?auth=error"));
    }

    Ok(Redirect::to(frontend_redirect_url))
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
