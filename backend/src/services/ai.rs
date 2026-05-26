// backend/src/services/ai.rs
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::domain::models::Settings;
use crate::domain::errors::AppError;
use tracing::{info, error};
use std::fs;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct TextGenerationResponse {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ImageGenerationResponse {
    pub image_url: String,
}

// Gemini API response formats
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Option<Vec<GeminiPart>>,
}

#[derive(Debug, Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

// Google Imagen 3 API formats
#[derive(Debug, Serialize)]
struct ImagenRequest {
    prompt: String,
    #[serde(rename = "numberOfImages")]
    number_of_images: u32,
    #[serde(rename = "outputMimeType")]
    output_mime_type: String,
    #[serde(rename = "aspectRatio")]
    aspect_ratio: String,
}

#[derive(Debug, Deserialize)]
struct ImagenResponse {
    #[serde(rename = "generatedImages")]
    generated_images: Option<Vec<GeneratedImage>>,
}

#[derive(Debug, Deserialize)]
struct GeneratedImage {
    image: Option<ImageBytesContainer>,
}

#[derive(Debug, Deserialize)]
struct ImageBytesContainer {
    #[serde(rename = "imageBytes")]
    image_bytes: Option<String>, // Base64
}

pub async fn generate_text(
    pool: &SqlitePool,
    topic: &str,
    prompt_override: &Option<String>,
) -> Result<TextGenerationResponse, AppError> {
    // Carregar configurações
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.gemini_key.is_some() && !settings.gemini_key.as_ref().unwrap().trim().is_empty();

    if !has_key || topic.to_lowercase().contains("mock") {
        info!("Rodando geração de texto no modo Simulação (Mock).");
        // Simulação de delay da I.A.
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        
        let title = format!("Dominando o(a) {}", topic);
        let content = format!(
            "🚀 **Construindo o futuro com {}!**\n\n\
            Recentemente tenho explorado a fundo o ecossistema de **{}** e o impacto que ele traz para arquiteturas modernas de software. A combinação de eficiência, scalabilidade e robustez é um diferencial enorme para qualquer equipe técnica.\n\n\
            Aqui estão 3 motivos pelos quais você deve prestar atenção nisso:\n\n\
            1️⃣ **Performance Absoluta**: Otimização em nível de sistema sem comprometer a facilidade de manutenção.\n\
            2️⃣ **Segurança em Primeiro Lugar**: Redução drástica de erros comuns em tempo de execução através de checagens rigorosas do compilador/interpretador.\n\
            3️⃣ **Produtividade Acelerada**: Ferramentas modernas que automatizam testes, builds e o gerenciamento de dependências de forma transparente.\n\n\
            Qual tem sido a sua experiência com {} em produção? Deixe seu comentário abaixo! 👇\n\n\
            #Tecnologia #SoftwareEngineering #Rust #Inovacao #WebDev",
            topic, topic, topic
        );

        return Ok(TextGenerationResponse { title, content });
    }

    let api_key = settings.gemini_key.ok_or_else(|| AppError::Gemini("Gemini API Key ausente".to_string()))?;
    let model = "gemini-1.5-flash"; // Modelo rápido e ótimo para textos
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let system_instruction = "Você é um especialista em marketing B2B e engenheiro de software experiente. \
    Sua tarefa é criar um post de LinkedIn engajador, profissional e de alta qualidade baseado no tópico fornecido pelo usuário. \
    O post deve incluir emoticons bem colocados, hashtags relevantes no final, formatação clara com espaçamentos \
    e uma pergunta de engajamento no final. Não use cabeçalhos como 'Post:', apenas retorne o texto final do post.";

    let prompt = match prompt_override {
        Some(po) if !po.trim().is_empty() => format!("{}. Tema específico: {}", system_instruction, po),
        _ => format!("Escreva um post do LinkedIn atraente e informativo sobre o tópico: '{}'. Siga as instruções de formato.", topic)
    };

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    info!("Enviando solicitação de geração de texto para o Gemini para o tema: {}", topic);

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_body = resp.text().await.unwrap_or_default();
        error!("Erro do Gemini API: {}", error_body);
        return Err(AppError::Gemini(format!("Gemini API retornou erro: {}", error_body)));
    }

    let gemini_resp: GeminiResponse = resp.json().await?;

    let generated_text = gemini_resp.candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|cand| cand.content)
        .and_then(|cont| cont.parts)
        .and_then(|p| p.into_iter().next())
        .and_then(|part| part.text)
        .ok_or_else(|| AppError::Gemini("A API do Gemini retornou uma resposta sem texto estruturado.".to_string()))?;

    let title = format!("Explorando {}", topic);

    Ok(TextGenerationResponse {
        title,
        content: generated_text,
    })
}

