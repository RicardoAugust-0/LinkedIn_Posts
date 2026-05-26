<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    Save, Link, Link2Off, Eye, EyeOff, Check, Copy, ExternalLink,
    AlertCircle, CheckCircle2, FileText, Info
  } from '@lucide/svelte';

  export let theme: 'dark' | 'light' = 'dark';

  // Configurações
  let geminiKey = '';
  let googleSearchKey = '';
  let googleSearchCx = '';
  let linkedinClientId = '';
  let linkedinClientSecret = '';

  let loading = true;
  let saving = false;
  
  // Toasts
  let showSaveSuccess = false;
  let authSuccessMessage = false;
  let authErrorMessage = false;
  
  // LinkedIn Auth Status
  let isAuthenticated = false;
  let expiresAt: string | null = null;
  let disconnecting = false;

  // Toggle visibilidade da chave
  let showGemini = false;
  let showGoogle = false;
  let showLinkedIn = false;

  // Preferências
  let prefAiModel = 'gemini-1.5-flash';
  let prefCharLimit = 3000;

  // Avançado
  let clearingDb = false;
  let activeAnchor = 'linkedin';

  onMount(async () => {
    loadPreferences();
    // Verificar URL query params para toasts de login
    const params = new URLSearchParams(window.location.search);
    if (params.get('auth') === 'success') {
      authSuccessMessage = true;
      window.history.replaceState({}, document.title, window.location.pathname);
      setTimeout(() => authSuccessMessage = false, 5000);
    } else if (params.get('auth') === 'error') {
      authErrorMessage = true;
      window.history.replaceState({}, document.title, window.location.pathname);
      setTimeout(() => authErrorMessage = false, 5000);
    }

    await loadSettings();
    await checkAuthStatus();
  });

  async function loadSettings() {
    try {
      const res = await fetch('http://localhost:3000/api/settings');
      if (res.ok) {
        const data = await res.json();
        geminiKey = data.gemini_key || '';
        googleSearchKey = data.google_search_key || '';
        googleSearchCx = data.google_search_cx || '';
        linkedinClientId = data.linkedin_client_id || '';
        linkedinClientSecret = data.linkedin_client_secret || '';
      }
    } catch (e) {
      console.error("Erro ao carregar configurações", e);
    } finally {
      loading = false;
    }
  }

  async function checkAuthStatus() {
    try {
      const res = await fetch('http://localhost:3000/api/auth/linkedin/status');
      if (res.ok) {
        const data = await res.json();
        isAuthenticated = data.authenticated;
        expiresAt = data.expires_at ? new Date(data.expires_at).toLocaleDateString('pt-BR') : null;
      }
    } catch (e) {
      console.error("Erro ao verificar autenticação do LinkedIn", e);
    }
  }

  async function saveSettings() {
    saving = true;
    showSaveSuccess = false;
    try {
      const res = await fetch('http://localhost:3000/api/settings', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          gemini_key: geminiKey || null,
          google_search_key: googleSearchKey || null,
          google_search_cx: googleSearchCx || null,
          linkedin_client_id: linkedinClientId || null,
          linkedin_client_secret: linkedinClientSecret || null
        })
      });

      if (res.ok) {
        showSaveSuccess = true;
        setTimeout(() => showSaveSuccess = false, 3000);
        await checkAuthStatus(); // Recarregar em caso de chaves alteradas
      }
    } catch (e) {
      console.error("Erro ao salvar configurações", e);
    } finally {
      saving = false;
    }
  }

  function startLinkedInAuth() {
    window.location.href = 'http://localhost:3000/api/auth/linkedin';
  }

  async function disconnectLinkedIn() {
    if (!confirm("Deseja realmente desconectar sua conta do LinkedIn? Suas publicações voltarão a rodar em Modo de Simulação.")) return;
    
    disconnecting = true;
    try {
      const res = await fetch('http://localhost:3000/api/auth/linkedin/disconnect', {
        method: 'POST'
      });
      if (res.ok) {
        isAuthenticated = false;
        expiresAt = null;
        showSaveSuccess = true;
        setTimeout(() => showSaveSuccess = false, 3000);
        await checkAuthStatus();
      } else {
        alert("Erro ao desconectar LinkedIn.");
      }
    } catch (e) {
      console.error(e);
      alert("Erro ao conectar ao servidor backend.");
    } finally {
      disconnecting = false;
    }
  }

  function copyKeyToClipboard(text: string) {
    if (!text) return;
    navigator.clipboard.writeText(text);
  }

  function loadPreferences() {
    const model = localStorage.getItem('quill-pref-ai-model');
    if (model) prefAiModel = model;
    
    const limit = localStorage.getItem('quill-pref-char-limit');
    if (limit) prefCharLimit = parseInt(limit, 10);
  }

  function savePreferences() {
    localStorage.setItem('quill-pref-ai-model', prefAiModel);
    localStorage.setItem('quill-pref-char-limit', String(prefCharLimit));
  }

  async function clearDatabase() {
    if (!confirm("⚠️ ATENÇÃO: Tem certeza absoluta de que deseja excluir TODOS os posts do banco de dados? Esta ação é irreversível e apagará rascunhos, agendados e publicados.")) {
      return;
    }

    clearingDb = true;
    try {
      const res = await fetch('http://localhost:3000/api/posts', {
        method: 'DELETE'
      });

      if (res.ok) {
        showSaveSuccess = true;
        setTimeout(() => showSaveSuccess = false, 3000);
        alert("Banco de dados limpo com sucesso! Todos os posts foram excluídos.");
      } else {
        const text = await res.text();
        alert(`Erro ao limpar banco: ${text}`);
      }
    } catch (e) {
      console.error(e);
      alert("Erro ao conectar ao servidor backend.");
    } finally {
      clearingDb = false;
    }
  }

  function scrollToSection(id: string) {
    activeAnchor = id;
    const el = document.getElementById(id + '-section');
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }
</script>

