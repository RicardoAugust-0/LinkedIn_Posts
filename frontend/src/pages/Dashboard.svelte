<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    Clock, Send, Trash2, ExternalLink, Sparkles, Filter, 
    ChevronDown, CheckCircle2, AlertCircle, TrendingUp, TrendingDown 
  } from '@lucide/svelte';
  import StatusBadge from '../components/StatusBadge.svelte';
  import PostPreview from '../components/PostPreview.svelte';

  export let theme: 'dark' | 'light' = 'dark';
  export let selectedPostId: string | null = null;

  // Dados do Dashboard
  let posts: any[] = [];
  let stats = {
    total_posts: 0,
    draft_posts: 0,
    scheduled_posts: 0,
    published_posts: 0
  };

  let loading = true;
  let selectedPost: any = null;
  let publishingId: string | null = null;
  
  // Feedback Toasts
  let showSuccessToast = false;
  let successToastMsg = "";
  let showErrorToast = false;
  let errorToastMsg = "";

  // Filtros & Ordenação
  let filterStatus = 'all';
  let sortBy: 'newest' | 'oldest' | 'title_asc' | 'title_desc' = 'newest';
  let showSortDropdown = false;
  let currentDate = '';

  // Selecionar post reactivamente se vier da busca global
  $: if (selectedPostId && posts.length > 0) {
    const found = posts.find(p => p.id === selectedPostId);
    if (found) {
      selectedPost = found;
    }
  }

  onMount(async () => {
    // Definir data atual no formato premium
    const options: Intl.DateTimeFormatOptions = {
      weekday: 'long',
      day: 'numeric',
      month: 'long',
      hour: '2-digit',
      minute: '2-digit'
    };
    currentDate = new Date().toLocaleString('pt-BR', options);

    await refreshData();
  });

  async function refreshData() {
    loading = true;
    try {
      // Carregar Posts
      const postsRes = await fetch('http://localhost:3000/api/posts');
      if (postsRes.ok) {
        posts = await postsRes.json();
        if (posts.length > 0) {
          // Selecionar o primeiro post por padrão para o preview
          if (selectedPostId) {
            selectedPost = posts.find(p => p.id === selectedPostId) || posts[0];
          } else if (!selectedPost) {
            selectedPost = posts[0];
          } else {
            // Atualizar o post selecionado com novos dados
            selectedPost = posts.find(p => p.id === selectedPost.id) || posts[0];
          }
        } else {
          selectedPost = null;
        }
      }

      // Carregar Estatísticas
      const statsRes = await fetch('http://localhost:3000/api/posts/stats');
      if (statsRes.ok) {
        stats = await statsRes.json();
      }
    } catch (e) {
      console.error("Erro ao carregar dados do dashboard", e);
    } finally {
      loading = false;
    }
  }

  async function deletePost(id: string, event: Event) {
    event.stopPropagation(); // Evitar abrir o post
    if (!confirm("Tem certeza que deseja excluir esta publicação?")) return;

    try {
      const res = await fetch(`http://localhost:3000/api/posts/${id}`, {
        method: 'DELETE'
      });

      if (res.ok) {
        if (selectedPost && selectedPost.id === id) {
          selectedPost = null;
        }
        triggerSuccessToast("Publicação excluída com sucesso.");
        await refreshData();
      }
    } catch (e) {
      console.error("Erro ao deletar post", e);
      triggerErrorToast("Falha ao excluir a publicação.");
    }
  }

  async function publishNow(id: string, event: Event) {
    event.stopPropagation();
    publishingId = id;

    try {
      const res = await fetch(`http://localhost:3000/api/posts/${id}/publish`, {
        method: 'POST'
      });

      const data = await res.json();
      if (res.ok && data.success) {
        triggerSuccessToast("Publicação efetuada com sucesso no LinkedIn!");
        await refreshData();
      } else {
        triggerErrorToast(data.message || "Falha na publicação. Verifique as credenciais.");
      }
    } catch (e) {
      triggerErrorToast("Erro de conexão ao servidor backend.");
      console.error(e);
    } finally {
      publishingId = null;
    }
  }

  function triggerSuccessToast(msg: string) {
    successToastMsg = msg;
    showSuccessToast = true;
    setTimeout(() => {
      showSuccessToast = false;
    }, 4000);
  }

  function triggerErrorToast(msg: string) {
    errorToastMsg = msg;
    showErrorToast = true;
    setTimeout(() => {
      showErrorToast = false;
    }, 4000);
  }

  // Filtragem e Ordenação dos posts
  $: sortedPosts = posts
    .filter(post => {
      if (filterStatus === 'all') return true;
      return post.status === filterStatus;
    })
    .sort((a, b) => {
      if (sortBy === 'newest') {
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      } else if (sortBy === 'oldest') {
        return new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
      } else if (sortBy === 'title_asc') {
        return a.title.localeCompare(b.title);
      } else if (sortBy === 'title_desc') {
        return b.title.localeCompare(a.title);
      }
      return 0;
    });

  function formatDateTime(isoString: string | null) {
    if (!isoString) return '—';
    const date = new Date(isoString);
    const dd = String(date.getDate()).padStart(2, '0');
    const mm = String(date.getMonth() + 1).padStart(2, '0');
    const hh = String(date.getHours()).padStart(2, '0');
    const mi = String(date.getMinutes()).padStart(2, '0');
    return `${dd}/${mm} ${hh}:${mi}`;
  }

  // Exportar relatório em formato CSV (UTF-8 com BOM para Excel)
  function exportReport() {
    if (posts.length === 0) {
      triggerErrorToast("Nenhuma publicação encontrada para exportar.");
      return;
    }

    let csvContent = "ID;Título;Tópico;Status;Criado Em;Agendado Para;Publicado Em;LinkedIn Post ID\n";

    posts.forEach(post => {
      const id = post.id;
      const title = `"${post.title.replace(/"/g, '""')}"`;
      const topic = `"${post.topic.replace(/"/g, '""')}"`;
      const status = post.status;
      const createdAt = post.created_at || '';
      const scheduledAt = post.scheduled_at || '';
      const publishedAt = post.published_at || '';
      const linkedinId = post.linkedin_post_id || '';

      csvContent += `${id};${title};${topic};${status};${createdAt};${scheduledAt};${publishedAt};${linkedinId}\n`;
    });

    const blob = new Blob([new Uint8Array([0xEF, 0xBB, 0xBF]), csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.setAttribute("href", url);
    link.setAttribute("download", "quill_posts_report.csv");
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    
    triggerSuccessToast("Relatório exportado com sucesso!");
  }

  // Obter métricas de engajamento simuladas determinísticas baseadas no ID do post
  function getPostEngagement(postId: string, metric: 'likes' | 'comments' | 'shares') {
    let hash = 0;
    for (let i = 0; i < postId.length; i++) {
      hash = postId.charCodeAt(i) + ((hash << 5) - hash);
    }
    hash = Math.abs(hash);

    if (metric === 'likes') {
      const val = (hash % 850) + 120; // 120 a 970
      if (val > 1000) return `${(val / 1000).toFixed(1)}k`;
      return String(val);
    } else if (metric === 'comments') {
      return String((hash % 85) + 12); // 12 a 97
    } else {
      return String((hash % 35) + 3); // 3 a 38
    }
  }

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();
  
  function triggerCreate() {
    dispatch('navigate', 'create');
  }
</script>

<div class="studio-page-header">
  <div class="studio-page-header-info">
    <div class="studio-eyebrow">{currentDate}</div>
    <h1>
      Olá, Ricardo Augusto <span class="header-divider">—</span> <span class="header-count">{stats.published_posts} post{stats.published_posts !== 1 ? 's' : ''} este mês</span>
    </h1>
  </div>
  
  <div class="header-actions">
    <button class="studio-btn studio-btn-secondary" on:click={exportReport}>
      Exportar relatório
    </button>
    <button class="studio-btn studio-btn-accent" on:click={triggerCreate}>
      <Sparkles size={12} />
      <span>Criar com IA</span>
    </button>
  </div>
</div>

<!-- Toast Notifications -->
<div class="studio-toast-container">
  {#if showSuccessToast}
    <div class="studio-toast studio-toast-success">
      <CheckCircle2 size={16} />
      <span>{successToastMsg}</span>
    </div>
  {/if}
  {#if showErrorToast}
    <div class="studio-toast studio-toast-error">
      <AlertCircle size={16} />
      <span>{errorToastMsg}</span>
    </div>
  {/if}
</div>

<div class="studio-page-body">
  <!-- Stat Cards -->
  <div class="studio-stat-grid">
    <div class="studio-stat-card">
      <div class="studio-stat-header">
        <span class="studio-stat-label">Total de posts</span>
        <div class="studio-stat-trend trend-up">
          <TrendingUp size={11} />
          <span>+33%</span>
        </div>
      </div>
      <div class="studio-stat-value">{stats.total_posts}</div>
      <div class="studio-stat-sub">Desde a criação da conta</div>
    </div>

    <div class="studio-stat-card">
      <div class="studio-stat-header">
        <span class="studio-stat-label">Rascunhos</span>
        <span class="studio-stat-trend trend-neutral">pendente</span>
      </div>
      <div class="studio-stat-value">{stats.draft_posts}</div>
      <div class="studio-stat-sub">Aguardando edição ou envio</div>
    </div>

    <div class="studio-stat-card">
      <div class="studio-stat-header">
        <span class="studio-stat-label">Agendados</span>
        <span class="studio-stat-trend trend-active">ativo</span>
      </div>
      <div class="studio-stat-value">{stats.scheduled_posts}</div>
      <div class="studio-stat-sub">Próximo post programado</div>
    </div>

    <div class="studio-stat-card">
      <div class="studio-stat-header">
        <span class="studio-stat-label">Publicados</span>
        <div class="studio-stat-trend trend-up">
          <TrendingUp size={11} />
          <span>+12%</span>
        </div>
      </div>
      <div class="studio-stat-value">{stats.published_posts}</div>
      <div class="studio-stat-sub">Sucesso no feed do LinkedIn</div>
    </div>
  </div>

  <!-- Toolbar Filters -->
  <div class="studio-toolbar">
    <div class="segmented-control">
      <button 
        class="segment-btn {filterStatus === 'all' ? 'active' : ''}" 
        on:click={() => filterStatus = 'all'}
      >
        Todos <span class="segment-count">{posts.length}</span>
      </button>
      <button 
        class="segment-btn {filterStatus === 'draft' ? 'active' : ''}" 
        on:click={() => filterStatus = 'draft'}
      >
        Rascunhos <span class="segment-count">{posts.filter(p => p.status === 'draft').length}</span>
      </button>
      <button 
        class="segment-btn {filterStatus === 'scheduled' ? 'active' : ''}" 
        on:click={() => filterStatus = 'scheduled'}
      >
        Agendados <span class="segment-count">{posts.filter(p => p.status === 'scheduled').length}</span>
      </button>
      <button 
        class="segment-btn {filterStatus === 'published' ? 'active' : ''}" 
        on:click={() => filterStatus = 'published'}
      >
        Publicados <span class="segment-count">{posts.filter(p => p.status === 'published').length}</span>
      </button>
      <button 
        class="segment-btn {filterStatus === 'failed' ? 'active' : ''}" 
        on:click={() => filterStatus = 'failed'}
      >
        Falhas <span class="segment-count">{posts.filter(p => p.status === 'failed').length}</span>
      </button>
    </div>

    <div class="sort-btn-wrapper" style="position: relative;">
      <button class="studio-btn studio-btn-secondary sort-btn" on:click={() => showSortDropdown = !showSortDropdown}>
        <Filter size={13} />
        <span>
          {#if sortBy === 'newest'}
            Ordenar: mais recente
          {:else if sortBy === 'oldest'}
            Ordenar: mais antigo
          {:else if sortBy === 'title_asc'}
            Ordenar: A-Z
          {:else if sortBy === 'title_desc'}
            Ordenar: Z-A
          {/if}
        </span>
        <ChevronDown size={11} />
      </button>
      
      {#if showSortDropdown}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div class="sort-dropdown" on:click={() => showSortDropdown = false}>
          <button class="sort-option-btn {sortBy === 'newest' ? 'active' : ''}" on:click={() => sortBy = 'newest'}>Mais recente</button>
          <button class="sort-option-btn {sortBy === 'oldest' ? 'active' : ''}" on:click={() => sortBy = 'oldest'}>Mais antigo</button>
          <button class="sort-option-btn {sortBy === 'title_asc' ? 'active' : ''}" on:click={() => sortBy = 'title_asc'}>Título (A-Z)</button>
          <button class="sort-option-btn {sortBy === 'title_desc' ? 'active' : ''}" on:click={() => sortBy = 'title_desc'}>Título (Z-A)</button>
        </div>
      {/if}
    </div>
  </div>

  <!-- Main Workspace Workspace -->
  {#if loading && posts.length === 0}
    <div class="loader-container">
      <div class="spinner"></div>
      <p class="loader-text">Buscando publicações no banco local...</p>
    </div>
  {:else}
    <div class="studio-workspace-grid">
      <!-- Left Column: Posts Scroll List -->
      <div class="posts-scroll-list">
        {#if sortedPosts.length === 0}
          <div class="empty-list-card">
            <Clock size={24} class="empty-icon" />
            <h3>Nenhuma publicação encontrada</h3>
            <p>Clique no botão 'Criar com IA' no topo direito para gerar sua primeira postagem.</p>
            <button class="studio-btn studio-btn-secondary" on:click={triggerCreate}>Criar Novo Post</button>
          </div>
        {:else}
          {#each sortedPosts as post}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div 
              class="post-list-item-card {selectedPost && selectedPost.id === post.id ? 'selected' : ''}"
              on:click={() => selectedPost = post}
            >
              {#if selectedPost && selectedPost.id === post.id}
                <div class="selection-marker"></div>
              {/if}

              <!-- Mini Thumbnail -->
              {#if post.image_url && post.image_source !== 'none'}
                <div class="post-card-thumbnail">
                  <img 
                    src={post.image_url.startsWith('/uploads') ? `http://localhost:3000${post.image_url}` : post.image_url} 
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
                    <Clock size={11} />
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
                      <Sparkles size={10} />
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
                        on:click={(e) => publishNow(post.id, e)}
                      >
                        {#if publishingId === post.id}
                          <span class="mini-spinner"></span>
                        {:else}
                          <Send size={11} />
                        {/if}
                      </button>
                    {/if}
                    <button class="meta-btn delete" title="Excluir" on:click={(e) => deletePost(post.id, e)}>
                      <Trash2 size={11} />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Right Column: Details & Feed Mockup -->
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
                <ExternalLink size={11} />
              </a>
            {/if}
          </div>

          <!-- Dummy Engagement Metrics -->
          <div class="details-metrics-row">
            <div class="metric-cell">
              <span class="metric-label">Reações</span>
              <span class="metric-value {theme === 'light' ? 'light-accent' : 'dark-accent'}">
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
    </div>
  {/if}
</div>

<style>
  .header-actions {
    display: flex;
    gap: 8px;
  }

  .header-divider {
    color: var(--text-dim);
    font-weight: 400;
  }

  .header-count {
    color: var(--text-muted);
    font-weight: 400;
  }

  .trend-neutral {
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .trend-active {
    color: var(--cyan);
    font-family: var(--font-mono);
    font-size: 11px;
  }

  /* Toolbar */
  .studio-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .segmented-control {
    display: flex;
    padding: 4px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 10px;
  }

  .segment-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 12px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 12.5px;
    font-weight: 400;
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .segment-btn:hover {
    color: var(--text);
  }

  .segment-btn.active {
    background: var(--surface);
    color: var(--text);
    font-weight: 500;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.06);
  }

  .segment-count {
    font-family: var(--font-mono);
    font-size: 10.5px;
    color: var(--text-dim);
    transition: color var(--transition-fast);
  }

  .segment-btn.active .segment-count {
    color: var(--text-muted);
  }

  .sort-btn {
    font-size: 12.5px;
    color: var(--text-muted);
    padding: 7px 12px;
  }

  .sort-dropdown {
    position: absolute;
    top: 105%;
    right: 0;
    background: var(--surface);
    border: 1px solid var(--border-strong);
    border-radius: 8px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.25);
    z-index: 10;
    min-width: 140px;
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sort-option-btn {
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    padding: 6px 10px;
    font-family: inherit;
    font-size: 12px;
    color: var(--text-muted);
    border-radius: 4px;
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .sort-option-btn:hover {
    background-color: var(--surface-alt);
    color: var(--text);
  }

  .sort-option-btn.active {
    background-color: var(--surface-hover);
    color: var(--accent);
    font-weight: 500;
  }
  
  :global(.theme-light) .sort-option-btn.active {
    color: var(--text);
  }

  /* Workspace Grid */
  .studio-workspace-grid {
    display: grid;
    grid-template-columns: 1.2fr 1fr;
    gap: 20px;
    min-height: 0;
    flex: 1;
  }

  @media (max-width: 1024px) {
    .studio-workspace-grid {
      grid-template-columns: 1fr;
    }
  }

  /* Posts Scroll List */
  .posts-scroll-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow-y: auto;
    max-height: calc(100vh - 310px);
    padding-right: 4px;
  }

  .post-list-item-card {
    display: flex;
    gap: 14px;
    padding: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 12px;
    position: relative;
    cursor: pointer;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .post-list-item-card:hover {
    border-color: var(--border-strong);
  }

  .post-list-item-card.selected {
    border-color: var(--border-strong);
    box-shadow: 0 0 0 3px rgba(163,230,53,0.06);
  }

  :global(.theme-light) .post-list-item-card.selected {
    box-shadow: 0 0 0 3px rgba(26,26,26,0.04);
  }

  .selection-marker {
    position: absolute;
    left: -1px;
    top: 14px;
    bottom: 14px;
    width: 3px;
    background: var(--text);
    border-radius: 3px;
  }

  :global(.theme-dark) .selection-marker {
    background: var(--accent);
  }

  /* Miniature Thumbnail */
  .post-card-thumbnail {
    width: 96px;
    height: 96px;
    border-radius: 8px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid var(--border);
  }

  .post-card-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .empty-thumbnail {
    background: var(--surface-alt);
    border: 1px dashed var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--text-dim);
    letter-spacing: 0.1em;
    text-transform: uppercase;
  }

  .post-card-details {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .post-card-row-one {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 10px;
    margin-bottom: 4px;
  }

  .post-card-title {
    font-size: 14.5px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: -0.005em;
    line-height: 1.25;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .post-card-snippet {
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.45;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .post-card-meta-row {
    display: flex;
    align-items: center;
    gap: 14px;
    font-size: 11.5px;
    color: var(--text-dim);
    font-family: var(--font-mono);
    margin-top: 8px;
  }

  .meta-time {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .meta-ai-badge {
    display: flex;
    align-items: center;
    gap: 4px;
    color: var(--accent);
  }

  :global(.theme-light) .meta-ai-badge {
    color: var(--text-muted);
  }

  /* Actions inside Card */
  .meta-actions {
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity var(--transition-fast);
  }

  .post-list-item-card:hover .meta-actions {
    opacity: 1;
  }

  .meta-btn {
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 6px;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .meta-btn:hover {
    color: var(--text);
    border-color: var(--border-strong);
  }

  .meta-btn.publish:hover {
    color: var(--accent);
    border-color: rgba(163, 230, 53, 0.3);
    background: var(--accent-muted);
  }

  .meta-btn.delete:hover {
    color: var(--rose);
    border-color: rgba(251, 113, 133, 0.3);
    background: var(--rose-muted);
  }

  .mini-spinner {
    width: 10px;
    height: 10px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: rotate 1s linear infinite;
  }

  @keyframes rotate {
    to { transform: rotate(360deg); }
  }

  /* Right Details Column */
  .details-panel-card {
    border: 1px solid var(--border);
    border-radius: 14px;
    background: var(--surface);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    max-height: calc(100vh - 310px);
  }

  .details-panel-header {
    padding: 16px 20px 14px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .details-header-text {
    flex: 1;
    min-width: 0;
  }

  .panel-eyebrow {
    font-family: var(--font-mono);
    font-size: 10.5px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--text-dim);
    margin-bottom: 4px;
    display: block;
  }

  .details-panel-header h2 {
    font-size: 15px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .details-linkedin-btn {
    font-size: 12px;
    padding: 6px 11px;
    font-weight: 500;
  }

  /* Dummy Metrics */
  .details-metrics-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    padding: 12px 20px 14px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-alt);
  }

  .metric-cell {
    display: flex;
    flex-direction: column;
  }

  .metric-label {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 500;
    margin-bottom: 6px;
  }

  .metric-value {
    font-size: 22px;
    font-weight: 500;
    letter-spacing: -0.02em;
    color: var(--text);
  }

  .metric-value.dark-accent {
    color: var(--accent);
  }
  .metric-value.light-accent {
    color: #1a1a1a;
  }

  .details-preview-container {
    padding: 18px;
    background: var(--bg-inset);
    flex: 1;
    overflow-y: auto;
  }

  /* Empty States & Loaders */
  .empty-state-card {
    padding: 40px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    border: 1px dashed var(--border);
  }

  .empty-icon {
    color: var(--text-dim);
  }

  .details-empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    gap: 12px;
    padding: 30px;
    color: var(--text-muted);
  }

  .details-empty-state p {
    font-size: 13px;
    max-width: 280px;
  }

  .loader-container {
    padding: 80px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    color: var(--text-muted);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border);
    border-right-color: var(--text);
    border-radius: 50%;
    animation: rotate 1s linear infinite;
  }

  .loader-text {
    font-size: 13.5px;
  }
</style>
