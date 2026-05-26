// backend/src/handlers/search.rs
use axum::{extract::{Query, State}, Json};
use sqlx::SqlitePool;
use crate::domain::models::SearchImagesRequest;
use crate::domain::errors::AppError;
use crate::services::search::{search_images as service_search_images, ImageSearchResult};

pub async fn search_images(
    State(pool): State<SqlitePool>,
    Query(req): Query<SearchImagesRequest>,
) -> Result<Json<Vec<ImageSearchResult>>, AppError> {
    let results = service_search_images(&pool, &req.query).await?;
    Ok(Json(results))
}
