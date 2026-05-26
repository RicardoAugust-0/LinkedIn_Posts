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
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

pub async fn get_auth_status(pool: &SqlitePool) -> Result<AuthStatusResponse, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let is_auth = settings.linkedin_access_token.is_some() && 
        settings.linkedin_access_token_expires.map_or(false, |exp| exp > Utc::now());

    Ok(AuthStatusResponse {
        authenticated: is_auth,
        expires_at: settings.linkedin_access_token_expires,
    })
}

pub async fn get_authorization_url(pool: &SqlitePool, public_url: &str) -> Result<Option<String>, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
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
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
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
        "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id FROM posts WHERE id = ?"
    )
    .bind(post_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post não encontrado".to_string()))?;

    // Carregar configurações e token
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
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
    
    // 1. Obter URN do Usuário
    let user_info: LinkedInUserInfo = client.get("https://api.linkedin.com/v2/userinfo")
        .bearer_auth(&token)
        .send()
        .await?
        .json()
        .await?;

    let author_urn = format!("urn:li:person:{}", user_info.sub);
    let mut media_urn: Option<String> = None;

    // 2. Se tiver imagem local e for um post com imagem, fazer o upload da imagem
    if let Some(img_url) = &post.image_url {
        if img_url.starts_with("/uploads/") {
            let file_name = img_url.trim_start_matches("/uploads/");
            let file_path = format!("uploads/{}", file_name);

            if let Ok(file_bytes) = fs::read(&file_path) {
                info!("Registrando imagem no LinkedIn API para upload: {}", file_path);
                
                // Registro do asset da imagem
                let register_payload = serde_json::json!({
                    "registerUploadRequest": {
                        "recipes": ["urn:li:digitalmediaRecipe:feedshare-image"],
                        "owner": author_urn,
                        "supportedUploadMechanism": ["SYNCHRONOUS_UPLOAD"]
                    }
                });

                let reg_resp = client.post("https://api.linkedin.com/v2/assets?action=registerUpload")
                    .bearer_auth(&token)
                    .json(&register_payload)
                    .send()
                    .await?;

                if reg_resp.status().is_success() {
                    let reg_json: serde_json::Value = reg_resp.json().await?;

                    let upload_url = reg_json["value"]["uploadMechanism"]["com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest"]["uploadUrl"]
                        .as_str()
                        .ok_or_else(|| AppError::LinkedIn("URL de upload ausente no retorno".to_string()))?;

                    let asset_id = reg_json["value"]["asset"]
                        .as_str()
                        .ok_or_else(|| AppError::LinkedIn("Asset ID ausente no retorno".to_string()))?;

                    // Realizar upload binário
                    info!("Fazendo upload binário para o LinkedIn no endpoint temporário.");
                    let upload_resp = client.post(upload_url)
                        .bearer_auth(&token)
                        .body(file_bytes)
                        .header("Content-Type", "image/jpeg")
                        .send()
                        .await?;

                    if upload_resp.status().is_success() {
                        media_urn = Some(asset_id.to_string());
                        info!("Imagem enviada e registrada com sucesso: {}", asset_id);
                    } else {
                        let err_text = upload_resp.text().await.unwrap_or_default();
                        warn!("Falha no upload binário da imagem, publicando apenas texto. Resposta: {}", err_text);
                    }
                } else {
                    let err_text = reg_resp.text().await.unwrap_or_default();
                    warn!("Falha ao registrar upload no LinkedIn. Resposta: {}", err_text);
                }
            } else {
                warn!("Arquivo de imagem local não foi encontrado em: {}", file_path);
            }
        } else if img_url.starts_with("http") {
            info!("Baixando imagem externa do Google Search para upload no LinkedIn: {}", img_url);
            if let Ok(res) = client.get(img_url).send().await {
                if let Ok(bytes) = res.bytes().await {
                    let register_payload = serde_json::json!({
                        "registerUploadRequest": {
                            "recipes": ["urn:li:digitalmediaRecipe:feedshare-image"],
                            "owner": author_urn,
                            "supportedUploadMechanism": ["SYNCHRONOUS_UPLOAD"]
                        }
                    });

                    if let Ok(reg_resp) = client.post("https://api.linkedin.com/v2/assets?action=registerUpload").bearer_auth(&token).json(&register_payload).send().await {
                        if reg_resp.status().is_success() {
                            if let Ok(reg_json) = reg_resp.json::<serde_json::Value>().await {
                                if let (Some(upload_url), Some(asset_id)) = (
                                    reg_json["value"]["uploadMechanism"]["com.linkedin.digitalmedia.uploading.MediaUploadHttpRequest"]["uploadUrl"].as_str(),
                                    reg_json["value"]["asset"].as_str()
                                ) {
                                    if client.post(upload_url).bearer_auth(&token).body(bytes.to_vec()).header("Content-Type", "image/jpeg").send().await.is_ok() {
                                        media_urn = Some(asset_id.to_string());
                                        info!("Imagem externa enviada e registrada com sucesso: {}", asset_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Montar payload do UGC Post
    let mut share_media_category = "NONE";
    let mut media_list = serde_json::json!([]);

    if let Some(urn) = media_urn {
        share_media_category = "IMAGE";
        media_list = serde_json::json!([
            {
                "status": "READY",
                "description": {
                    "text": post.title.clone()
                },
                "media": urn,
                "title": {
                    "text": post.title.clone()
                }
            }
        ]);
    }

    let post_payload = serde_json::json!({
        "author": author_urn,
        "lifecycleState": "PUBLISHED",
        "specificContent": {
            "com.linkedin.ugc.ShareContent": {
                "shareCommentary": {
                    "text": post.content.clone()
                },
                "shareMediaCategory": share_media_category,
                "media": media_list
            }
        },
        "visibility": {
            "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
        }
    });

    info!("Enviando post para o feed do LinkedIn...");
    let post_resp = client.post("https://api.linkedin.com/v2/ugcPosts")
        .bearer_auth(&token)
        .json(&post_payload)
        .send()
        .await?;

    if !post_resp.status().is_success() {
        let err_body = post_resp.text().await.unwrap_or_default();
        error!("Erro no LinkedIn API ao criar UGC post: {}", err_body);
        return Err(AppError::LinkedIn(format!("LinkedIn API retornou erro: {}", err_body)));
    }

    let post_json: serde_json::Value = post_resp.json().await?;

    let li_id = post_json["id"]
        .as_str()
        .ok_or_else(|| AppError::LinkedIn("ID do post ausente no retorno".to_string()))?
        .to_string();

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
