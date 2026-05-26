<!-- frontend/src/components/settings/SettingsAIKeys.svelte -->
<script lang="ts">
  import { Eye, EyeOff, Copy } from '@lucide/svelte';

  export let geminiKey = '';
  export let pexelsKey = '';

  let showGemini = false;
  let showPexels = false;

  function copyKeyToClipboard(text: string) {
    if (!text) return;
    navigator.clipboard.writeText(text);
  }
</script>

<!-- Credentials forms card -->
<div class="keys-wrapper">
  <div class="card-title-row header-only">
    <div class="title-details">
      <h2>Chaves de IA</h2>
      <p class="card-desc">Google AI Studio. Necessário para geração de textos (Gemini) e imagens (Imagen 4).</p>
    </div>
  </div>

  <!-- Gemini Key input -->
  <div class="studio-field">
    <div class="studio-field-header">
      <label class="studio-label" for="gemini-key">
        Gemini API Key
        <a href="https://aistudio.google.com/app/apikey" target="_blank" rel="noopener noreferrer" class="external-link-btn" title="Obter chave no Google AI Studio">Obter chave ↗</a>
      </label>
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
        <button type="button" class="key-action-btn" on:click={() => showGemini = !showGemini} title="Alternar visibilidade">
          {#if showGemini}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
        </button>
        <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(geminiKey)} title="Copiar chave">
          <Copy size={13} />
        </button>
      </div>
    </div>
    <div class="studio-hint">Modelo utilizado: gemini-2.5-pro / Imagen 4.</div>
  </div>

  <!-- Imagen 4 Key input (shares gemini key indicator) -->
  <div class="studio-field">
    <div class="studio-field-header">
      <label class="studio-label" for="imagen-key">Imagen 4 API Key</label>
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
        <button type="button" class="key-action-btn" on:click={() => showGemini = !showGemini} title="Alternar visibilidade">
          {#if showGemini}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
        </button>
      </div>
    </div>
    <div class="studio-hint">Para geração de ilustrações no Wizard do post.</div>
  </div>

  <!-- Pexels Search Key -->
  <div id="search-section" class="card-title-row header-only inner-divider">
    <div class="title-details">
      <h2>Busca de imagens</h2>
      <p class="card-desc">API do Pexels para a aba 'Buscar Imagem' no criador de posts.</p>
    </div>
  </div>

  <div class="studio-field">
    <div class="studio-field-header">
      <label class="studio-label" for="pexels-key">
        Pexels API Key
        <a href="https://www.pexels.com/api/" target="_blank" rel="noopener noreferrer" class="external-link-btn" title="Obter chave no Pexels">Obter chave ↗</a>
      </label>
      <span class="status-indicator-inline {pexelsKey ? 'valid' : 'empty'}">
        <span class="status-dot"></span>
        {pexelsKey ? 'Verificada' : 'Ausente'}
      </span>
    </div>
    <div class="key-input-container">
      <input 
        id="pexels-key"
        type={showPexels ? "text" : "password"} 
        class="studio-input key-input-field" 
        bind:value={pexelsKey}
        placeholder="Insira sua Pexels API Key"
      />
      <div class="key-actions-group">
        <button type="button" class="key-action-btn" on:click={() => showPexels = !showPexels} title="Alternar visibilidade">
          {#if showPexels}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
        </button>
        <button type="button" class="key-action-btn" on:click={() => copyKeyToClipboard(pexelsKey)} title="Copiar chave">
          <Copy size={13} />
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .keys-wrapper {
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

  .studio-field-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
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

  .studio-hint {
    font-size: 11.5px;
    color: var(--text-dim);
    margin-top: 2px;
  }


</style>
