<!-- frontend/src/components/settings/SettingsLinkedInProfile.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let isAuthenticated = false;
  export let isSimulated = false;
  export let expiresAt: string | null = null;
  export let disconnecting = false;
</script>

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
          Conectado ao LinkedIn oficial
        {:else}
          LinkedIn rodando em Modo de Simulação (Mock)
        {/if}
      </div>
      
      <div class="profile-meta-row">
        <span>token: <span class="meta-highlight">{isAuthenticated ? '•••••••••fa92e' : (isSimulated ? 'mock_token' : 'desconectado')}</span></span>
        <span>expira: <span class="meta-highlight">{expiresAt || (isSimulated ? 'Simulação sem expiração' : '—')}</span></span>
        <span>escopo: <span class="meta-highlight">w_member_social</span></span>
      </div>
    </div>

    <div class="profile-actions-wrapper">
      <button type="button" class="studio-btn studio-btn-secondary" on:click={() => dispatch('connect')}>
        {isAuthenticated ? 'Renovar token' : 'Conectar Conta'}
      </button>
      {#if isAuthenticated}
        <button type="button" class="studio-btn studio-btn-danger" on:click={() => dispatch('disconnect')} disabled={disconnecting}>
          Desconectar
        </button>
      {/if}
    </div>
  </div>
</section>

<style>
  .settings-card {
    display: flex;
    flex-direction: column;
  }

  .card-title-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
    padding-bottom: 14px;
    border-bottom: 1px solid var(--border);
  }

  .title-details h2 {
    font-size: 14.5px;
    font-weight: 600;
    color: var(--text);
  }

  .card-desc {
    font-size: 11.5px;
    color: var(--text-muted);
    line-height: 1.45;
    margin-top: 3px;
  }

  .status-indicator-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    border-radius: var(--radius-pill);
    font-size: 10px;
    font-family: var(--font-mono);
    text-transform: uppercase;
    font-weight: 600;
  }

  .status-indicator-badge.connected {
    background: rgba(163, 230, 53, 0.08);
    color: var(--accent);
  }
  .status-indicator-badge.connected .status-dot {
    background: var(--accent);
  }

  .status-indicator-badge.simulated {
    background: rgba(251, 191, 36, 0.08);
    color: var(--amber);
  }
  .status-indicator-badge.simulated .status-dot {
    background: var(--amber);
  }

  .status-dot {
    width: 4px;
    height: 4px;
    border-radius: 50%;
  }

  /* Profile row */
  .connection-profile-row {
    display: flex;
    align-items: center;
    gap: 16px;
    padding-top: 15px;
  }

  @media (max-width: 640px) {
    .connection-profile-row {
      flex-direction: column;
      align-items: flex-start;
      gap: 14px;
    }
    .profile-actions-wrapper {
      width: 100%;
    }
  }

  .profile-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--avatar-a), var(--avatar-b));
    color: #ffffff;
    font-weight: 600;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
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
</style>
