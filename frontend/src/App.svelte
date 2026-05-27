<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from './components/Sidebar.svelte';
  import Dashboard from './pages/Dashboard.svelte';
  import CreatePost from './pages/CreatePost.svelte';
  import Automation from './pages/Automation.svelte';
  import Settings from './pages/Settings.svelte';
  import SearchModal from './components/SearchModal.svelte';

  // Gerenciamento de rotas/páginas simples
  let activePage: 'dashboard' | 'create' | 'settings' | 'automation' = 'dashboard';
  let theme: 'dark' | 'light' = 'dark';
  let showSearchModal = false;
  let searchSelectedPostId: string | null = null;

  onMount(() => {
    const storedTheme = localStorage.getItem('quill-theme');
    if (storedTheme === 'light' || storedTheme === 'dark') {
      theme = storedTheme;
    }
    applyTheme();

    // Check URL path first for direct routing (e.g. from OAuth redirects or direct navigation)
    const path = window.location.pathname;
    if (path.includes('/settings')) {
      activePage = 'settings';
      localStorage.setItem('quill-active-page', 'settings');
    } else if (path.includes('/create')) {
      activePage = 'create';
      localStorage.setItem('quill-active-page', 'create');
    } else if (path.includes('/automation')) {
      activePage = 'automation';
      localStorage.setItem('quill-active-page', 'automation');
    } else {
      const storedPage = localStorage.getItem('quill-active-page');
      if (storedPage === 'dashboard' || storedPage === 'create' || storedPage === 'settings' || storedPage === 'automation') {
        activePage = storedPage;
      }
    }
  });

  function applyTheme() {
    if (theme === 'light') {
      document.documentElement.classList.add('theme-light');
    } else {
      document.documentElement.classList.remove('theme-light');
    }
    localStorage.setItem('quill-theme', theme);
  }

  function handleToggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    applyTheme();
  }

  // Tratar navegação
  function handleNavigate(event: CustomEvent<'dashboard' | 'create' | 'settings' | 'automation'> | any) {
    const targetPage = event.detail !== undefined ? event.detail : event;
    activePage = targetPage;
    localStorage.setItem('quill-active-page', targetPage);
  }

  // Ouvir atalhos de teclado (⌘K ou Ctrl+K) para abrir busca
  function handleKeyDown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
      event.preventDefault();
      showSearchModal = !showSearchModal;
    }
  }

  function handleSelectPostFromSearch(event: CustomEvent<string>) {
    searchSelectedPostId = event.detail;
    activePage = 'dashboard';
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<div class="app-container">
  <Sidebar 
    activePage={activePage} 
    theme={theme}
    on:navigate={handleNavigate} 
    on:toggleTheme={handleToggleTheme}
    on:openSearch={() => showSearchModal = true}
  />
  
  <main class="main-content">
    {#if activePage === 'dashboard'}
      <Dashboard 
        on:navigate={handleNavigate} 
        selectedPostId={searchSelectedPostId} 
      />
    {:else if activePage === 'create'}
      <CreatePost />
    {:else if activePage === 'automation'}
      <Automation on:navigate={handleNavigate} />
    {:else if activePage === 'settings'}
      <Settings />
    {/if}
  </main>
</div>

<SearchModal 
  show={showSearchModal} 
  on:close={() => showSearchModal = false}
  on:select={handleSelectPostFromSearch}
/>

<style>
  /* Base structural layout is defined globally in app.css */
</style>
