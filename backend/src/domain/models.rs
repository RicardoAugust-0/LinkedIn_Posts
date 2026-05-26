// backend/src/domain/models.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Scheduled,
    Published,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ImageSource {
    Google,
    Ai,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub content: String,
    pub image_url: Option<String>,
    pub image_source: ImageSource,
    pub status: PostStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub linkedin_post_id: Option<String>,
    #[sqlx(default)]
    pub is_automated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Settings {
    pub id: i32,
    pub gemini_key: Option<String>,
    pub google_search_key: Option<String>,
    pub google_search_cx: Option<String>,
    pub linkedin_client_id: Option<String>,
    pub linkedin_client_secret: Option<String>,
    pub linkedin_access_token: Option<String>,
    pub linkedin_access_token_expires: Option<DateTime<Utc>>,
    pub pexels_key: Option<String>,
    pub user_context: Option<String>,
    #[sqlx(default)]
    pub campaign_active: bool,
    #[sqlx(default)]
    pub campaign_topic: Option<String>,
    #[sqlx(default)]
    pub campaign_quantity: Option<i32>,
    #[sqlx(default)]
    pub campaign_cadence: Option<String>,
    #[sqlx(default)]
    pub campaign_windows: Option<String>,
    #[sqlx(default)]
    pub campaign_tone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub topic: String,
    pub prompt_override: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImprovePostRequest {
    pub content: String,
    pub mode: String,
}

#[derive(Debug, Serialize)]
pub struct ImprovePostResponse {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateImageRequest {
    pub prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchImagesRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct SavePostRequest {
    pub title: String,
    pub topic: String,
    pub content: String,
    pub image_url: Option<String>,
    pub image_source: ImageSource,
    pub status: PostStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub is_automated: bool,
}

#[derive(Debug, Deserialize)]
pub struct SaveSettingsRequest {
    pub gemini_key: Option<String>,
    pub google_search_key: Option<String>,
    pub google_search_cx: Option<String>,
    pub linkedin_client_id: Option<String>,
    pub linkedin_client_secret: Option<String>,
    pub pexels_key: Option<String>,
    pub user_context: Option<String>,
    pub campaign_active: Option<bool>,
    pub campaign_topic: Option<String>,
    pub campaign_quantity: Option<i32>,
    pub campaign_cadence: Option<String>,
    pub campaign_windows: Option<String>,
    pub campaign_tone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_posts: i64,
    pub draft_posts: i64,
    pub scheduled_posts: i64,
    pub published_posts: i64,
}
