// backend/src/handlers/ai.rs
use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::domain::models::{CreatePostRequest, GenerateImageRequest};
use crate::domain::errors::AppError;
use crate::services::ai::{
    generate_text as service_generate_text,
    generate_image as service_generate_image,
    TextGenerationResponse,
    ImageGenerationResponse,
};

pub async fn generate_text(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<TextGenerationResponse>, AppError> {
    let result = service_generate_text(&pool, &req.topic, &req.prompt_override).await?;
    Ok(Json(result))
}

pub async fn generate_image(
    State(pool): State<SqlitePool>,
    Json(req): Json<GenerateImageRequest>,
) -> Result<Json<ImageGenerationResponse>, AppError> {
    let result = service_generate_image(&pool, &req.prompt).await?;
    Ok(Json(result))
}
