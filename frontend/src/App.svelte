<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from './components/Sidebar.svelte';
  import Dashboard from './pages/Dashboard.svelte';
  import CreatePost from './pages/CreatePost.svelte';
  import Settings from './pages/Settings.svelte';
  import SearchModal from './components/SearchModal.svelte';

  // Gerenciamento de rotas/páginas simples
  let activePage: 'dashboard' | 'create' | 'settings' = 'dashboard';
  let theme: 'dark' | 'light' = 'dark';
  let showSearchModal = false;
  let searchSelectedPostId: string | null = null;

  onMount(() => {
    const storedTheme = localStorage.getItem('quill-theme');
    if (storedTheme === 'light' || storedTheme === 'dark') {
      theme = storedTheme;
    }
    applyTheme();
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
  function handleNavigate(event: CustomEvent<'dashboard' | 'create' | 'settings'> | any) {
    const targetPage = event.detail !== undefined ? event.detail : event;
    activePage = targetPage;
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
        theme={theme} 
        on:navigate={handleNavigate} 
        selectedPostId={searchSelectedPostId} 
      />
    {:else if activePage === 'create'}
      <CreatePost theme={theme} />
    {:else if activePage === 'settings'}
      <Settings theme={theme} />
    {/if}
  </main>
</div>

<SearchModal 
  show={showSearchModal} 
  theme={theme} 
  on:close={() => showSearchModal = false}
  on:select={handleSelectPostFromSearch}
/>

<style>
  /* Base structural layout is defined globally in app.css */
</style>
