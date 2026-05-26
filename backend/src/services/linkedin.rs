// backend/src/services/linkedin.rs
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::domain::models::{Settings, Post, PostStatus};
use crate::domain::errors::AppError;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::fs;
use tracing::{info, error, warn};

#[derive(Debug, Deserialize)]
struct LinkedInTokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
struct LinkedInUserInfo {
    sub: String,
}

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub authenticated: bool,
    pub simulated: bool,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

pub async fn get_auth_status(pool: &SqlitePool) -> Result<AuthStatusResponse, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let token = settings.linkedin_access_token.clone().unwrap_or_default();
    let is_mock = token.starts_with("mock_access_token");

    let is_auth = settings.linkedin_access_token.is_some() && 
        settings.linkedin_access_token_expires.map_or(false, |exp| exp > Utc::now());

    Ok(AuthStatusResponse {
        authenticated: is_auth && !is_mock,
        simulated: is_mock,
        expires_at: if is_mock { None } else { settings.linkedin_access_token_expires },
    })
}

pub async fn get_authorization_url(pool: &SqlitePool, public_url: &str) -> Result<Option<String>, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let client_id = match settings.linkedin_client_id {
        Some(cid) if !cid.trim().is_empty() => cid,
        _ => return Ok(None), // Indicativo para usar login simulado
    };

    let redirect_uri = format!("{}/api/auth/linkedin/callback", public_url);
    let auth_url = format!(
        "https://www.linkedin.com/oauth/v2/authorization?\
        response_type=code&\
        client_id={}&\
        redirect_uri={}&\
        state=linkedin_auth_state&\
        scope=w_member_social%20openid%20profile%20email",
        client_id, redirect_uri
    );

    Ok(Some(auth_url))
}

pub async fn handle_callback(
    pool: &SqlitePool,
    code: &str,
    public_url: &str,
) -> Result<(), AppError> {
    if code == "mock_code" {
        info!("Processando login simulado do LinkedIn.");
        let mock_token = "mock_access_token_portfolio_demo";
        let expires_at = Utc::now() + Duration::days(60);

        sqlx::query(
            "UPDATE settings SET linkedin_access_token = ?, linkedin_access_token_expires = ? WHERE id = 1"
        )
        .bind(mock_token)
        .bind(expires_at)
        .execute(pool)
        .await?;

        return Ok(());
    }

    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let client_id = settings.linkedin_client_id.ok_or_else(|| AppError::BadRequest("Client ID não configurado".to_string()))?;
    let client_secret = settings.linkedin_client_secret.ok_or_else(|| AppError::BadRequest("Client Secret não configurado".to_string()))?;
    let redirect_uri = format!("{}/api/auth/linkedin/callback", public_url);

    let client = reqwest::Client::new();
    let resp = client.post("https://www.linkedin.com/oauth/v2/accessToken")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &redirect_uri),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        let err_body = resp.text().await.unwrap_or_default();
        error!("Erro ao trocar código pelo token do LinkedIn: {}", err_body);
        return Err(AppError::LinkedIn(format!("Erro ao trocar código: {}", err_body)));
    }

    let token_res: LinkedInTokenResponse = resp.json().await?;
    let expires_at = Utc::now() + Duration::seconds(token_res.expires_in);

    sqlx::query(
        "UPDATE settings SET linkedin_access_token = ?, linkedin_access_token_expires = ? WHERE id = 1"
    )
    .bind(&token_res.access_token)
    .bind(expires_at)
    .execute(pool)
    .await?;

    info!("LinkedIn autenticado com sucesso. Token expira em: {}", expires_at);
    Ok(())
}

