<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import { LayoutDashboard, Sparkles, Settings, Search, Moon, Sun, Zap } from '@lucide/svelte';
  import { API_URL } from '../lib/api';

  export let activePage: 'dashboard' | 'create' | 'settings' | 'automation' = 'dashboard';
  export let theme: 'dark' | 'light' = 'dark';

  const dispatch = createEventDispatcher();

  let isAuthenticated = false;
  let daysRemaining = 28;

  onMount(async () => {
    await checkLinkedInStatus();
  });

  async function checkLinkedInStatus() {
    try {
      const res = await fetch(`${API_URL}/api/auth/linkedin/status`);
      if (res.ok) {
        const data = await res.json();
        isAuthenticated = data.authenticated;
        if (data.expires_at) {
          const exp = new Date(data.expires_at);
          const diff = exp.getTime() - Date.now();
          daysRemaining = Math.max(0, Math.ceil(diff / (1000 * 60 * 60 * 24)));
        }
      }
    } catch (e) {
      console.error("Erro ao verificar status do LinkedIn na Sidebar", e);
    }
  }

  function navigate(page: 'dashboard' | 'create' | 'settings' | 'automation') {
    dispatch('navigate', page);
  }

  function toggleTheme() {
    dispatch('toggleTheme');
  }
</script>

