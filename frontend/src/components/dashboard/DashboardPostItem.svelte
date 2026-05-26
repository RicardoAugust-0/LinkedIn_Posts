<!-- frontend/src/components/dashboard/DashboardPostItem.svelte -->
<script lang="ts">
  import { Clock, Sparkles, Send, Trash2 } from '@lucide/svelte';
  import StatusBadge from '../StatusBadge.svelte';
  import { createEventDispatcher } from 'svelte';
  import { API_URL } from '../../lib/api';
  const dispatch = createEventDispatcher();

  export let post: any;
  export let isSelected = false;
  export let publishingId: string | null = null;
  export let formatDateTime: (isoString: string | null) => string;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div 
  class="post-list-item-card {isSelected ? 'selected' : ''}"
  on:click={() => dispatch('select', post)}
>
  {#if isSelected}
    <div class="selection-marker"></div>
  {/if}

  <!-- Mini Thumbnail -->
  {#if post.image_url && post.image_source !== 'none'}
    <div class="post-card-thumbnail">
       <img 
        src={post.image_url.startsWith('/uploads') ? `${API_URL}${post.image_url}` : post.image_url} 
        alt={post.title} 
      />
    </div>
  {:else}
    <div class="post-card-thumbnail empty-thumbnail">
      <span>sem mídia</span>
    </div>
  {/if}

  <!-- Details Info -->
  <div class="post-card-details">
    <div class="post-card-row-one">
      <span class="post-card-title">{post.title}</span>
      <StatusBadge status={post.status} />
    </div>
    <p class="post-card-snippet">{post.content.substring(0, 110)}...</p>
    <div class="post-card-meta-row">
      <span class="meta-time">
        <Clock size={11} class="clock-icon" />
        {#if post.status === 'scheduled'}
          {formatDateTime(post.scheduled_at)}
        {:else if post.status === 'published'}
          {formatDateTime(post.published_at)}
        {:else}
          {formatDateTime(post.created_at)}
        {/if}
      </span>

      {#if post.image_source === 'ai'}
        <span class="meta-ai-badge">
          <Sparkles size={10} class="spark-icon" />
          gerado por IA
        </span>
      {/if}

      <div class="flex-spacer"></div>

      <div class="meta-actions">
        {#if post.status === 'draft' || post.status === 'failed'}
          <button 
            class="meta-btn publish" 
            title="Publicar agora" 
            disabled={publishingId === post.id}
            on:click|stopPropagation={(e) => dispatch('publish', { id: post.id, event: e })}
          >
            {#if publishingId === post.id}
              <span class="mini-spinner"></span>
            {:else}
              <Send size={11} />
            {/if}
          </button>
        {/if}
        <button class="meta-btn delete" title="Excluir" on:click|stopPropagation={(e) => dispatch('delete', { id: post.id, event: e })}>
          <Trash2 size={11} />
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .post-list-item-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 12px 14px;
    display: flex;
    gap: 14px;
    cursor: pointer;
    position: relative;
    box-shadow: var(--shadow-sm);
    transition: all var(--transition-fast);
  }

  .post-list-item-card:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .post-list-item-card.selected {
    background: var(--surface-alt);
    border-color: var(--border-strong);
    box-shadow: inset 0 0 0 1px var(--border-strong);
  }

  .selection-marker {
    position: absolute;
    top: 14px;
    left: 0;
    width: 2px;
    height: 18px;
    background: var(--accent);
    border-radius: 0 4px 4px 0;
  }

  /* Thumbnail */
  .post-card-thumbnail {
    width: 68px;
    height: 68px;
    border-radius: 6px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid var(--border);
    background: var(--bg-inset);
  }

  .post-card-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .empty-thumbnail {
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    font-size: 10px;
    color: var(--text-dim);
    font-family: var(--font-mono);
    text-transform: uppercase;
  }

  /* Card details */
  .post-card-details {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 4px;
  }

  .post-card-row-one {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 10px;
  }

  .post-card-title {
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .post-card-snippet {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.4;
    word-break: break-all;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .post-card-meta-row {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 2px;
  }

  .meta-time {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: var(--text-dim);
    font-family: var(--font-mono);
  }

  .meta-time :global(.clock-icon) {
    color: var(--text-dim);
  }

  .meta-ai-badge {
    display: inline-flex;
    align-items: center;
    gap: 3.5px;
    font-size: 10px;
    font-family: var(--font-mono);
    color: var(--cyan);
    background: var(--cyan-muted);
    padding: 1px 5px;
    border-radius: 4px;
    font-weight: 500;
    text-transform: uppercase;
  }

  .meta-ai-badge :global(.spark-icon) {
    color: var(--cyan);
  }

  .flex-spacer {
    flex: 1;
  }

  /* Actions inside meta */
  .meta-actions {
    display: flex;
    gap: 4px;
  }

  .meta-btn {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: none;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .meta-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .meta-btn.publish:hover {
    background-color: var(--accent-muted);
    color: var(--accent);
  }

  .meta-btn.delete:hover {
    background-color: var(--rose-muted);
    color: var(--rose);
  }

  /* Mini spinner */
  .mini-spinner {
    width: 10px;
    height: 10px;
    border: 1.5px solid var(--border-strong);
    border-top-color: var(--text);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
