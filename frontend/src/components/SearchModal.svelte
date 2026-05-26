<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { Search, Calendar, Sparkles, X, CornerDownLeft } from '@lucide/svelte';
  import StatusBadge from './StatusBadge.svelte';

  export let show = false;

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let searchQuery = '';
  let posts: any[] = [];
  let loading = false;
  let activeIndex = 0;
  let searchInput: HTMLInputElement;

  $: if (show) {
    loadPosts();
    searchQuery = '';
    activeIndex = 0;
    focusInput();
  }

  async function focusInput() {
    await tick();
    if (searchInput) {
      searchInput.focus();
    }
  }

  async function loadPosts() {
    loading = true;
    try {
      const res = await fetch('http://localhost:3000/api/posts');
      if (res.ok) {
        posts = await res.json();
      }
    } catch (e) {
      console.error("Erro ao carregar posts na busca", e);
    } finally {
      loading = false;
    }
  }

  $: filteredPosts = posts.filter(post => 
    post.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    post.topic.toLowerCase().includes(searchQuery.toLowerCase()) ||
    post.content.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // Reiniciar activeIndex quando a query mudar para não estourar os limites
  $: if (searchQuery) {
    activeIndex = 0;
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown') {
      event.preventDefault();
      activeIndex = (activeIndex + 1) % Math.max(1, filteredPosts.length);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      activeIndex = (activeIndex - 1 + filteredPosts.length) % Math.max(1, filteredPosts.length);
    } else if (event.key === 'Enter') {
      event.preventDefault();
      if (filteredPosts[activeIndex]) {
        selectPost(filteredPosts[activeIndex]);
      }
    } else if (event.key === 'Escape') {
      event.preventDefault();
      close();
    }
  }

  function selectPost(post: any) {
    dispatch('select', post.id);
    close();
  }

  function close() {
    dispatch('close');
  }

  function formatDateTime(isoString: string | null) {
    if (!isoString) return '—';
    const date = new Date(isoString);
    const dd = String(date.getDate()).padStart(2, '0');
    const mm = String(date.getMonth() + 1).padStart(2, '0');
    return `${dd}/${mm}`;
  }
</script>

