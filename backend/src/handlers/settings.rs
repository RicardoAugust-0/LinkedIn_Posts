// backend/src/handlers/settings.rs
use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::domain::models::{Settings, SaveSettingsRequest};
use crate::domain::errors::AppError;

pub async fn get_settings(
    State(pool): State<SqlitePool>,
) -> Result<Json<Settings>, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(settings))
}

pub async fn save_settings(
    State(pool): State<SqlitePool>,
    Json(req): Json<SaveSettingsRequest>,
) -> Result<Json<Settings>, AppError> {
    sqlx::query(
        "UPDATE settings SET \
            gemini_key = ?, \
            google_search_key = ?, \
            google_search_cx = ?, \
            linkedin_client_id = ?, \
            linkedin_client_secret = ? \
         WHERE id = 1"
    )
    .bind(req.gemini_key)
    .bind(req.google_search_key)
    .bind(req.google_search_cx)
    .bind(req.linkedin_client_id)
    .bind(req.linkedin_client_secret)
    .execute(&pool)
    .await?;

    // Retorna as configurações atualizadas
    let updated = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}