pub async fn disconnect(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE settings SET linkedin_access_token = NULL, linkedin_access_token_expires = NULL WHERE id = 1"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn publish_post(
    pool: &SqlitePool,
    post_id: &str,
) -> Result<String, AppError> {
    // Carregar post
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id, is_automated FROM posts WHERE id = ?"
    )
    .bind(post_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post não encontrado".to_string()))?;

    // Carregar configurações e token
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let token = match settings.linkedin_access_token {
        Some(t) if !t.trim().is_empty() => t,
        _ => return Err(AppError::BadRequest("Não autenticado no LinkedIn. Faça o login nas configurações.".to_string())),
    };

    // Verificar se é token mockado
    if token.starts_with("mock_access_token") {
        info!("[SIMULAÇÃO] Publicando post no LinkedIn (Mock). ID: {}", post.id);
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        let mock_li_id = format!("urn:li:share:{}", Uuid::new_v4().simple());

        sqlx::query(
            "UPDATE posts SET status = ?, published_at = ?, linkedin_post_id = ? WHERE id = ?"
        )
        .bind(PostStatus::Published)
        .bind(Utc::now())
        .bind(&mock_li_id)
        .bind(post_id)
        .execute(pool)
        .await?;

        return Ok(mock_li_id);
    }

    // --- FLUXO REAL DE POSTAGEM NO LINKEDIN ---
    let client = reqwest::Client::new();
    
    // 1. Obter URN do Usuário (OpenID Connect)
    let user_info: LinkedInUserInfo = client.get("https://api.linkedin.com/v2/userinfo")
        .bearer_auth(&token)
        .send()
        .await?
        .json()
        .await?;

    let author_urn = format!("urn:li:person:{}", user_info.sub);
    let mut media_urn: Option<String> = None;

    // 2. Se tiver imagem local ou externa, obter bytes e fazer o upload usando /rest/images
    if let Some(img_url) = &post.image_url {
        let image_bytes = if img_url.starts_with("/uploads/") {
            let file_name = img_url.trim_start_matches("/uploads/");
            let file_path = format!("uploads/{}", file_name);
            fs::read(&file_path).ok()
        } else if img_url.starts_with("http") {
            info!("Baixando imagem externa do Google Search para upload no LinkedIn: {}", img_url);
            if let Ok(res) = client.get(img_url).send().await {
                res.bytes().await.map(|b| b.to_vec()).ok()
            } else {
                None
            }
        } else {
            None
        };

        if let Some(bytes) = image_bytes {
            info!("Registrando imagem no LinkedIn REST API para upload.");
            
            // Registro do asset da imagem
            let register_payload = serde_json::json!({
                "initializeUploadRequest": {
                    "owner": author_urn
                }
            });

            let reg_resp = client.post("https://api.linkedin.com/rest/images?action=initializeUpload")
                .bearer_auth(&token)
                .header("LinkedIn-Version", "202603")
                .header("X-Restli-Protocol-Version", "2.0.0")
                .json(&register_payload)
                .send()
                .await?;

            if reg_resp.status().is_success() {
                let reg_json: serde_json::Value = reg_resp.json().await?;

                let upload_url = reg_json["value"]["uploadUrl"]
                    .as_str()
                    .ok_or_else(|| AppError::LinkedIn("URL de upload ausente no retorno de initializeUpload".to_string()))?;

                let asset_id = reg_json["value"]["image"]
                    .as_str()
                    .ok_or_else(|| AppError::LinkedIn("Asset ID (image) ausente no retorno de initializeUpload".to_string()))?;

                // Realizar upload binário via PUT
                info!("Fazendo upload binário PUT para o LinkedIn no endpoint temporário.");
                let upload_resp = client.put(upload_url)
                    .bearer_auth(&token)
                    .body(bytes)
                    .header("Content-Type", "image/jpeg")
                    .send()
                    .await?;

                if upload_resp.status().is_success() {
                    media_urn = Some(asset_id.to_string());
                    info!("Imagem enviada e registrada com sucesso: {}", asset_id);
                } else {
                    let err_text = upload_resp.text().await.unwrap_or_default();
                    warn!("Falha no upload binário PUT da imagem, publicando apenas texto. Resposta: {}", err_text);
                }
            } else {
                let err_text = reg_resp.text().await.unwrap_or_default();
                warn!("Falha ao registrar upload via rest/images. Resposta: {}", err_text);
            }
        }
    }

    // 3. Montar payload do Post versionado (/rest/posts)
    let mut post_payload = serde_json::json!({
        "author": author_urn,
        "commentary": post.content.clone(),
        "visibility": "PUBLIC",
        "distribution": {
            "feedDistribution": "MAIN_FEED"
        },
        "lifecycleState": "PUBLISHED"
    });

    if let Some(urn) = media_urn {
        post_payload["content"] = serde_json::json!({
            "media": {
                "altText": post.title.clone(),
                "id": urn
            }
        });
    }

    info!("Enviando post para o feed do LinkedIn via /rest/posts...");
    let post_resp = client.post("https://api.linkedin.com/rest/posts")
        .bearer_auth(&token)
        .header("LinkedIn-Version", "202603")
        .header("X-Restli-Protocol-Version", "2.0.0")
        .json(&post_payload)
        .send()
        .await?;

    if !post_resp.status().is_success() {
        let err_body = post_resp.text().await.unwrap_or_default();
        error!("Erro no LinkedIn API ao criar post via /rest/posts: {}", err_body);
        return Err(AppError::LinkedIn(format!("LinkedIn API retornou erro: {}", err_body)));
    }

    // Extrair ID do post criado do header x-restli-id
    let li_id = if let Some(id_header) = post_resp.headers().get("x-restli-id") {
        id_header.to_str().unwrap_or_default().to_string()
    } else {
        let post_json: serde_json::Value = post_resp.json().await.unwrap_or_default();
        post_json["id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!("urn:li:share:{}", Uuid::new_v4().simple())
            })
    };

    // Atualizar status no banco
    sqlx::query(
        "UPDATE posts SET status = ?, published_at = ?, linkedin_post_id = ? WHERE id = ?"
    )
    .bind(PostStatus::Published)
    .bind(Utc::now())
    .bind(&li_id)
    .bind(post_id)
    .execute(pool)
    .await?;

    info!("Post publicado com sucesso no LinkedIn: {}", li_id);
    Ok(li_id)
}
