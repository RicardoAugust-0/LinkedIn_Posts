<!-- frontend/src/components/automation/AutomationQueueItem.svelte -->
<script lang="ts">
  import { 
    Clock, Pencil, RefreshCw, SkipForward, ChevronUp, ChevronDown, 
    Image as ImageIcon, Trash2 
  } from '@lucide/svelte';
  import StatusBadge from '../StatusBadge.svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let post: any;
  export let i: number;
  export let expanded = false;
  export let topicSeed = '';
  export let tone = '';
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div 
  class="queue-row {post.status === 'published' ? 'published' : ''} {expanded ? 'expanded' : ''}"
  on:click={() => dispatch('expand', post.id)}
>
  <!-- Connector dot & lines -->
  <div class="connector-cell">
    <div class="connector-line-top {i === 0 ? 'first' : ''} {post.status === 'published' ? 'published' : ''}"></div>
    <div class="connector-dot {post.status === 'published' ? 'published' : ''} {post.status === 'scheduled' ? 'scheduled' : ''}">
      {i + 1}
    </div>
    <div class="connector-line-bottom {post.status === 'published' ? 'published' : ''}"></div>
  </div>

  <div class="queue-title-cell">
    <span class="queue-title">{post.title}</span>
    <span class="queue-subtitle">ângulo {String(i + 1).padStart(2, '0')} · {post.imageSource === 'ai' ? 'imagem por IA' : 'imagem de busca'}</span>
  </div>

  <div class="queue-meta">
    <Clock size={11} class="clock-icon" />
    <span>{post.scheduled}</span>
  </div>

  <div class="queue-badge-cell">
    <StatusBadge status={post.status} />
  </div>

  <!-- Actions hovering/expanded -->
  <div class="actions-cell">
    <button class="icon-btn" title="Editar" on:click|stopPropagation={(e) => dispatch('edit', { id: post.id, event: e })}>
      <Pencil size={11} />
    </button>
    <button class="icon-btn" title="Regenerar" on:click|stopPropagation={(e) => dispatch('regenerate', { id: post.id, event: e })}>
      <RefreshCw size={11} />
    </button>
    <button class="icon-btn" title="Pular" on:click|stopPropagation={(e) => dispatch('skip', { id: post.id, event: e })}>
      <SkipForward size={11} />
    </button>
    <button class="icon-btn" title={expanded ? 'Recolher' : 'Expandir'} on:click|stopPropagation={() => dispatch('expand', post.id)}>
      {#if expanded}
        <ChevronUp size={11} />
      {:else}
        <ChevronDown size={11} />
      {/if}
    </button>
  </div>

  {#if expanded}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="expansion" on:click|stopPropagation>
      <div class="expansion-left">
        <span class="expand-label">Prompt da IA</span>
        <div class="ai-prompt-card">
          tema: {topicSeed.split('.')[0]}.{"\n"}
          ângulo {String(i + 1).padStart(2, '0')}: {post.title.toLowerCase()}{"\n"}
          tom: {tone ? tone.split('.')[0].toLowerCase() : 'padrão pragmático'}
        </div>
        <span class="expand-label" style="margin-top: 14px;">Conteúdo gerado</span>
        <div class="expand-body">
          {post.snippet}{"\n\n"}— escrito por Gemini 2.5 Pro · 280 tokens · score de variação 0.82
        </div>
      </div>
      <div class="expand-actions">
        <button class="expand-action-btn" on:click={() => dispatch('edit', { id: post.id, event: new MouseEvent('click') })}>
          <Pencil size={12} />
          <span>Editar manualmente</span>
        </button>
        <button class="expand-action-btn" on:click={() => dispatch('regenerate', { id: post.id, event: new MouseEvent('click') })}>
          <RefreshCw size={12} />
          <span>Regenerar texto</span>
        </button>
        <button class="expand-action-btn" on:click={() => dispatch('changeImage', post.id)}>
          <ImageIcon size={12} />
          <span>Trocar imagem</span>
        </button>
        <button class="expand-action-btn" on:click={() => dispatch('reschedule', post.id)}>
          <Clock size={12} />
          <span>Reagendar</span>
        </button>
        <button class="expand-action-btn delete-btn" on:click={() => dispatch('remove', post.id)}>
          <Trash2 size={12} />
          <span>Remover</span>
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .queue-row {
    display: grid;
    grid-template-columns: 60px 1fr 140px 100px 120px;
    align-items: center;
    padding: 10px 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    position: relative;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  @media (max-width: 768px) {
    .queue-row {
      grid-template-columns: 60px 1fr;
      row-gap: 8px;
    }
    .queue-meta, .queue-badge-cell, .actions-cell {
      grid-column: 2;
    }
  }

  .queue-row:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .queue-row.expanded {
    background: var(--surface-alt);
    border-color: var(--border-strong);
  }

  /* Connector Cell styles */
  .connector-cell {
    display: flex;
    flex-direction: column;
    align-items: center;
    align-self: stretch;
    position: relative;
  }

  .connector-line-top, .connector-line-bottom {
    width: 2px;
    background: var(--border);
    flex: 1;
  }

  .connector-line-top.first {
    background: transparent;
  }

  .connector-line-top.published, .connector-line-bottom.published {
    background: var(--accent);
  }

  .connector-dot {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--bg-inset);
    border: 2.5px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-mono);
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    z-index: 10;
    margin: 4px 0;
  }

  .connector-dot.published {
    border-color: var(--accent);
    color: var(--accent);
  }

  .connector-dot.scheduled {
    border-color: var(--cyan);
    color: var(--cyan);
  }

  /* Title and meta */
  .queue-title-cell {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
  }

  .queue-title {
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .queue-subtitle {
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 1px;
  }

  .queue-meta {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 12px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .queue-meta :global(.clock-icon) {
    color: var(--text-dim);
  }

  .queue-badge-cell {
    display: flex;
    align-items: center;
  }

  /* Actions Hovering */
  .actions-cell {
    display: flex;
    align-items: center;
    gap: 4px;
    justify-content: flex-end;
  }

  .icon-btn {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-fast);
  }

  .icon-btn:hover {
    background: var(--surface-hover);
    color: var(--text);
  }

  /* Expanded Section */
  .expansion {
    grid-column: 1 / -1;
    display: grid;
    grid-template-columns: 1fr 200px;
    gap: 20px;
    padding: 16px 0 6px;
    border-top: 1px solid var(--border);
    margin-top: 10px;
    cursor: default;
    animation: slideDown 0.15s ease-out;
  }

  @media (max-width: 768px) {
    .expansion {
      grid-template-columns: 1fr;
    }
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .expansion-left {
    min-width: 0;
  }

  .expand-label {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-dim);
    display: block;
    margin-bottom: 6px;
  }

  .ai-prompt-card {
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px 12px;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--cyan);
    white-space: pre-line;
    line-height: 1.5;
  }

  .expand-body {
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 10px 12px;
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.5;
    white-space: pre-line;
  }

  .expand-actions {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .expand-action-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    font-family: inherit;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
  }

  .expand-action-btn:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .expand-action-btn.delete-btn {
    border-color: rgba(251, 113, 133, 0.15);
    color: var(--rose);
  }

  .expand-action-btn.delete-btn:hover {
    background: rgba(251, 113, 133, 0.04);
    border-color: var(--rose);
  }
</style>