<aside class="sidebar">
  <!-- Wordmark Logo -->
  <div class="sidebar-brand">
    <div class="brand-icon">
      <Sparkles size={14} />
    </div>
    <div class="brand-text">
      <div class="brand-name">LinkedMaker</div>
      <div class="brand-version">posts · v2</div>
    </div>
  </div>

  <!-- Search Shortcut Button -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="search-bar" on:click={() => dispatch('openSearch')} style="cursor: pointer;">
    <Search size={13} class="search-icon" />
    <span class="search-placeholder">Buscar</span>
    <kbd class="search-kbd">⌘K</kbd>
  </div>

  <!-- Navigation Section -->
  <nav class="sidebar-nav">
    <button 
      class="nav-link {activePage === 'dashboard' ? 'active' : ''}" 
      on:click={() => navigate('dashboard')}
    >
      <LayoutDashboard size={15} />
      <span class="nav-label">Dashboard</span>
    </button>

    <button 
      class="nav-link {activePage === 'create' ? 'active' : ''}" 
      on:click={() => navigate('create')}
    >
      <Sparkles size={15} class="create-icon" />
      <span class="nav-label">Criar post</span>
    </button>

    <button 
      class="nav-link {activePage === 'automation' ? 'active' : ''}" 
      on:click={() => navigate('automation')}
    >
      <Zap size={15} class="automation-icon" />
      <span class="nav-label">Automação</span>
    </button>

    <button 
      class="nav-link {activePage === 'settings' ? 'active' : ''}" 
      on:click={() => navigate('settings')}
    >
      <Settings size={15} />
      <span class="nav-label">Configurações</span>
    </button>
  </nav>

  <div class="sidebar-spacer"></div>

  <!-- LinkedIn Connection Card -->
  <div class="connection-card">
    <div class="card-status-row">
      <span class="status-indicator-dot {isAuthenticated ? 'connected' : 'simulated'}"></span>
      <span class="status-title">{isAuthenticated ? 'LinkedIn ativo' : 'Modo Simulação'}</span>
    </div>
    <div class="card-desc">
      {#if isAuthenticated}
        Token expira em {daysRemaining} dias.
      {:else}
        Rodando localmente em simulação.
      {/if}
    </div>
    <div class="progress-bar-container">
      <div class="progress-bar-fill {isAuthenticated ? 'connected' : 'simulated'}" style="width: {isAuthenticated ? Math.min(100, (daysRemaining / 60) * 100) : 100}%"></div>
    </div>
  </div>

  <!-- User Chip Footer -->
  <div class="user-chip">
    <div class="user-avatar">RA</div>
    <div class="user-info">
      <div class="user-name">Ricardo Augusto</div>
      <div class="user-email">ricardo@linkedmaker.dev</div>
    </div>
    <button class="theme-toggle-btn" on:click={toggleTheme} title="Alternar tema">
      {#if theme === 'dark'}
        <Sun size={14} />
      {:else}
        <Moon size={14} />
      {/if}
    </button>
  </div>
</aside>

<style>
  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    width: var(--sidebar-width);
    height: 100vh;
    background: var(--surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    padding: 20px 16px 18px;
    z-index: 100;
    transition: background-color var(--transition-normal), border-color var(--transition-normal);
  }

  .sidebar-brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 2px 6px 22px;
  }

  .brand-icon {
    width: 30px;
    height: 30px;
    border-radius: 9px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    color: var(--accent);
    box-shadow: 0 1px 2px rgba(0,0,0,0.05);
  }

  :global(.theme-light) .brand-icon {
    background: var(--text);
    border: none;
    box-shadow: 0 1px 2px rgba(0,0,0,0.08);
  }

  .brand-text {
    display: flex;
    flex-direction: column;
  }

  .brand-name {
    font-size: 15px;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.1;
  }

  .brand-version {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    line-height: 1.1;
    margin-top: 2px;
  }

  /* Search Bar */
  .search-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 10px;
    margin-bottom: 18px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--text-muted);
    font-size: 12.5px;
  }

  :global(.search-icon) {
    color: var(--text-dim);
  }

  .search-placeholder {
    flex: 1;
    color: var(--text-dim);
  }

  .search-kbd {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--text-dim);
  }

  /* Sidebar Navigation */
  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-link {
    width: 100%;
    text-align: left;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    background: transparent;
    border: none;
    border-radius: 8px;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 13px;
    font-weight: 400;
    cursor: pointer;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .nav-link:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .nav-link.active {
    background-color: var(--surface-alt);
    color: var(--text);
    font-weight: 500;
  }

  .nav-link.active :global(svg) {
    color: var(--text);
  }

  .nav-link :global(svg) {
    color: var(--text-muted);
    transition: color var(--transition-fast);
  }

  .nav-link.active :global(svg.create-icon) {
    color: var(--accent);
  }

  .nav-link:hover :global(svg.create-icon) {
    color: var(--accent);
  }

  .nav-link.active :global(svg.automation-icon) {
    color: var(--accent);
  }

  .nav-link:hover :global(svg.automation-icon) {
    color: var(--accent);
  }

  .sidebar-spacer {
    flex: 1;
  }

  /* Connection Card */
  .connection-card {
    padding: 12px 14px;
    border-radius: 10px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    margin-bottom: 12px;
    transition: border-color var(--transition-normal);
  }

  .card-status-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    color: var(--text);
    margin-bottom: 6px;
  }

  .status-indicator-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .status-indicator-dot.connected {
    background-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-muted);
  }

  .status-indicator-dot.simulated {
    background-color: var(--amber);
    box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.15);
  }

  .status-title {
    font-weight: 500;
  }

  .card-desc {
    font-size: 11.5px;
    color: var(--text-muted);
    line-height: 1.4;
    margin-bottom: 8px;
  }

  .progress-bar-container {
    height: 3px;
    background: var(--bg-app);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width var(--transition-normal);
  }

  .progress-bar-fill.connected {
    background-color: var(--accent);
  }

  .progress-bar-fill.simulated {
    background-color: var(--amber);
  }

  /* User Chip */
  .user-chip {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 4px 6px;
  }

  .user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--avatar-a), var(--avatar-b));
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 11.5px;
    letter-spacing: -0.02em;
    flex-shrink: 0;
    box-shadow: inset 0 0 0 1px rgba(255,255,255,0.04);
  }

  .user-info {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 13px;
    font-weight: 500;
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .user-email {
    font-size: 11px;
    color: var(--text-dim);
    line-height: 1.2;
    margin-top: 2px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .theme-toggle-btn {
    background: transparent;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .theme-toggle-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }
</style>
