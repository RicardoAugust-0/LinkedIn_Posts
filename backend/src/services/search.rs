// backend/src/services/search.rs
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::domain::models::Settings;
use crate::domain::errors::AppError;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchResult {
    pub title: String,
    pub link: String,
    pub thumbnail: String,
}

#[derive(Debug, Deserialize)]
struct PexelsResponse {
    photos: Option<Vec<PexelsPhoto>>,
}

#[derive(Debug, Deserialize)]
struct PexelsPhoto {
    alt: Option<String>,
    src: Option<PexelsPhotoSrc>,
}

#[derive(Debug, Deserialize)]
struct PexelsPhotoSrc {
    large: Option<String>,
    medium: Option<String>,
}

pub async fn search_images(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ImageSearchResult>, AppError> {
    // Buscar chaves
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.pexels_key.as_ref().map(|k| !k.trim().is_empty()).unwrap_or(false);

    if !has_key || query.to_lowercase().contains("mock") {
        info!("Rodando busca de imagens no modo Simulação (Mock).");
        return Ok(get_mock_results(query));
    }

    // Modo real: chamando a API do Pexels
    let key = settings.pexels_key.ok_or_else(|| AppError::BadRequest("Chave Pexels API ausente".to_string()))?;
    
    let url = "https://api.pexels.com/v1/search";

    info!("Buscando imagens na API do Pexels para a query: {}", query);

    let client = reqwest::Client::new();
    let resp = match client.get(url)
        .header("Authorization", &key)
        .query(&[("query", query), ("per_page", "6")])
        .send()
        .await 
    {
        Ok(r) => r,
        Err(e) => {
            warn!("Erro ao enviar requisição para Pexels API: {}. Usando fallback mock.", e);
            return Ok(get_mock_results(query));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_default();
        warn!("Pexels API Error (Status {}): {}. Usando fallback mock.", status, err_text);
        return Ok(get_mock_results(query));
    }

    let search_res: PexelsResponse = match resp.json().await {
        Ok(res) => res,
        Err(e) => {
            warn!("Erro ao ler JSON da API Pexels: {}. Usando fallback mock.", e);
            return Ok(get_mock_results(query));
        }
    };

    let results = search_res.photos.unwrap_or_default().into_iter().filter_map(|photo| {
        let title = photo.alt.filter(|t| !t.trim().is_empty()).unwrap_or_else(|| format!("Imagem Pexels: {}", query));
        let src = photo.src?;
        let link = src.large?;
        let thumbnail = src.medium.unwrap_or(link.clone());
        Some(ImageSearchResult { title, link, thumbnail })
    }).collect::<Vec<_>>();

    Ok(results)
}

fn get_mock_results(query: &str) -> Vec<ImageSearchResult> {
    let clean_query = query.replace(' ', "+");
    vec![
        ImageSearchResult {
            title: format!("Referência visual para: {}", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}1", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}1", clean_query)),
        },
        ImageSearchResult {
            title: format!("Inspiração de tecnologia: {}", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}2", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}2", clean_query)),
        },
        ImageSearchResult {
            title: format!("Diagrama corporativo - {}", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}3", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}3", clean_query)),
        },
        ImageSearchResult {
            title: format!("Design conceitual de {}", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}4", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}4", clean_query)),
        },
        ImageSearchResult {
            title: format!("Espaço de trabalho de desenvolvimento ({})", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}5", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}5", clean_query)),
        },
        ImageSearchResult {
            title: format!("Ilustração de IA sobre {}", query),
            link: format!("https://picsum.photos/seed/{}/800/600", format!("{}6", clean_query)),
            thumbnail: format!("https://picsum.photos/seed/{}/300/200", format!("{}6", clean_query)),
        },
    ]
}
