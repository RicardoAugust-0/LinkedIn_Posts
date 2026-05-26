// backend/src/handlers/settings.rs
use axum::{extract::State, Json};
use sqlx::SqlitePool;
use crate::domain::models::{Settings, SaveSettingsRequest};
use crate::domain::errors::AppError;

pub async fn get_settings(
    State(pool): State<SqlitePool>,
) -> Result<Json<Settings>, AppError> {
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context, campaign_active, campaign_topic, campaign_quantity, campaign_cadence, campaign_windows, campaign_tone FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(settings))
}

pub async fn save_settings(
    State(pool): State<SqlitePool>,
    Json(req): Json<SaveSettingsRequest>,
) -> Result<Json<Settings>, AppError> {
    // 1. Carregar configurações existentes para fazer merge parcial
    let existing = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context, campaign_active, campaign_topic, campaign_quantity, campaign_cadence, campaign_windows, campaign_tone FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    // 2. Mesclar campos do request ou manter existentes
    let gemini_key = req.gemini_key.or(existing.gemini_key);
    let google_search_key = req.google_search_key.or(existing.google_search_key);
    let google_search_cx = req.google_search_cx.or(existing.google_search_cx);
    let linkedin_client_id = req.linkedin_client_id.or(existing.linkedin_client_id);
    let linkedin_client_secret = req.linkedin_client_secret.or(existing.linkedin_client_secret);
    let pexels_key = req.pexels_key.or(existing.pexels_key);
    let user_context = req.user_context.or(existing.user_context);
    
    let campaign_active = req.campaign_active.unwrap_or(existing.campaign_active);
    let campaign_topic = req.campaign_topic.or(existing.campaign_topic);
    let campaign_quantity = req.campaign_quantity.or(existing.campaign_quantity);
    let campaign_cadence = req.campaign_cadence.or(existing.campaign_cadence);
    let campaign_windows = req.campaign_windows.or(existing.campaign_windows);
    let campaign_tone = req.campaign_tone.or(existing.campaign_tone);

    sqlx::query(
        "UPDATE settings SET \
            gemini_key = ?, \
            google_search_key = ?, \
            google_search_cx = ?, \
            linkedin_client_id = ?, \
            linkedin_client_secret = ?, \
            pexels_key = ?, \
            user_context = ?, \
            campaign_active = ?, \
            campaign_topic = ?, \
            campaign_quantity = ?, \
            campaign_cadence = ?, \
            campaign_windows = ?, \
            campaign_tone = ? \
         WHERE id = 1"
    )
    .bind(gemini_key)
    .bind(google_search_key)
    .bind(google_search_cx)
    .bind(linkedin_client_id)
    .bind(linkedin_client_secret)
    .bind(pexels_key)
    .bind(user_context)
    .bind(campaign_active)
    .bind(campaign_topic)
    .bind(campaign_quantity)
    .bind(campaign_cadence)
    .bind(campaign_windows)
    .bind(campaign_tone)
    .execute(&pool)
    .await?;

    // Retorna as configurações atualizadas
    let updated = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context, campaign_active, campaign_topic, campaign_quantity, campaign_cadence, campaign_windows, campaign_tone FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

#[derive(serde::Deserialize)]
pub struct TestAllRequest {
    pub gemini_key: Option<String>,
    pub linkedin_client_id: Option<String>,
    pub linkedin_client_secret: Option<String>,
    pub pexels_key: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TestResponse {
    pub success: bool,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct TestAllResponse {
    pub gemini: TestResponse,
    pub pexels: TestResponse,
    pub linkedin: TestResponse,
}

pub async fn test_all_credentials(
    State(pool): State<SqlitePool>,
    Json(req): Json<TestAllRequest>,
) -> Result<Json<TestAllResponse>, AppError> {
    // 1. Testar Gemini
    let gemini = match &req.gemini_key {
        Some(key) if !key.trim().is_empty() => {
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
                key
            );
            let body = serde_json::json!({
                "contents": [{ "parts": [{ "text": "test" }] }]
            });
            let client = reqwest::Client::new();
            match client.post(&url).json(&body).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        TestResponse {
                            success: true,
                            message: "Conexão com Gemini estabelecida! Chave válida.".to_string(),
                        }
                    } else {
                        let err_body = resp.text().await.unwrap_or_default();
                        let parsed: serde_json::Value = serde_json::from_str(&err_body).unwrap_or_default();
                        let error_msg = parsed["error"]["message"].as_str().unwrap_or(&err_body).to_string();
                        TestResponse {
                            success: false,
                            message: format!("Gemini rejeitou a chave: {}", error_msg),
                        }
                    }
                }
                Err(e) => TestResponse {
                    success: false,
                    message: format!("Erro de rede ao conectar com o Gemini: {}", e),
                },
            }
        }
        _ => TestResponse {
            success: false,
            message: "Chave do Gemini não fornecida.".to_string(),
        },
    };

    // 2. Testar Pexels
    let pexels = match &req.pexels_key {
        Some(key) if !key.trim().is_empty() => {
            let url = "https://api.pexels.com/v1/search?query=test&per_page=1";
            let client = reqwest::Client::new();
            match client.get(url)
                .header("Authorization", key)
                .send()
                .await 
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        TestResponse {
                            success: true,
                            message: "Conexão com Pexels estabelecida! Chave válida.".to_string(),
                        }
                    } else {
                        let status = resp.status();
                        let err_body = resp.text().await.unwrap_or_default();
                        TestResponse {
                            success: false,
                            message: format!("Pexels rejeitou a chave (Status {}): {}", status, err_body),
                        }
                    }
                }
                Err(e) => TestResponse {
                    success: false,
                    message: format!("Erro de rede ao conectar com Pexels: {}", e),
                },
            }
        }
        _ => TestResponse {
            success: false,
            message: "Chave de API do Pexels ausente.".to_string(),
        },
    };

    // 3. Testar LinkedIn
    // Primeiro, carregar o token salvo no banco de dados para ver se temos uma autenticação ativa
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(&pool)
    .await?;

    let linkedin = match &settings.linkedin_access_token {
        Some(token) if !token.trim().is_empty() => {
            if token.starts_with("mock_access_token") {
                TestResponse {
                    success: false,
                    message: "LinkedIn rodando em Modo de Simulação. Para testar credenciais reais, clique em 'Conectar Conta' para autenticar no LinkedIn.".to_string(),
                }
            } else {
                // Fazer requisição real para o perfil do usuário no LinkedIn
                let client = reqwest::Client::new();
                match client.get("https://api.linkedin.com/v2/userinfo")
                    .bearer_auth(token)
                    .send()
                    .await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            TestResponse {
                                success: true,
                                message: "Token de acesso válido! Conectado e ativo.".to_string(),
                            }
                        } else {
                            TestResponse {
                                success: false,
                                message: "Token de acesso inválido ou expirado. Clique em 'Conectar Conta' novamente.".to_string(),
                            }
                        }
                    }
                    Err(e) => TestResponse {
                        success: false,
                        message: format!("Erro de rede ao validar token do LinkedIn: {}", e),
                    },
                }
            }
        }
        _ => {
            // Sem token. Verificar se Client ID e Secret estão fornecidos no request
            match (&req.linkedin_client_id, &req.linkedin_client_secret) {
                (Some(cid), Some(sec)) if !cid.trim().is_empty() && !sec.trim().is_empty() => {
                    TestResponse {
                        success: false,
                        message: "Configurações preenchidas, mas a conta não está conectada. Clique em 'Conectar Conta'.".to_string(),
                    }
                }
                _ => TestResponse {
                    success: false,
                    message: "Credenciais de API do LinkedIn não configuradas.".to_string(),
                },
            }
        }
    };

    Ok(Json(TestAllResponse {
        gemini,
        pexels,
        linkedin,
    }))
}
