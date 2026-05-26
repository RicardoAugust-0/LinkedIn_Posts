<!-- frontend/src/pages/Dashboard.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    Clock, Sparkles, CheckCircle2, AlertCircle
  } from '@lucide/svelte';
  import { API_URL } from '../lib/api';
  import DashboardStats from '../components/dashboard/DashboardStats.svelte';
  import DashboardToolbar from '../components/dashboard/DashboardToolbar.svelte';
  import DashboardPostItem from '../components/dashboard/DashboardPostItem.svelte';
  import DashboardPostDetails from '../components/dashboard/DashboardPostDetails.svelte';

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

  // Filtros ativos
  let filterStatus = 'all';
  let sortBy = 'newest';

  // Outras variáveis de UI
  let currentDate = '26 de maio, terça-feira';

  onMount(async () => {
    // Formatar data local
    const options: Intl.DateTimeFormatOptions = { day: 'numeric', month: 'long', weekday: 'long' };
    currentDate = new Date().toLocaleDateString('pt-BR', options);

    await refreshData();
    
    // Selecionar post se ID vier via props (ex: busca)
    if (selectedPostId) {
      const target = posts.find(p => p.id === selectedPostId);
      if (target) {
        selectedPost = target;
      }
    }
  });

  async function refreshData() {
    loading = true;
    try {
      await Promise.all([
        loadDashboardStats(),
        loadPosts()
      ]);
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function loadDashboardStats() {
    try {
      const res = await fetch(`${API_URL}/api/posts/stats`);
      if (res.ok) {
        stats = await res.json();
      }
    } catch (e) {
      console.error("Erro ao carregar estatísticas do dashboard", e);
    }
  }

  async function loadPosts() {
    try {
      const res = await fetch(`${API_URL}/api/posts`);
      if (res.ok) {
        posts = await res.json();
        // Manter o post selecionado atualizado após recarregar
        if (selectedPost) {
          const updated = posts.find(p => p.id === selectedPost.id);
          selectedPost = updated || null;
        }
      }
    } catch (e) {
      console.error("Erro ao carregar posts", e);
    }
  }

  async function deletePost(id: string, event: Event) {
    event.stopPropagation();
    if (!confirm("Deseja realmente excluir esta publicação do banco de dados local?")) return;

    try {
      const res = await fetch(`${API_URL}/api/posts/${id}`, {
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
      const res = await fetch(`${API_URL}/api/posts/${id}/publish`, {
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

  // Error toast trigger
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
      <Sparkles size={12} class="spark-icon" />
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
  <DashboardStats {stats} />

  <!-- Toolbar Filters -->
  <DashboardToolbar 
    bind:filterStatus 
    bind:sortBy 
    {posts} 
  />

  <!-- Main Workspace -->
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
            <DashboardPostItem 
              {post}
              isSelected={selectedPost && selectedPost.id === post.id}
              {publishingId}
              {formatDateTime}
              on:select={(e) => selectedPost = e.detail}
              on:publish={(e) => publishNow(e.detail.id, e.detail.event)}
              on:delete={(e) => deletePost(e.detail.id, e.detail.event)}
            />
          {/each}
        {/if}
      </div>

      <!-- Right Column: Details & Feed Mockup -->
      <DashboardPostDetails 
        {selectedPost}
        {getPostEngagement}
      />
    </div>
  {/if}
</div>

<style>
  /* Page Header */
  .studio-page-header {
    padding: 24px 32px 22px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: flex-end;
    gap: 24px;
    background: var(--bg-app);
  }

  .studio-page-header-info {
    flex: 1;
    min-width: 0;
  }

  .studio-eyebrow {
    font-family: var(--font-mono);
    font-size: 11.5px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 4px;
  }

  .studio-page-header h1 {
    font-size: 1.875rem;
    font-weight: 600;
    letter-spacing: -0.03em;
    line-height: 1.15;
    color: var(--text);
  }

  .header-divider {
    color: var(--text-dim);
    font-weight: 400;
  }

  .header-count {
    color: var(--text-muted);
    font-weight: 400;
  }

  .header-actions {
    display: flex;
    gap: 8px;
  }

  .studio-btn-accent :global(.spark-icon) {
    color: var(--accent-ink);
  }

  /* Page Body */
  .studio-page-body {
    flex: 1;
    padding: 24px 32px 28px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    min-height: 0;
  }

  /* Workspace Grid layout */
  .studio-workspace-grid {
    display: grid;
    grid-template-columns: 460px 1fr;
    gap: 20px;
    min-height: 0;
    flex: 1;
  }

  @media (max-width: 1024px) {
    .studio-workspace-grid {
      grid-template-columns: 1fr;
    }
  }

  .posts-scroll-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    overflow-y: auto;
    max-height: calc(100vh - 210px);
    padding-right: 4px;
  }

  /* Empty state */
  .empty-list-card {
    padding: 40px 20px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    border: 1px dashed var(--border);
    border-radius: var(--radius-xl);
  }

  .empty-list-card :global(.empty-icon) {
    color: var(--text-dim);
  }

  .empty-list-card h3 {
    font-size: var(--type-h3-size);
    font-weight: var(--type-h3-weight);
    color: var(--text);
  }

  .empty-list-card p {
    font-size: var(--type-body-size);
    color: var(--text-muted);
    max-width: 280px;
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

  .studio-btn-accent {
    background: var(--accent);
    color: var(--accent-ink);
  }

  .studio-btn-accent:hover {
    background: var(--accent);
    opacity: 0.9;
  }

  /* Loader */
  .loader-container {
    padding: 80px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    border: 1px dashed var(--border);
    border-radius: 12px;
    background: var(--surface);
  }

  .loader-text {
    font-size: 13.5px;
    color: var(--text-muted);
  }

  .spinner {
    width: 28px;
    height: 28px;
    border: 2px solid var(--border-strong);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
