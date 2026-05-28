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

// Google Imagen 4 API formats
#[derive(Debug, Serialize)]
struct ImagenRequest {
    instances: Vec<ImagenInstance>,
    parameters: ImagenParameters,
}

#[derive(Debug, Serialize)]
struct ImagenInstance {
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ImagenParameters {
    #[serde(rename = "sampleCount")]
    sample_count: u32,
    #[serde(rename = "outputMimeType")]
    output_mime_type: String,
    #[serde(rename = "aspectRatio")]
    aspect_ratio: String,
}

#[derive(Debug, Deserialize)]
struct ImagenResponse {
    predictions: Option<Vec<Prediction>>,
}

#[derive(Debug, Deserialize)]
struct Prediction {
    #[serde(rename = "bytesBase64Encoded")]
    bytes_base64_encoded: Option<String>,
}

pub async fn generate_text(
    pool: &SqlitePool,
    topic: &str,
    prompt_override: &Option<String>,
) -> Result<TextGenerationResponse, AppError> {
    // Carregar configurações
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
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
    let model = "gemini-2.5-flash"; // Modelo rápido e ótimo para textos
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let mut system_instruction = "Você é um profissional sênior de tecnologia e engenharia de software muito ativo no LinkedIn. \
    Sua tarefa é criar um post para o LinkedIn que seja curto, extremamente coeso, dinâmico e que pareça ter sido escrito por uma pessoa real de verdade, e não por uma IA corporativa. \
    REGRAS CRÍTICAS DE ESCRITA: \
    1. ESCREVA SEMPRE EM PORTUGUÊS (do Brasil). \
    2. Seja conciso: limite o post a no máximo 3 ou 4 parágrafos curtos. Nunca gere tutoriais longos ou múltiplos blocos de código gigantescos. \
    3. Use um tom de conversa natural e de bastidores (pessoal e direto). Fale em primeira pessoa ('eu' ou 'nós'). \
    4. Use pouquíssimos emojis (no máximo 2 ou 3) para não parecer spam comercial. \
    5. Termine com uma pergunta curta de engajamento e no máximo 3 a 4 hashtags relevantes no final. \
    6. Não use cabeçalhos artificiais (como 'Post:', 'Título:', etc.). Retorne apenas o texto final pronto. \
    7. EVITE REPETIÇÕES: Não use frases de abertura clichês ou repetitivas como 'Recentemente tenho explorado...', 'Nos últimos tempos...', 'No ecossistema de...'. Comece diretamente com uma reflexão, um fato, uma provocação ou um aprendizado prático. \
    8. NÃO EXAGERE NO PERFIL/BIOGRAFIA: Não liste suas experiências, cargo atual, tempo de carreira ou histórico profissional em todos os posts. O contexto do autor serve APENAS para ajustar o tom de voz, estilo técnico e vocabulário. Não faça autopromoção explícita ou repetida das mesmas informações biográficas.".to_string();

    if let Some(ref context) = settings.user_context {
        if !context.trim().is_empty() {
            system_instruction.push_str(&format!(
                "\n\nCONTEXTO DO AUTOR (Use este contexto APENAS para alinhar a personalidade, tom de voz e estilo técnico. NÃO repita esses detalhes biográficos no post):\n{}",
                context.trim()
            ));
        }
    }

    let prompt = match prompt_override {
        Some(po) if !po.trim().is_empty() => format!("{}\nDiretrizes adicionais: {}\nTópico do post: '{}'", system_instruction, po, topic),
        _ => format!("{}\nTópico do post: '{}'", system_instruction, topic)
    };

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.8,
            "topP": 0.95,
            "topK": 40
        }
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
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.gemini_key.is_some() && !settings.gemini_key.as_ref().unwrap().trim().is_empty();

    // Criar pasta de uploads se não existir
    let uploads_dir = std::env::var("UPLOADS_DIR").unwrap_or_else(|_| "uploads".to_string());
    fs::create_dir_all(&uploads_dir).ok();

    if !has_key || prompt.to_lowercase().contains("mock") {
        info!("Rodando geração de imagem no modo Simulação (Mock).");
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let mock_image_id = Uuid::new_v4().to_string();
        let file_path = format!("{}/{}.jpg", uploads_dir, mock_image_id);
        
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
    // Endpoint para Imagen 4 no Google AI Studio (modelo do Gemini API para imagens)
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/imagen-4.0-generate-001:predict?key={}",
        api_key
    );

    let body = ImagenRequest {
        instances: vec![ImagenInstance {
            prompt: prompt.to_string(),
        }],
        parameters: ImagenParameters {
            sample_count: 1,
            output_mime_type: "image/jpeg".to_string(),
            aspect_ratio: "1:1".to_string(),
        },
    };