{#if show}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="search-backdrop" on:click={close}>
    <div class="search-modal" on:click|stopPropagation on:keydown={handleKeyDown}>
      <div class="search-input-wrapper">
        <Search size={16} class="search-input-icon" />
        <input 
          type="text" 
          placeholder="Buscar publicações..." 
          bind:value={searchQuery}
          bind:this={searchInput}
          class="search-input"
          autocomplete="off"
        />
        <kbd class="esc-kbd">ESC</kbd>
        <button class="close-btn" on:click={close} title="Fechar busca">
          <X size={15} />
        </button>
      </div>

      <div class="search-results-container">
        {#if loading}
          <div class="search-loading">
            <div class="spinner"></div>
            <span>Buscando publicações...</span>
          </div>
        {:else if filteredPosts.length === 0}
          <div class="search-empty">
            <Search size={22} class="empty-icon" />
            <h3>Nenhum resultado encontrado</h3>
            <p>Tente refinar sua busca por palavras-chave diferentes.</p>
          </div>
        {:else}
          <div class="results-list">
            <div class="results-header">Publicações ({filteredPosts.length})</div>
            {#each filteredPosts as post, idx}
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <!-- svelte-ignore a11y-no-static-element-interactions -->
              <div 
                class="result-item {idx === activeIndex ? 'active' : ''}"
                on:click={() => selectPost(post)}
                on:mouseenter={() => activeIndex = idx}
              >
                <div class="result-badge">
                  <StatusBadge status={post.status} />
                </div>
                <div class="result-info">
                  <div class="result-title">{post.title}</div>
                  <div class="result-meta">
                    <span class="result-topic">{post.topic}</span>
                    <span class="meta-dot">•</span>
                    <span class="result-date">
                      <Calendar size={10} />
                      {formatDateTime(post.created_at)}
                    </span>
                    {#if post.image_source === 'ai'}
                      <span class="meta-dot">•</span>
                      <span class="result-ai">
                        <Sparkles size={9} />
                        IA
                      </span>
                    {/if}
                  </div>
                </div>
                {#if idx === activeIndex}
                  <div class="enter-badge">
                    <span>ir para</span>
                    <CornerDownLeft size={10} />
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
      
      <div class="search-footer">
        <div class="help-item">
          <span class="key-indicator">↑↓</span>
          <span>Navegar</span>
        </div>
        <div class="help-item">
          <span class="key-indicator">Enter</span>
          <span>Selecionar</span>
        </div>
        <div class="help-item">
          <span class="key-indicator">Esc</span>
          <span>Fechar</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .search-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(10, 10, 11, 0.75);
    backdrop-filter: blur(12px);
    z-index: 1000;
    display: flex;
    align-items: start;
    justify-content: center;
    padding-top: 12vh;
  }

  :global(.theme-light) .search-backdrop {
    background: rgba(240, 239, 234, 0.75);
  }

  .search-modal {
    width: 600px;
    max-width: 90vw;
    background: var(--surface);
    border: 1px solid var(--border-strong);
    border-radius: 12px;
    box-shadow: var(--shadow-popover);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: scaleUp 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes scaleUp {
    from {
      transform: scale(0.97);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }

  .search-input-wrapper {
    display: flex;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    gap: 12px;
  }

  :global(.search-input-icon) {
    color: var(--text-muted);
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    outline: none;
    font-size: 15px;
    color: var(--text);
    font-family: var(--font-main);
  }

  .search-input::placeholder {
    color: var(--text-dim);
  }

  .esc-kbd {
    font-family: var(--font-mono);
    font-size: 9px;
    background: var(--surface-alt);
    border: 1px solid var(--border-strong);
    padding: 2px 5px;
    border-radius: 4px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 4px;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .close-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .search-results-container {
    max-height: 360px;
    overflow-y: auto;
    padding: 8px;
    min-height: 120px;
    background-color: var(--bg-inset);
  }

  .search-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    height: 160px;
    color: var(--text-muted);
    font-size: 13.5px;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid var(--border-strong);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .search-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 32px 20px;
    height: 180px;
  }

  :global(.empty-icon) {
    color: var(--text-dim);
    margin-bottom: 12px;
  }

  .search-empty h3 {
    font-size: 14.5px;
    margin-bottom: 6px;
    font-weight: 600;
  }

  .search-empty p {
    font-size: 12.5px;
    color: var(--text-muted);
    max-width: 280px;
  }

  .results-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .results-header {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 6px 12px 4px;
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 14px;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color var(--transition-fast);
  }

  .result-item.active {
    background-color: var(--surface-hover);
  }

  .result-badge {
    flex-shrink: 0;
  }

  .result-info {
    flex: 1;
    min-width: 0;
  }

  .result-title {
    font-size: 13.5px;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.3;
  }

  .result-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 3px;
    font-size: 11px;
    color: var(--text-muted);
  }

  .result-topic {
    font-family: var(--font-mono);
    font-size: 10.5px;
    color: var(--text-dim);
  }

  .meta-dot {
    color: var(--border-strong);
  }

  .result-date {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .result-ai {
    display: flex;
    align-items: center;
    gap: 3px;
    color: var(--cyan);
    font-family: var(--font-mono);
    font-size: 9.5px;
  }

  .enter-badge {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--text-dim);
    animation: fadeIn 0.1s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .search-footer {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 20px;
    border-top: 1px solid var(--border);
    background-color: var(--surface);
  }

  .help-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text-muted);
  }

  .key-indicator {
    font-family: var(--font-mono);
    background: var(--surface-alt);
    border: 1px solid var(--border-strong);
    padding: 1px 4px;
    border-radius: 3px;
    font-size: 9.5px;
    color: var(--text-dim);
    font-weight: 500;
  }
</style>