<div class="studio-page-header">
  <div class="studio-page-header-info">
    <div class="studio-eyebrow">Configurações</div>
    <h1>Credenciais & conexões</h1>
    <p class="studio-subtitle">Chaves armazenadas localmente em seu arquivo de configuração local. Nada é compartilhado externamente.</p>
  </div>
  
  <div class="header-actions">
    <a 
      href="https://developer.linkedin.com" 
      target="_blank" 
      rel="noopener noreferrer" 
      class="studio-btn studio-btn-secondary"
    >
      <span>Documentação</span>
      <ExternalLink size={11} />
    </a>
  </div>
</div>

<!-- Toast Notifications -->
<div class="studio-toast-container">
  {#if authSuccessMessage}
    <div class="studio-toast studio-toast-success">
      <CheckCircle2 size={16} />
      <span>Conectado com sucesso ao LinkedIn! Publicações ativas.</span>
    </div>
  {/if}
  {#if authErrorMessage}
    <div class="studio-toast studio-toast-error">
      <AlertCircle size={16} />
      <span>Falha ao conectar com o LinkedIn. Verifique suas credenciais.</span>
    </div>
  {/if}
  {#if showSaveSuccess}
    <div class="studio-toast studio-toast-success">
      <CheckCircle2 size={16} />
      <span>Configurações salvas com sucesso!</span>
    </div>
  {/if}
</div>

<div class="studio-page-body">
  {#if loading}
    <div class="loader-container">
      <div class="spinner"></div>
      <p class="loader-text">Carregando configurações locais...</p>
    </div>
  {:else}
    <div class="settings-workspace-layout">
      <!-- Left side anchors column -->
      <nav class="settings-anchors-nav">
        <button class="anchor-btn {activeAnchor === 'linkedin' ? 'active' : ''}" on:click={() => scrollToSection('linkedin')}>Conexão LinkedIn</button>
        <button class="anchor-btn {activeAnchor === 'keys' ? 'active' : ''}" on:click={() => scrollToSection('keys')}>Chaves de IA</button>
        <button class="anchor-btn {activeAnchor === 'search' ? 'active' : ''}" on:click={() => scrollToSection('search')}>Busca de imagens</button>
        <button class="anchor-btn {activeAnchor === 'preferences' ? 'active' : ''}" on:click={() => scrollToSection('preferences')}>Preferências</button>
        <button class="anchor-btn {activeAnchor === 'advanced' ? 'active' : ''}" on:click={() => scrollToSection('advanced')}>Avançado</button>
        
        <div class="local-file-card">
          <div class="local-file-label">arquivo de config</div>
          <div class="local-file-path">~/.quill/.env</div>
          <div class="local-file-desc">Modificado localmente.</div>
        </div>
      </nav>

      <!-- Right side cards column -->
      <div class="settings-cards-list">
        <!-- LinkedIn connection card -->
        <section id="linkedin-section" class="studio-card settings-card">
          <div class="card-title-row">
            <div class="title-details">
              <h2>Conexão LinkedIn</h2>
              <p class="card-desc">Sua conta autorizada para publicar via API oficial do LinkedIn.</p>
            </div>
            
            <span class="status-indicator-badge {isAuthenticated ? 'connected' : 'simulated'}">
              <span class="status-dot"></span>
              {isAuthenticated ? 'Ativo' : 'Simulador'}
            </span>
          </div>

          <div class="connection-profile-row">
            <div class="profile-avatar">RA</div>
            <div class="profile-details">
              <div class="profile-name">Ricardo Augusto</div>
              <div class="profile-email">
                {#if isAuthenticated}
                  Conectado como ricardo@quill.dev
                {:else}
                  LinkedIn rodando em Modo de Simulação (Mock)
                {/if}
              </div>
              
              <div class="profile-meta-row">
                <span>token: <span class="meta-highlight">{isAuthenticated ? '•••••••••fa92e' : 'mock_token'}</span></span>
                <span>expira: <span class="meta-highlight">{expiresAt || 'Simulação sem expiração'}</span></span>
                <span>escopo: <span class="meta-highlight">w_member_social</span></span>
              </div>
            </div>

            <div class="profile-actions-wrapper" style="display: flex; gap: 8px;">
              <button type="button" class="studio-btn studio-btn-secondary" on:click={startLinkedInAuth}>
                {isAuthenticated ? 'Renovar token' : 'Conectar Conta'}
              </button>
              {#if isAuthenticated}
                <button type="button" class="studio-btn studio-btn-danger" on:click={disconnectLinkedIn} disabled={disconnecting}>
                  Desconectar
                </button>
              {/if}
            </div>
          </div>
        </section>

        <!-- Credentials forms card -->
        <section id="keys-section" class="studio-card settings-card">
          <div class="card-title-row header-only">
            <div class="title-details">
              <h2>Chaves de IA</h2>
              <p class="card-desc">Google AI Studio. Necessário para geração de textos (Gemini) e imagens (Imagen 3).</p>
            </div>
          </div>

          <form on:submit|preventDefault={saveSettings} class="settings-form">
            <!-- Gemini Key input -->
            <div class="studio-field">
              <div class="studio-field-header">
                <label class="studio-label" for="gemini-key">Gemini API Key</label>
                <span class="status-indicator-inline {geminiKey ? 'valid' : 'empty'}">
                  <span class="status-dot"></span>
                  {geminiKey ? 'Verificada' : 'Ausente'}
                </span>
              </div>
              <div class="key-input-container">
                <input 
                  id="gemini-key"
                  type={showGemini ? "text" : "password"} 
                  class="studio-input key-input-field" 
                  bind:value={geminiKey}
                  placeholder="Insira sua Gemini API Key"
                />
                <div class="key-actions-group">
                  <button type="button" class="key-action-btn" on:click={() => showGemini = !showGemini}>
                    {#if showGemini}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
                  </button>
                  <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(geminiKey)}>
                    <Copy size={13} />
                  </button>
                </div>
              </div>
              <div class="studio-hint">Modelo utilizado: gemini-2.5-pro / Imagen 3.</div>
            </div>

            <!-- Imagen 3 Key input (shares gemini key indicator) -->
            <div class="studio-field">
              <div class="studio-field-header">
                <label class="studio-label" for="imagen-key">Imagen 3 API Key</label>
                <span class="status-indicator-inline valid">
                  <span class="status-dot"></span>
                  Compartilha Gemini
                </span>
              </div>
              <div class="key-input-container">
                <input 
                  id="imagen-key"
                  type={showGemini ? "text" : "password"} 
                  class="studio-input key-input-field" 
                  value={geminiKey}
                  placeholder="Usa a mesma chave do Gemini por padrão"
                  disabled
                />
                <div class="key-actions-group">
                  <button type="button" class="key-action-btn" on:click={() => showGemini = !showGemini}>
                    {#if showGemini}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
                  </button>
                </div>
              </div>
              <div class="studio-hint">Para geração de ilustrações no Wizard do post.</div>
            </div>

            <!-- Google Custom Search keys -->
            <div id="search-section" class="card-title-row header-only inner-divider">
              <div class="title-details">
                <h2>Busca de imagens</h2>
                <p class="card-desc">Google Custom Search Engine para a aba 'Buscar no Google' no criador de posts.</p>
              </div>
            </div>

            <div class="keys-grid-row">
              <div class="studio-field">
                <div class="studio-field-header">
                  <label class="studio-label" for="search-key">Google Search API Key</label>
                  <span class="status-indicator-inline {googleSearchKey ? 'valid' : 'empty'}">
                    <span class="status-dot"></span>
                    {googleSearchKey ? 'Verificada' : 'Ausente'}
                  </span>
                </div>
                <div class="key-input-container">
                  <input 
                    id="search-key"
                    type={showGoogle ? "text" : "password"} 
                    class="studio-input key-input-field" 
                    bind:value={googleSearchKey}
                    placeholder="Search API Key"
                  />
                  <div class="key-actions-group">
                    <button type="button" class="key-action-btn" on:click={() => showGoogle = !showGoogle}>
                      {#if showGoogle}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
                    </button>
                    <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(googleSearchKey)}>
                      <Copy size={13} />
                    </button>
                  </div>
                </div>
              </div>

              <div class="studio-field">
                <div class="studio-field-header">
                  <label class="studio-label" for="search-cx">Search Engine ID (CX)</label>
                  <span class="status-indicator-inline {googleSearchCx ? 'valid' : 'empty'}">
                    <span class="status-dot"></span>
                    {googleSearchCx ? 'Verificado' : 'Ausente'}
                  </span>
                </div>
                <input 
                  id="search-cx"
                  type="text" 
                  class="studio-input mono-text" 
                  bind:value={googleSearchCx}
                  placeholder="Ex: 017a4c8e9b..."
                />
              </div>
            </div>

            <!-- Client ID & Secret LinkedIn App -->
            <div class="card-title-row header-only inner-divider">
              <div class="title-details">
                <h2>Credenciais LinkedIn Developer</h2>
                <p class="card-desc">Necessário para publicar de verdade na sua conta. Caso contrário, roda em Simulação.</p>
              </div>
            </div>

            <div class="keys-grid-row">
              <div class="studio-field">
                <label class="studio-label" for="linkedin-client">LinkedIn Client ID</label>
                <input 
                  id="linkedin-client"
                  type="text" 
                  class="studio-input" 
                  bind:value={linkedinClientId}
                  placeholder="Client ID da Developer App"
                />
              </div>

              <div class="studio-field">
                <label class="studio-label" for="linkedin-secret">LinkedIn Client Secret</label>
                <div class="key-input-container">
                  <input 
                    id="linkedin-secret"
                    type={showLinkedIn ? "text" : "password"} 
                    class="studio-input key-input-field" 
                    bind:value={linkedinClientSecret}
                    placeholder="Client Secret"
                  />
                  <div class="key-actions-group">
                    <button type="button" class="key-action-btn" on:click={() => showLinkedIn = !showLinkedIn}>
                      {#if showLinkedIn}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
                    </button>
                    <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(linkedinClientSecret)}>
                      <Copy size={13} />
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <!-- Form Save Actions -->
            <div class="form-save-actions">
              <button type="submit" class="studio-btn studio-btn-accent" disabled={saving}>
                <Save size={12} />
                <span>{saving ? 'Salvando...' : 'Salvar Configurações'}</span>
              </button>
            </div>
          </form>
        </section>

        <!-- Preferências Card -->
        <section id="preferences-section" class="studio-card settings-card">
          <div class="card-title-row header-only">
            <div class="title-details">
              <h2>Preferências do Sistema</h2>
              <p class="card-desc">Personalize o comportamento do gerador de posts e do layout.</p>
            </div>
          </div>

          <div class="settings-form" style="margin-top: 15px;">
            <div class="keys-grid-row">
              <div class="studio-field">
                <label class="studio-label" for="pref-ai-model">Modelo de IA Padrão</label>
                <select id="pref-ai-model" class="studio-input" bind:value={prefAiModel} on:change={savePreferences} style="width: 100%; height: 38px; padding: 0 12px; background: var(--surface); border: 1px solid var(--border); border-radius: 6px; color: var(--text);">
                  <option value="gemini-1.5-flash">Gemini 1.5 Flash (Rápido e Conciso)</option>
                  <option value="gemini-1.5-pro">Gemini 1.5 Pro (Criativo e Detalhado)</option>
                </select>
              </div>

              <div class="studio-field">
                <label class="studio-label" for="pref-char-limit">Aviso de Limite de Caracteres</label>
                <input 
                  id="pref-char-limit"
                  type="number" 
                  class="studio-input" 
                  bind:value={prefCharLimit} 
                  on:input={savePreferences}
                  min="500" 
                  max="10000"
                />
              </div>
            </div>
          </div>
        </section>

        <!-- Configurações Avançadas Card -->
        <section id="advanced-section" class="studio-card settings-card">
          <div class="card-title-row header-only">
            <div class="title-details">
              <h2>Configurações Avançadas</h2>
              <p class="card-desc">Operações críticas do sistema e diagnósticos do banco de dados.</p>
            </div>
          </div>

          <div class="advanced-workspace">
            <div class="db-diagnostics-row">
              <div class="diagnostic-item">
                <span class="diagnostic-label">Status da Conexão:</span>
                <span class="diagnostic-val active">
                  <span class="status-dot"></span>
                  Conectado (SQLite)
                </span>
              </div>
              <div class="diagnostic-item" style="margin-top: 8px;">
                <span class="diagnostic-label">Arquivo do Banco:</span>
                <span class="diagnostic-val font-mono">sqlite:posts.db</span>
              </div>
            </div>

            <div class="danger-zone-divider">Zona de Risco</div>
            
            <div class="danger-action-row">
              <div class="danger-action-details">
                <h4>Limpar Banco de Dados</h4>
                <p>Exclui permanentemente todas as publicações (rascunhos, agendados e publicados) do banco de dados local. Esta ação não pode ser desfeita.</p>
              </div>
              <button type="button" class="studio-btn studio-btn-danger" on:click={clearDatabase} disabled={clearingDb}>
                {#if clearingDb}
                  <span>Limpando...</span>
                {:else}
                  <span>Excluir todos os posts</span>
                {/if}
              </button>
            </div>
          </div>
        </section>

        <!-- Help Documentation Card -->
        <section class="studio-card developer-help-card">
          <div class="help-card-header">
            <Info size={16} class="info-icon" />
            <h3>Como conectar de verdade à API do LinkedIn?</h3>
          </div>
          <p class="help-card-desc">Para que suas publicações cheguem à sua conta pessoal do LinkedIn, siga o passo a passo abaixo:</p>
          <ol class="help-steps-list">
            <li>Crie uma conta de desenvolvedor em <a href="https://developer.linkedin.com" target="_blank" rel="noopener noreferrer">LinkedIn Developer Portal</a>.</li>
            <li>Crie um novo aplicativo e solicite o produto de publicação <strong>Share on LinkedIn</strong> (escopo <code>w_member_social</code>).</li>
            <li>Nas configurações do App do LinkedIn, registre o endereço callback autorizado: <code class="code-highlight">http://localhost:3000/api/auth/linkedin/callback</code>.</li>
            <li>Insira o Client ID e Client Secret gerados no formulário acima, clique em Salvar e depois clique em "Conectar Conta" no primeiro painel.</li>
          </ol>
        </section>
      </div>
    </div>
  {/if}
</div>

<style>
  .settings-workspace-layout {
    display: grid;
    grid-template-columns: 220px 1fr;
    gap: 32px;
    min-height: 0;
    flex: 1;
  }

  @media (max-width: 1024px) {
    .settings-workspace-layout {
      grid-template-columns: 1fr;
    }
  }

  /* Left Tab Anchors */
  .settings-anchors-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-top: 4px;
  }

  .anchor-btn {
    width: 100%;
    text-align: left;
    padding: 7px 10px;
    background: transparent;
    border: none;
    border-left: 2px solid transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 13px;
    font-weight: 400;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .anchor-btn:hover {
    color: var(--text);
  }

  .anchor-btn.active {
    color: var(--text);
    font-weight: 500;
    border-left-color: var(--text);
  }

  :global(.theme-dark) .anchor-btn.active {
    border-left-color: var(--accent);
  }

  .local-file-card {
    margin-top: 14px;
    padding: 12px 14px;
    border: 1px dashed var(--border);
    border-radius: 9px;
    font-size: 11.5px;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .local-file-label {
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--text-dim);
    margin-bottom: 6px;
  }

  .local-file-path {
    color: var(--text);
    font-family: var(--font-mono);
    font-size: 11.5px;
    word-break: break-all;
  }

  .local-file-desc {
    margin-top: 4px;
    color: var(--text-dim);
  }

  /* Right Cards List */
  .settings-cards-list {
    display: flex;
    flex-direction: column;
    gap: 18px;
    overflow-y: auto;
    max-height: calc(100vh - 210px);
    padding-right: 4px;
  }

  .settings-card {
    display: flex;
    flex-direction: column;
  }

  .card-title-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
    padding-bottom: 14px;
    border-bottom: 1px solid var(--border);
  }

  .card-title-row.header-only {
    border-bottom: none;
    padding-bottom: 8px;
  }

  .card-title-row.inner-divider {
    border-top: 1px solid var(--border);
    padding-top: 20px;
    margin-top: 8px;
  }

  .title-details {
    flex: 1;
  }

  .card-desc {
    font-size: 12.5px;
    color: var(--text-muted);
    margin-top: 4px;
    line-height: 1.45;
  }

  /* Status Badge Indicator */
  .status-indicator-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 99px;
    font-size: 11.5px;
    font-weight: 500;
    border: 1px solid transparent;
  }

  .status-indicator-badge .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  .status-indicator-badge.connected {
    background: var(--accent-muted);
    color: var(--accent);
    border-color: rgba(163, 230, 53, 0.2);
  }
  .status-indicator-badge.connected .status-dot {
    background-color: var(--accent);
    box-shadow: 0 0 8px var(--accent);
  }

  .status-indicator-badge.simulated {
    background: rgba(251, 191, 36, 0.08);
    color: var(--amber);
    border-color: rgba(251, 191, 36, 0.15);
  }
  .status-indicator-badge.simulated .status-dot {
    background-color: var(--amber);
  }

  /* Connection Profile Row */
  .connection-profile-row {
    display: flex;
    align-items: center;
    gap: 18px;
    padding-top: 16px;
  }

  @media (max-width: 640px) {
    .connection-profile-row {
      flex-direction: column;
      align-items: flex-start;
      gap: 14px;
    }
  }

  .profile-avatar {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--avatar-a), var(--avatar-b));
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 20.16px;
    letter-spacing: -0.02em;
    flex-shrink: 0;
    box-shadow: inset 0 0 0 1px rgba(255,255,255,0.04);
  }

  .profile-details {
    flex: 1;
    min-width: 0;
  }

  .profile-name {
    font-size: 15px;
    font-weight: 600;
    color: var(--text);
  }

  .profile-email {
    font-size: 12.5px;
    color: var(--text-muted);
    margin-top: 3px;
  }

  .profile-meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 18px;
    font-size: 11px;
    color: var(--text-dim);
    font-family: var(--font-mono);
    margin-top: 10px;
  }

  .meta-highlight {
    color: var(--text);
  }

  .profile-actions-wrapper {
    display: flex;
    gap: 8px;
  }

  /* Form & Key fields */
  .settings-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 12px;
  }

  .status-indicator-inline {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 11.5px;
  }

  .status-indicator-inline .status-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
  }

  .status-indicator-inline.valid {
    color: var(--accent);
  }
  .status-indicator-inline.valid .status-dot {
    background-color: var(--accent);
  }

  .status-indicator-inline.empty {
    color: var(--text-dim);
  }
  .status-indicator-inline.empty .status-dot {
    background-color: var(--border-strong);
  }

  .key-input-container {
    display: flex;
    align-items: center;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 9px;
    overflow: hidden;
  }

  .key-input-field {
    flex: 1;
    min-width: 0;
    border: none;
    background: transparent;
    padding: 10px 12px;
    font-family: var(--font-mono);
    font-size: 12.5px;
    color: var(--text);
    outline: none;
  }

  .key-actions-group {
    display: flex;
    gap: 0;
    padding: 4px;
    border-left: 1px solid var(--border);
    background: var(--surface-alt);
  }

  .key-action-btn {
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .key-action-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .keys-grid-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
  }

  @media (max-width: 640px) {
    .keys-grid-row {
      grid-template-columns: 1fr;
    }
  }

  .form-save-actions {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    padding-top: 14px;
    border-top: 1px solid var(--border);
  }

  /* Help integrations card */
  .developer-help-card {
    background: rgba(255, 255, 255, 0.01);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
  }

  .help-card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text);
    margin-bottom: 8px;
  }

  .help-card-header .info-icon {
    color: var(--text-muted);
  }

  .help-card-desc {
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.45;
    margin-bottom: 12px;
  }

  .help-steps-list {
    font-size: 12px;
    color: var(--text-muted);
    padding-left: 18px;
    line-height: 1.6;
  }

  .help-steps-list li {
    margin-bottom: 6px;
  }

  .help-steps-list a {
    color: var(--text);
    font-weight: 500;
    text-decoration: underline;
  }

  .help-steps-list a:hover {
    color: var(--accent);
  }

  .code-highlight {
    background: var(--surface-alt);
    border: 1px solid var(--border);
    padding: 1px 5px;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text);
  }

  /* Loader */
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

  @keyframes rotate {
    to { transform: rotate(360deg); }
  }

  /* Advanced & Preferences styling */
  .advanced-workspace {
    margin-top: 15px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .db-diagnostics-row {
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 14px 16px;
  }

  .diagnostic-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 13px;
  }

  .diagnostic-label {
    color: var(--text-muted);
  }

  .diagnostic-val {
    color: var(--text);
  }

  .diagnostic-val.active {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--accent);
    font-weight: 500;
  }
  
  :global(.theme-light) .diagnostic-val.active {
    color: #3f6212;
  }

  .diagnostic-val.active .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--accent);
  }
  
  :global(.theme-light) .diagnostic-val.active .status-dot {
    background-color: #3f6212;
  }

  .danger-zone-divider {
    font-family: var(--font-mono);
    font-size: 10px;
    text-transform: uppercase;
    color: var(--rose);
    letter-spacing: 0.08em;
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 8px;
  }

  .danger-zone-divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: var(--rose-muted);
  }

  .danger-action-row {
    border: 1px solid var(--rose-muted);
    background: rgba(251, 113, 133, 0.02);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
  }

  @media (max-width: 640px) {
    .danger-action-row {
      flex-direction: column;
      align-items: start;
    }
  }

  .danger-action-details h4 {
    font-size: 14.5px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 4px;
  }

  .danger-action-details p {
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.45;
  }
</style>