    info!("Enviando solicitação de geração de imagem para o Imagen 4 com prompt: {}", prompt);

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

    let base64_image = imagen_resp.predictions
        .and_then(|preds| preds.into_iter().next())
        .and_then(|pred| pred.bytes_base64_encoded)
        .ok_or_else(|| AppError::Gemini("A API do Imagen 4 retornou uma resposta vazia ou sem bytes da imagem.".to_string()))?;

    // Decodificar base64 e salvar como arquivo local
    use base64::{Engine as _, engine::general_purpose};
    let image_bytes = general_purpose::STANDARD.decode(base64_image)
        .map_err(|e| AppError::Internal(format!("Erro ao decodificar imagem base64: {}", e)))?;

    let image_uuid = Uuid::new_v4().to_string();
    let file_path = format!("{}/{}.jpg", uploads_dir, image_uuid);
    
    fs::write(&file_path, &image_bytes)
        .map_err(|e| AppError::Internal(format!("Erro ao salvar arquivo de imagem gerada: {}", e)))?;

    let image_url = format!("/uploads/{}.jpg", image_uuid);
    info!("Imagem gerada com sucesso e salva em {}", file_path);

    Ok(ImageGenerationResponse { image_url })
}

pub async fn suggest_topics(
    pool: &SqlitePool,
    seed: Option<String>,
    quantity: Option<i32>,
) -> Result<Vec<String>, AppError> {
    // Carregar configurações
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.gemini_key.is_some() && !settings.gemini_key.as_ref().unwrap().trim().is_empty();
    let count = quantity.unwrap_or(4);

    if !has_key {
        info!("Rodando sugestão de tópicos no modo Simulação (Mock).");
        let base_topics = vec![
            "Rust 1.95 e const generics".to_string(),
            "Edge runtime migrations".to_string(),
            "Liderança técnica pragmática".to_string(),
            "Burnout em times de produto".to_string(),
            "Arquitetura de microsserviços resilientes".to_string(),
            "Boas práticas com SQLx e SQLite".to_string(),
            "O futuro do serverless e edge computing".to_string(),
            "Desenvolvimento dirigido a testes em Rust".to_string(),
            "Gerenciamento de estado fino em frontend".to_string(),
            "Como reduzir a latência de APIs críticas".to_string(),
        ];
        
        let mut mock_topics = Vec::new();
        let topic_seed = seed.as_deref().unwrap_or("tecnologia").to_lowercase();
        
        for base in &base_topics {
            if mock_topics.len() >= count as usize {
                break;
            }
            mock_topics.push(base.clone());
        }
        
        let mut index = 1;
        while mock_topics.len() < count as usize {
            mock_topics.push(format!("Tópico extra {} sobre {}", index, topic_seed));
            index += 1;
        }

        return Ok(mock_topics);
    }

    let api_key = settings.gemini_key.ok_or_else(|| AppError::Gemini("Gemini API Key ausente".to_string()))?;
    let model = "gemini-2.5-flash";
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let topic_seed = seed.unwrap_or_else(|| "tecnologia, engenharia de software e arquitetura".to_string());
    let prompt = format!(
        "Você é um estrategista de conteúdo para o LinkedIn. \
        Gere uma lista de exatamente {} ideias de tópicos curtos, específicos e muito engajadores para postagens no LinkedIn baseados no seguinte tema ou semente: '{}'. \
        Cada tópico deve abordar um sub-tema diferente, prático e interessante para evitar repetição. \
        Retorne apenas as ideias de tópicos, uma por linha, sem numeração, explicações, aspas ou cabeçalhos.",
        count, topic_seed
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_body = resp.text().await.unwrap_or_default();
        error!("Erro do Gemini API na sugestão de tópicos: {}", error_body);
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

    // Dividir as linhas e limpar
    let mut topics: Vec<String> = generated_text
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            let without_num = trimmed.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == ' ');
            without_num.trim_matches('"').trim().to_string()
        })
        .filter(|line| !line.is_empty())
        .take(count as usize)
        .collect();

    if topics.is_empty() {
        let base_topics = vec![
            "Rust 1.95 e const generics".to_string(),
            "Edge runtime migrations".to_string(),
            "Liderança técnica pragmática".to_string(),
            "Burnout em times de produto".to_string(),
        ];
        for base in base_topics {
            if topics.len() >= count as usize {
                break;
            }
            topics.push(base);
        }
    }

    Ok(topics)
}

