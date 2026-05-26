// backend/src/scheduler.rs
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use tracing::{info, error};
use crate::domain::models::{Post, PostStatus};
use crate::services::linkedin::publish_post;

pub async fn start_scheduler(pool: SqlitePool) {
    info!("Iniciando o agendador de posts em background...");
    
    loop {
        // Dormir por 15 segundos antes da próxima checagem
        sleep(Duration::from_secs(15)).await;
        
        let now = Utc::now();
        
        // Buscar posts agendados que já passaram da hora de publicação
        let scheduled_posts_res = sqlx::query_as::<_, Post>(
            "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id \
             FROM posts WHERE status = ? AND scheduled_at <= ?"
        )
        .bind(PostStatus::Scheduled)
        .bind(now)
        .fetch_all(&pool)
        .await;

        let scheduled_posts = match scheduled_posts_res {
            Ok(posts) => posts,
            Err(e) => {
                error!("Agendador: Erro ao buscar posts agendados do banco: {}", e);
                continue;
            }
        };

        if !scheduled_posts.is_empty() {
            info!("Agendador: Detectados {} posts para publicação imediata.", scheduled_posts.len());
        }

        for post_record in scheduled_posts {
            info!("Agendador: Publicando post automaticamente: '{}' (ID: {})", post_record.title, post_record.id);
            
            match publish_post(&pool, &post_record.id).await {
                Ok(li_id) => {
                    info!("Agendador: Post '{}' publicado com sucesso no LinkedIn. ID Retornado: {}", post_record.title, li_id);
                }
                Err(err_msg) => {
                    error!("Agendador: Falha ao publicar post '{}'. Erro: {:?}", post_record.title, err_msg);
                    // Atualizar status para 'failed' no banco
                    let update_res = sqlx::query(
                        "UPDATE posts SET status = ? WHERE id = ?"
                    )
                    .bind(PostStatus::Failed)
                    .bind(&post_record.id)
                    .execute(&pool)
                    .await;
                    
                    if let Err(db_err) = update_res {
                        error!("Agendador: Erro ao atualizar status do post para falho no banco: {}", db_err);
                    }
                }
            }
        }
    }
}
