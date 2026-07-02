<!-- frontend/src/components/dashboard/DashboardPostDetails.svelte -->
<script lang="ts">
  import { ExternalLink, TrendingDown, AlertTriangle } from '@lucide/svelte';
  import PostPreview from '../PostPreview.svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let selectedPost: any = null;
  export let getPostEngagement: (postId: string, metric: 'likes' | 'comments' | 'shares') => string;
</script>

<div class="details-panel-card">
  {#if selectedPost}
    <div class="details-panel-header">
      <div class="details-header-text">
        <span class="panel-eyebrow">Post selecionado</span>
        <h2>{selectedPost.title}</h2>
      </div>
      {#if selectedPost.status === 'published' && selectedPost.linkedin_post_id}
        <a 
          href="https://www.linkedin.com/feed/update/{selectedPost.linkedin_post_id}" 
          target="_blank" 
          rel="noopener noreferrer" 
          class="studio-btn studio-btn-secondary details-linkedin-btn"
        >
          <span>Ver no LinkedIn</span>
          <ExternalLink size={11} class="link-icon" />
        </a>
      {:else if selectedPost.status === 'draft' || selectedPost.status === 'failed'}
        <button 
          class="studio-btn studio-btn-secondary"
          on:click={() => dispatch('edit', selectedPost)}
        >
          <span>Editar rascunho</span>
        </button>
      {:else if selectedPost.status === 'scheduled'}
        <button 
          class="studio-btn studio-btn-secondary"
          on:click={() => dispatch('edit', selectedPost)}
        >
          <span>Editar agendamento</span>
        </button>
      {/if}
    </div>

    <!-- Aviso de falha / retry de publicação -->
    {#if selectedPost.status === 'failed' && selectedPost.error_message}
      <div class="failure-notice">
        <AlertTriangle size={14} class="notice-icon" />
        <div class="notice-body">
          <strong>Falha ao publicar</strong> (após {selectedPost.retry_count} {selectedPost.retry_count === 1 ? 'tentativa' : 'tentativas'})
          <span class="notice-detail">{selectedPost.error_message}</span>
        </div>
      </div>
    {:else if selectedPost.status === 'scheduled' && selectedPost.retry_count > 0}
      <div class="retry-notice">
        <AlertTriangle size={14} class="notice-icon" />
        <div class="notice-body">
          <strong>Reagendado automaticamente</strong> (tentativa {selectedPost.retry_count})
          {#if selectedPost.error_message}<span class="notice-detail">{selectedPost.error_message}</span>{/if}
        </div>
      </div>
    {/if}

    <!-- Dummy Engagement Metrics -->
    <div class="details-metrics-row">
      <div class="metric-cell">
        <span class="metric-label">Reações</span>
        <span class="metric-value metric-reactions {selectedPost.status === 'published' ? 'published-reactions' : ''}">
          {selectedPost.status === 'published' ? getPostEngagement(selectedPost.id, 'likes') : '0'}
        </span>
      </div>
      <div class="metric-cell">
        <span class="metric-label">Comentários</span>
        <span class="metric-value">
          {selectedPost.status === 'published' ? getPostEngagement(selectedPost.id, 'comments') : '0'}
        </span>
      </div>
      <div class="metric-cell">
        <span class="metric-label">Compartilhamentos</span>
        <span class="metric-value">
          {selectedPost.status === 'published' ? getPostEngagement(selectedPost.id, 'shares') : '0'}
        </span>
      </div>
    </div>

    <div class="details-preview-container">
      <PostPreview 
        content={selectedPost.content}
        imageUrl={selectedPost.image_url}
        title={selectedPost.title}
        imageSource={selectedPost.image_source}
        compact={true}
      />
    </div>
  {:else}
    <div class="details-empty-state">
      <TrendingDown size={32} class="empty-icon" />
      <h3>Nenhum post selecionado</h3>
      <p>Selecione um post da lista para visualizar a prévia ao vivo e os dados estatísticos.</p>
    </div>
  {/if}
</div>

<style>
  .details-panel-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 20px;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-card);
    max-height: calc(100vh - 210px);
    overflow-y: auto;
  }

  .details-panel-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
  }

  .details-header-text {
    flex: 1;
    min-width: 0;
  }

  .panel-eyebrow {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-dim);
    display: block;
    margin-bottom: 3px;
  }

  .details-panel-header h2 {
    font-size: 14.5px;
    font-weight: 600;
    color: var(--text);
    line-height: 1.35;
    word-break: break-word;
  }

  .details-linkedin-btn {
    flex-shrink: 0;
  }

  .details-linkedin-btn :global(.link-icon) {
    color: var(--text-muted);
  }

  /* Aviso de falha / retry */
  .failure-notice,
  .retry-notice {
    display: flex;
    align-items: flex-start;
    gap: 9px;
    margin-top: 14px;
    padding: 10px 12px;
    border-radius: var(--radius-lg);
    font-size: 12px;
    line-height: 1.4;
  }

  .failure-notice {
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.35);
    color: var(--text);
  }

  .failure-notice :global(.notice-icon) {
    color: #ef4444;
    flex-shrink: 0;
    margin-top: 1px;
  }

  .retry-notice {
    background: rgba(251, 191, 36, 0.08);
    border: 1px solid rgba(251, 191, 36, 0.35);
    color: var(--text);
  }

  .retry-notice :global(.notice-icon) {
    color: var(--amber);
    flex-shrink: 0;
    margin-top: 1px;
  }

  .notice-body {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .notice-detail {
    color: var(--text-muted);
    font-size: 11.5px;
    word-break: break-word;
  }

  /* Dummy Metrics */
  .details-metrics-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
    padding: 14px 0;
    border-bottom: 1px solid var(--border);
  }

  .metric-cell {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .metric-label {
    font-size: 11px;
    color: var(--text-dim);
  }

  .metric-value {
    font-family: var(--font-mono);
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text-muted);
  }

  .metric-reactions.published-reactions {
    color: var(--accent);
  }

  /* Live Preview */
  .details-preview-container {
    padding-top: 18px;
    flex: 1;
    min-height: 0;
  }

  /* Empty state */
  .details-empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 60px 20px;
  }

  .details-empty-state :global(.empty-icon) {
    color: var(--text-dim);
    margin-bottom: 12px;
  }

  .details-empty-state h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 6px;
  }

  .details-empty-state p {
    font-size: 12.5px;
    color: var(--text-muted);
    max-width: 250px;
    line-height: 1.45;
  }

  /* Shared buttons */
  .studio-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 7px 12px;
    border-radius: var(--radius-lg);
    font-family: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all var(--transition-fast);
    white-space: nowrap;
  }

  .studio-btn-secondary {
    background: var(--surface);
    border-color: var(--border);
    color: var(--text);
  }

  .studio-btn-secondary:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }
</style>
