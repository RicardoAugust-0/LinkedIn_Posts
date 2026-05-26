// backend/src/handlers/posts.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::models::{Post, SavePostRequest, DashboardStats, PostStatus};
use crate::domain::errors::AppError;
use tracing::info;

pub async fn list_posts(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Post>>, AppError> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}

pub async fn get_post(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post não encontrado".to_string()))?;

    Ok(Json(post))
}

pub async fn create_post(
    State(pool): State<SqlitePool>,
    Json(req): Json<SavePostRequest>,
) -> Result<Json<Post>, AppError> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();

    sqlx::query(
        "INSERT INTO posts (id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, NULL, ?, NULL)"
    )
    .bind(&id)
    .bind(&req.title)
    .bind(&req.topic)
    .bind(&req.content)
    .bind(&req.image_url)
    .bind(req.image_source)
    .bind(req.status)
    .bind(req.scheduled_at)
    .bind(created_at)
    .execute(&pool)
    .await?;

    info!("Novo post criado no banco: {} (Status: {:?})", id, req.status);

    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

pub async fn update_post(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(req): Json<SavePostRequest>,
) -> Result<Json<Post>, AppError> {
    // Verificar se o post existe
    let existing = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post não encontrado".to_string()))?;

    // Se já estiver publicado, não permitir certas edições de agendamento
    let published_at = existing.published_at;
    let linkedin_post_id = existing.linkedin_post_id;

    sqlx::query(
        "UPDATE posts SET \
            title = ?, \
            topic = ?, \
            content = ?, \
            image_url = ?, \
            image_source = ?, \
            status = ?, \
            scheduled_at = ?, \
            published_at = ?, \
            linkedin_post_id = ? \
         WHERE id = ?"
    )
    .bind(req.title)
    .bind(req.topic)
    .bind(req.content)
    .bind(req.image_url)
    .bind(req.image_source)
    .bind(req.status)
    .bind(req.scheduled_at)
    .bind(published_at)
    .bind(linkedin_post_id)
    .bind(&id)
    .execute(&pool)
    .await?;

    info!("Post atualizado no banco: {} (Novo Status: {:?})", id, req.status);

    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

pub async fn delete_post(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_dashboard_stats(
    State(pool): State<SqlitePool>,
) -> Result<Json<DashboardStats>, AppError> {
    let total_posts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await?;

    let draft_posts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts WHERE status = ?")
        .bind(PostStatus::Draft)
        .fetch_one(&pool)
        .await?;

    let scheduled_posts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts WHERE status = ?")
        .bind(PostStatus::Scheduled)
        .fetch_one(&pool)
        .await?;

    let published_posts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM posts WHERE status = ?")
        .bind(PostStatus::Published)
        .fetch_one(&pool)
        .await?;

    Ok(Json(DashboardStats {
        total_posts,
        draft_posts,
        scheduled_posts,
        published_posts,
    }))
}

pub async fn clear_all_posts(
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM posts")
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
