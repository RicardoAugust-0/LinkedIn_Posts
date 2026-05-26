// backend/src/domain/errors.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro no banco de dados: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Erro de rede/HTTP: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Erro de serialização JSON: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Erro do LinkedIn API: {0}")]
    LinkedIn(String),

    #[error("Erro do Gemini API: {0}")]
    Gemini(String),

    #[error("Recurso não encontrado: {0}")]
    NotFound(String),

    #[error("Requisição inválida: {0}")]
    BadRequest(String),

    #[error("Erro interno do servidor: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Network(e) => (StatusCode::BAD_GATEWAY, e.to_string()),
            AppError::Serialization(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::LinkedIn(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Gemini(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}
