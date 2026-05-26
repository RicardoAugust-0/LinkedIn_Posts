// backend/src/handlers/ai.rs
use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::domain::models::{CreatePostRequest, GenerateImageRequest, ImprovePostRequest, ImprovePostResponse};
use crate::domain::errors::AppError;
use crate::services::ai::{
    generate_text as service_generate_text,
    generate_image as service_generate_image,
    suggest_topics as service_suggest_topics,
    improve_post as service_improve_post,
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

pub async fn suggest_topics(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<String>>, AppError> {
    let result = service_suggest_topics(&pool).await?;
    Ok(Json(result))
}

pub async fn improve_post(
    State(pool): State<SqlitePool>,
    Json(req): Json<ImprovePostRequest>,
) -> Result<Json<ImprovePostResponse>, AppError> {
    let content = service_improve_post(&pool, &req.content, &req.mode).await?;
    Ok(Json(ImprovePostResponse { content }))
}
