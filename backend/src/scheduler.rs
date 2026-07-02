// backend/src/scheduler.rs
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use chrono::{Utc, DateTime};
use tracing::{info, warn, error};

/// Número máximo de tentativas de publicação antes de marcar o post como falho.
const MAX_PUBLISH_RETRIES: i64 = 3;
use crate::domain::models::{Post, PostStatus};
use crate::services::linkedin::{publish_post, get_auth_status, refresh_access_token};

pub async fn start_scheduler(pool: SqlitePool) {
    info!("Iniciando o agendador de posts em background...");

    // Última vez que avisamos sobre expiração de token (throttle: no máx 1x/6h).
    let mut last_token_warning: Option<DateTime<Utc>> = None;

    loop {
        // Dormir por 15 segundos antes da próxima checagem
        sleep(Duration::from_secs(15)).await;

        let now = Utc::now();

        // Token do LinkedIn perto de expirar: tenta renovar automaticamente;
        // só avisa se não houver refresh token disponível (checado no máx 1x/6h).
        if last_token_warning.map_or(true, |t| (now - t).num_hours() >= 6) {
            if let Ok(status) = get_auth_status(&pool).await {
                if status.reauth_soon {
                    match refresh_access_token(&pool).await {
                        Ok(true) => {
                            info!("Agendador: Token do LinkedIn renovado automaticamente antes de expirar.");
                        }
                        Ok(false) => {
                            warn!(
                                "Agendador: Token do LinkedIn expira em {} dia(s) e não há refresh token disponível. \
                                 Reautentique em Configurações para manter a publicação automática.",
                                status.days_until_expiry.unwrap_or(0),
                            );
                            last_token_warning = Some(now);
                        }
                        Err(e) => {
                            error!(
                                "Agendador: Falha ao renovar token do LinkedIn (expira em {} dia(s)): {:?}. \
                                 Reautentique manualmente se persistir.",
                                status.days_until_expiry.unwrap_or(0), e
                            );
                            last_token_warning = Some(now);
                        }
                    }
                }
            }
        }
        
        // Buscar posts agendados que já passaram da hora de publicação
        let scheduled_posts_res = sqlx::query_as::<_, Post>(
            "SELECT id, title, topic, content, image_url, image_source, status, scheduled_at, published_at, created_at, linkedin_post_id, is_automated \
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
                    // Limpar qualquer mensagem de erro de tentativas anteriores.
                    let _ = sqlx::query("UPDATE posts SET error_message = NULL WHERE id = ?")
                        .bind(&post_record.id)
                        .execute(&pool)
                        .await;
                }
                Err(err_msg) => {
                    let err_text = format!("{}", err_msg);

                    // Ler o número de tentativas atual e decidir entre reagendar ou desistir.
                    let current_retry: i64 = sqlx::query_scalar("SELECT retry_count FROM posts WHERE id = ?")
                        .bind(&post_record.id)
                        .fetch_one(&pool)
                        .await
                        .unwrap_or(0);
                    let new_retry = current_retry + 1;

                    let update_res = if new_retry >= MAX_PUBLISH_RETRIES {
                        error!(
                            "Agendador: Post '{}' falhou definitivamente após {} tentativa(s). Erro: {}",
                            post_record.title, new_retry, err_text
                        );
                        sqlx::query(
                            "UPDATE posts SET status = ?, retry_count = ?, error_message = ? WHERE id = ?"
                        )
                        .bind(PostStatus::Failed)
                        .bind(new_retry)
                        .bind(&err_text)
                        .bind(&post_record.id)
                        .execute(&pool)
                        .await
                    } else {
                        // Backoff progressivo: reagenda para o futuro (5min * tentativa).
                        let next_attempt = now + chrono::Duration::minutes(5 * new_retry);
                        warn!(
                            "Agendador: Falha ao publicar '{}' (tentativa {}/{}). Reagendado para {}. Erro: {}",
                            post_record.title, new_retry, MAX_PUBLISH_RETRIES, next_attempt, err_text
                        );
                        sqlx::query(
                            "UPDATE posts SET retry_count = ?, error_message = ?, scheduled_at = ? WHERE id = ?"
                        )
                        .bind(new_retry)
                        .bind(&err_text)
                        .bind(next_attempt)
                        .bind(&post_record.id)
                        .execute(&pool)
                        .await
                    };

                    if let Err(db_err) = update_res {
                        error!("Agendador: Erro ao atualizar post após falha de publicação no banco: {}", db_err);
                    }
                }
            }
        }
    }
}
