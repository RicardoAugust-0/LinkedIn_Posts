<!-- frontend/src/pages/Settings.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    Save, ExternalLink, AlertCircle, CheckCircle2, Check
  } from '@lucide/svelte';
  import { API_URL } from '../lib/api';
  import SettingsLinkedInProfile from '../components/settings/SettingsLinkedInProfile.svelte';
  import SettingsAIKeys from '../components/settings/SettingsAIKeys.svelte';
  import SettingsLinkedInCredentials from '../components/settings/SettingsLinkedInCredentials.svelte';
  import SettingsAuthorContext from '../components/settings/SettingsAuthorContext.svelte';
  import SettingsPreferences from '../components/settings/SettingsPreferences.svelte';
  import SettingsAdvanced from '../components/settings/SettingsAdvanced.svelte';
  import SettingsDiagnostics from '../components/settings/SettingsDiagnostics.svelte';
  import SettingsHelp from '../components/settings/SettingsHelp.svelte';

  // Configurações
  let geminiKey = '';
  let pexelsKey = '';
  let linkedinClientId = '';
  let linkedinClientSecret = '';
  let userContext = '';

  let loading = true;
  let saving = false;
  let testingAll = false;
  
  // Toasts
  let showSaveSuccess = false;
  let authSuccessMessage = false;
  let authErrorMessage = false;
  let showSuccessToast = false;
  let successToastMsg = "";
  let showErrorToast = false;
  let errorToastMsg = "";

  function triggerSuccessToast(msg: string) {
    successToastMsg = msg;
    showSuccessToast = true;
    setTimeout(() => showSuccessToast = false, 5000);
  }

  function triggerErrorToast(msg: string) {
    errorToastMsg = msg;
    showErrorToast = true;
    setTimeout(() => showErrorToast = false, 6000);
  }

  let testResults: any = null;

  async function testAllConnections() {
    testingAll = true;
    testResults = null;
    try {
      const res = await fetch(`${API_URL}/api/settings/test-all`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          gemini_key: geminiKey || null,
          pexels_key: pexelsKey || null,
          linkedin_client_id: linkedinClientId || null,
          linkedin_client_secret: linkedinClientSecret || null
        })
      });

      if (res.ok) {
        testResults = await res.json();
        const allSuccess = testResults.gemini?.success && testResults.pexels?.success && testResults.linkedin?.success;
        if (allSuccess) {
          triggerSuccessToast("Todas as conexões foram testadas e estão funcionando perfeitamente!");
        } else {
          triggerErrorToast("Alguns testes de conexão falharam. Veja os detalhes abaixo.");
        }
      } else {
        triggerErrorToast("Erro ao conectar ao servidor de testes.");
      }
    } catch (e) {
      console.error(e);
      triggerErrorToast("Erro de rede ao testar conexões.");
    } finally {
      testingAll = false;
    }
  }
  
  // LinkedIn Auth Status
  let isAuthenticated = false;
  let isSimulated = false;
  let expiresAt: string | null = null;
  let disconnecting = false;

  // Preferências
  let prefAiModel = 'gemini-2.5-flash';
  let prefCharLimit = 3000;

  // Avançado
  let activeAnchor = 'linkedin';
  let clearingDb = false;

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
      const res = await fetch(`${API_URL}/api/settings`);
      if (res.ok) {
        const data = await res.json();
        geminiKey = data.gemini_key || '';
        pexelsKey = data.pexels_key || '';
        linkedinClientId = data.linkedin_client_id || '';
        linkedinClientSecret = data.linkedin_client_secret || '';
        userContext = data.user_context || '';
      }
    } catch (e) {
      console.error("Erro ao carregar configurações", e);
    } finally {
      loading = false;
    }
  }

  async function checkAuthStatus() {
    try {
      const res = await fetch(`${API_URL}/api/auth/linkedin/status`);
      if (res.ok) {
        const data = await res.json();
        isAuthenticated = data.authenticated;
        isSimulated = data.simulated || false;
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
      const res = await fetch(`${API_URL}/api/settings`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          gemini_key: geminiKey || null,
          pexels_key: pexelsKey || null,
          linkedin_client_id: linkedinClientId || null,
          linkedin_client_secret: linkedinClientSecret || null,
          user_context: userContext || null
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
    const redirectUrl = encodeURIComponent(window.location.origin);
    window.location.href = `${API_URL}/api/auth/linkedin?redirect_url=${redirectUrl}`;
  }

  async function disconnectLinkedIn() {
    if (!confirm("Deseja realmente desconectar sua conta do LinkedIn? Suas publicações voltarão a rodar em Modo de Simulação.")) return;
    
    disconnecting = true;
    try {
      const res = await fetch(`${API_URL}/api/auth/linkedin/disconnect`, {
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
      const res = await fetch(`${API_URL}/api/posts`, {
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
        <SettingsLinkedInProfile 
          {isAuthenticated} 
          {isSimulated}
          {expiresAt} 
          {disconnecting} 
          on:connect={startLinkedInAuth} 
          on:disconnect={disconnectLinkedIn} 
        />

        <!-- Credentials forms card -->
        <section id="keys-section" class="studio-card settings-card">
          <form on:submit|preventDefault={saveSettings} class="settings-form">
            <SettingsAIKeys 
              bind:geminiKey 
              bind:pexelsKey 
            />

            <SettingsLinkedInCredentials 
              bind:linkedinClientId 
              bind:linkedinClientSecret 
            />

            <SettingsAuthorContext 
              bind:userContext 
            />

            <!-- Form Save Actions -->
            <div class="form-save-actions">
              <button type="submit" class="studio-btn studio-btn-accent" disabled={saving}>
                <Save size={12} />
                <span>{saving ? 'Salvando...' : 'Salvar Configurações'}</span>
              </button>

              <button type="button" class="studio-btn studio-btn-secondary" on:click={testAllConnections} disabled={testingAll || saving}>
                {#if testingAll}
                  <span class="mini-spinner"></span>
                  <span>Testando conexões...</span>
                {:else}
                  <Check size={12} />
                  <span>Testar todas as conexões</span>
                {/if}
              </button>
            </div>
          </form>

          <!-- Diagnóstico das Conexões Panel -->
          <SettingsDiagnostics {testResults} on:close={() => testResults = null} />
        </section>

        <!-- Preferências Card -->
        <SettingsPreferences 
          bind:prefAiModel 
          bind:prefCharLimit 
          on:change={savePreferences} 
        />

        <!-- Configurações Avançadas Card -->
        <SettingsAdvanced 
          {clearingDb} 
          on:clearDb={clearDatabase} 
        />

        <!-- Help Documentation Card -->
        <SettingsHelp />
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

  .settings-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 12px;
  }

  .form-save-actions {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 12px;
    flex-wrap: wrap;
    padding-top: 14px;
    border-top: 1px solid var(--border);
  }

  /* Mini spinner */
  .mini-spinner {
    width: 12px;
    height: 12px;
    border: 1.5px solid var(--border-strong);
    border-top-color: var(--text);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
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
</style>