pub async fn generate_image(
    pool: &SqlitePool,
    prompt: &str,
) -> Result<ImageGenerationResponse, AppError> {
    // Carregar configurações
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.gemini_key.is_some() && !settings.gemini_key.as_ref().unwrap().trim().is_empty();

    // Criar pasta de uploads se não existir
    fs::create_dir_all("uploads").ok();

    if !has_key || prompt.to_lowercase().contains("mock") {
        info!("Rodando geração de imagem no modo Simulação (Mock).");
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let mock_image_id = Uuid::new_v4().to_string();
        let file_path = format!("uploads/{}.jpg", mock_image_id);
        
        let client = reqwest::Client::new();
        // Baixar imagem do picsum
        if let Ok(res) = client.get("https://picsum.photos/800/800").send().await {
            if let Ok(bytes) = res.bytes().await {
                if fs::write(&file_path, bytes).is_ok() {
                    let image_url = format!("/uploads/{}.jpg", mock_image_id);
                    return Ok(ImageGenerationResponse { image_url });
                }
            }
        }

        // Fallback do mockup se a rede falhar
        return Ok(ImageGenerationResponse {
            image_url: "https://picsum.photos/seed/ai-gen/800/800".to_string()
        });
    }

    let api_key = settings.gemini_key.ok_or_else(|| AppError::Gemini("Gemini API Key ausente".to_string()))?;
    // Endpoint para Imagen 3 no Google AI Studio (modelo do Gemini API para imagens)
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/imagen-3.0-generate-002:generateImages?key={}",
        api_key
    );

    let body = ImagenRequest {
        prompt: prompt.to_string(),
        number_of_images: 1,
        output_mime_type: "image/jpeg".to_string(),
        aspect_ratio: "1:1".to_string(),
    };

    info!("Enviando solicitação de geração de imagem para o Imagen 3 com prompt: {}", prompt);

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_body = resp.text().await.unwrap_or_default();
        error!("Erro do Imagen API: {}", error_body);
        return Err(AppError::Gemini(format!("Imagen API retornou erro: {}", error_body)));
    }

    let imagen_resp: ImagenResponse = resp.json().await?;

    let base64_image = imagen_resp.generated_images
        .and_then(|images| images.into_iter().next())
        .and_then(|img| img.image)
        .and_then(|c| c.image_bytes)
        .ok_or_else(|| AppError::Gemini("A API do Imagen 3 retornou uma resposta vazia ou sem bytes da imagem.".to_string()))?;

    // Decodificar base64 e salvar como arquivo local
    use base64::{Engine as _, engine::general_purpose};
    let image_bytes = general_purpose::STANDARD.decode(base64_image)
        .map_err(|e| AppError::Internal(format!("Erro ao decodificar imagem base64: {}", e)))?;

    let image_uuid = Uuid::new_v4().to_string();
    let file_path = format!("uploads/{}.jpg", image_uuid);
    
    fs::write(&file_path, &image_bytes)
        .map_err(|e| AppError::Internal(format!("Erro ao salvar arquivo de imagem gerada: {}", e)))?;

    let image_url = format!("/uploads/{}.jpg", image_uuid);
    info!("Imagem gerada com sucesso e salva em {}", file_path);

    Ok(ImageGenerationResponse { image_url })
}
