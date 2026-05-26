<!-- frontend/src/components/settings/SettingsLinkedInCredentials.svelte -->
<script lang="ts">
  import { Eye, EyeOff, Copy } from '@lucide/svelte';

  export let linkedinClientId = '';
  export let linkedinClientSecret = '';

  let showLinkedIn = false;

  function copyKeyToClipboard(text: string) {
    if (!text) return;
    navigator.clipboard.writeText(text);
  }
</script>

<div class="credentials-wrapper">
  <div class="card-title-row header-only inner-divider">
    <div class="title-details">
      <h2>Credenciais LinkedIn Developer</h2>
      <p class="card-desc">Necessário para publicar de verdade na sua conta. Caso contrário, roda em Simulação.</p>
    </div>
  </div>

  <div class="keys-grid-row">
    <div class="studio-field">
      <label class="studio-label" for="linkedin-client">
        LinkedIn Client ID
        <a href="https://www.linkedin.com/developers/apps" target="_blank" rel="noopener noreferrer" class="external-link-btn" title="Criar/Gerenciar apps no LinkedIn Developers">Portal Dev ↗</a>
      </label>
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
          <button type="button" class="key-action-btn" on:click={() => showLinkedIn = !showLinkedIn} title="Alternar visibilidade">
            {#if showLinkedIn}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
          </button>
          <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(linkedinClientSecret)} title="Copiar chave">
            <Copy size={13} />
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .credentials-wrapper {
    display: flex;
    flex-direction: column;
    gap: 14px;
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

  .card-title-row.inner-divider {
    border-top: 1px solid var(--border);
    margin-top: 20px;
    padding-top: 20px;
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

  .studio-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .studio-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .external-link-btn {
    font-size: 11px;
    color: var(--text-dim);
    text-decoration: none;
    transition: color var(--transition-fast);
  }

  .external-link-btn:hover {
    color: var(--accent);
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
</style>
