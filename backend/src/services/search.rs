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
struct GoogleSearchResponse {
    items: Option<Vec<GoogleSearchItem>>,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchItem {
    title: Option<String>,
    link: Option<String>,
    image: Option<GoogleImageDetail>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct GoogleImageDetail {
    thumbnailLink: Option<String>,
}

pub async fn search_images(
    pool: &SqlitePool,
    query: &str,
) -> Result<Vec<ImageSearchResult>, AppError> {
    // Buscar chaves
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_keys = settings.google_search_key.is_some() && settings.google_search_cx.is_some();

    if !has_keys || query.to_lowercase().contains("mock") {
        info!("Rodando busca de imagens no modo Simulação (Mock).");
        // Gerar 6 imagens mockadas baseado no termo de busca para parecer realista
        let clean_query = query.replace(' ', "+");
        let mock_results = vec![
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
        ];
        return Ok(mock_results);
    }

    // Modo real: chamando a API do Google Custom Search
    let key = settings.google_search_key.ok_or_else(|| AppError::BadRequest("Chave Google Search API ausente".to_string()))?;
    let cx = settings.google_search_cx.ok_or_else(|| AppError::BadRequest("ID do mecanismo de busca Google CX ausente".to_string()))?;
    
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&searchType=image&num=6",
        key, cx, query
    );

    info!("Buscando imagens na API do Google para a query: {}", query);

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err_text = resp.text().await.unwrap_or_default();
        warn!("Google API Error: {}", err_text);
        return Err(AppError::Internal(format!("Google API returned status {}: {}", status, err_text)));
    }

    let search_res: GoogleSearchResponse = resp.json().await?;

    let results = search_res.items.unwrap_or_default().into_iter().filter_map(|item| {
        let title = item.title.unwrap_or_else(|| "Google Image".to_string());
        let link = item.link?;
        let thumbnail = item.image.and_then(|img| img.thumbnailLink).unwrap_or_else(|| link.clone());
        Some(ImageSearchResult { title, link, thumbnail })
    }).collect::<Vec<_>>();

    Ok(results)
}
