<!-- frontend/src/components/settings/SettingsAdvanced.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let clearingDb = false;
</script>

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
      <button type="button" class="studio-btn studio-btn-danger" on:click={() => dispatch('clearDb')} disabled={clearingDb}>
        {#if clearingDb}
          <span>Limpando...</span>
        {:else}
          <span>Excluir todos os posts</span>
        {/if}
      </button>
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

  .card-title-row.header-only {
    border-bottom: none;
    padding-bottom: 0;
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

  .advanced-workspace {
    margin-top: 15px;
  }

  .db-diagnostics-row {
    padding: 12px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 9px;
  }

  .diagnostic-item {
    display: flex;
    justify-content: space-between;
    font-size: 12.5px;
  }

  .diagnostic-label {
    color: var(--text-muted);
  }

  .diagnostic-val {
    color: var(--text);
  }

  .diagnostic-val.active {
    color: var(--accent);
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
  }

  .status-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background-color: var(--accent);
  }

  .font-mono {
    font-family: var(--font-mono);
    font-size: 11.5px;
  }

  .danger-zone-divider {
    margin-top: 24px;
    margin-bottom: 14px;
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--rose);
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .danger-zone-divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: rgba(251, 113, 133, 0.15);
  }

  .danger-action-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 16px;
    background: rgba(251, 113, 133, 0.02);
    border: 1px dashed rgba(251, 113, 133, 0.2);
    border-radius: 9px;
  }

  @media (max-width: 640px) {
    .danger-action-row {
      flex-direction: column;
      align-items: stretch;
      text-align: left;
    }
  }

  .danger-action-details h4 {
    font-size: 13.5px;
    font-weight: 600;
    color: var(--text);
  }

  .danger-action-details p {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.45;
    margin-top: 4px;
    max-width: 440px;
  }
</style>