pub async fn improve_post(
    pool: &SqlitePool,
    content: &str,
    mode: &str,
) -> Result<String, AppError> {
    // Carregar configurações
    let settings = sqlx::query_as::<_, Settings>(
        "SELECT id, gemini_key, google_search_key, google_search_cx, linkedin_client_id, linkedin_client_secret, linkedin_access_token, linkedin_access_token_expires, pexels_key, user_context FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let has_key = settings.gemini_key.is_some() && !settings.gemini_key.as_ref().unwrap().trim().is_empty();

    if !has_key || content.to_lowercase().contains("mock") {
        info!("Rodando melhoria de post no modo Simulação (Mock).");
        // Simulação de delay da I.A.
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        let improved = match mode {
            "professional" => format!(
                "💼 **[Versão Profissional]**\n\n\
                Gostaria de compartilhar uma reflexão sobre a evolução profissional e técnica. {}\n\n\
                Como você enxerga essa evolução em sua trajetória? Compartilhe nos comentários. 👇\n\n\
                #LiderancaTecnica #Carreira #Desenvolvimento",
                content
            ),
            "storytelling" => format!(
                "📖 **[Versão Storytelling]**\n\n\
                Tudo começou com um desafio simples, mas que me fez repensar toda a abordagem técnica.\n\n\
                A realidade é esta: {}\n\n\
                No final do dia, o maior aprendizado foi a importância de iterar rápido e com foco em qualidade. E você, já passou por algo parecido?\n\n\
                #Storytelling #EngenhariaDeSoftware #Aprendizado",
                content
            ),
            "direct" => format!(
                "🎯 **[Versão Direta]**\n\n\
                Direto ao ponto: {}\n\n\
                Concorda com essa visão? Comente abaixo.\n\n\
                #Produtividade #Clareza",
                content
            ),
            "persuasive" => format!(
                "🔥 **[Versão Persuasiva]**\n\n\
                Você realmente está aproveitando todo o potencial da sua stack?\n\n\
                Se o seu objetivo é escalar com segurança e performance, preste atenção nisso: {}\n\n\
                Não perca tempo com soluções ultrapassadas. O que você acha? 👇\n\n\
                #Inovacao #Resultados #Tecnologia",
                content
            ),
            _ => format!("✨ **[Versão Enriquecida]**\n\n{}\n\n#Linkedin", content)
        };

        return Ok(improved);
    }

    let api_key = settings.gemini_key.ok_or_else(|| AppError::Gemini("Gemini API Key ausente".to_string()))?;
    let model = "gemini-3.5-flash"; // Usar o melhor modelo conforme escolha do usuário
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let system_instruction = match mode {
        "professional" => "Você é um revisor especialista de posts do LinkedIn. Sua tarefa é reescrever o texto fornecido pelo usuário para torná-lo mais profissional, maduro e polido. Mantenha a essência e a mensagem original, mas melhore a escolha de palavras, o ritmo e a estrutura. Use um tom executivo e experiente. Não use muitos emojis (no máximo 2). Não use títulos ou cabeçalhos artificiais, retorne apenas o texto do post final.",
        "storytelling" => "Você é um escritor especialista em storytelling para o LinkedIn. Sua tarefa é reescrever o post do usuário adicionando um gancho (hook) inicial forte, narrando como uma história ou aprendizado prático e mantendo a leitura fluida e envolvendo o leitor. Torne-o atraente e emocionante. Não use títulos artificiais, retorne apenas o texto do post final.",
        "direct" => "Você é um editor focado em clareza e concisão. Sua tarefa é simplificar e reescrever o post do usuário, tornando-o extremamente direto ao ponto, curto e impactante. Elimine rodeios, palavras vazias e parágrafos desnecessários. Deixe as ideias principais brilharem. Retorne apenas o texto final pronto.",
        "persuasive" => "Você é um especialista em copywriting persuasivo para o LinkedIn. Sua tarefa é reescrever o post do usuário de forma a engajar fortemente o leitor, criando curiosidade, valor claro e usando técnicas de escrita persuasiva. Retorne apenas o texto final pronto.",
        _ => "Você é um assistente de escrita para o LinkedIn. Melhore e enriqueça o post fornecido, corrigindo a gramática, otimizando os parágrafos e garantindo boa legibilidade. Adicione hashtags relevantes e uma pergunta instigante ao final se julgar necessário. Retorne apenas o texto final pronto."
    };

    let prompt = format!("{}\n\nTexto original para reescrever:\n\"\"\"\n{}\n\"\"\"", system_instruction, content);

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    info!("Enviando solicitação de melhoria de post para o Gemini usando o modelo {}.", model);

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

    Ok(generated_text)
}
