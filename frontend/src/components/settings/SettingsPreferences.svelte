<!-- frontend/src/components/settings/SettingsPreferences.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let prefAiModel = 'gemini-2.5-flash';
  export let prefCharLimit = 3000;
</script>

<section id="preferences-section" class="studio-card settings-card">
  <div class="card-title-row header-only">
    <div class="title-details">
      <h2>Preferências do System</h2>
      <p class="card-desc">Personalize o comportamento do gerador de posts e do layout.</p>
    </div>
  </div>

  <div class="settings-form-row">
    <div class="keys-grid-row">
      <div class="studio-field">
        <label class="studio-label" for="pref-ai-model">Modelo de IA Padrão</label>
        <select id="pref-ai-model" class="studio-select" bind:value={prefAiModel} on:change={() => dispatch('change')}>
          <option value="gemini-2.5-flash">Gemini 2.5 Flash (Rápido e Conciso)</option>
          <option value="gemini-2.5-pro">Gemini 2.5 Pro (Criativo e Detalhado)</option>
        </select>
      </div>

      <div class="studio-field">
        <label class="studio-label" for="pref-char-limit">Aviso de Limite de Caracteres</label>
        <input 
          id="pref-char-limit"
          type="number" 
          class="studio-input" 
          bind:value={prefCharLimit} 
          on:input={() => dispatch('change')}
          min="500" 
          max="10000"
        />
      </div>
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

  .settings-form-row {
    margin-top: 15px;
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

  .studio-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .studio-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .studio-select {
    width: 100%;
    height: 38px;
    padding: 0 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    outline: none;
    font-size: 13.5px;
  }

  .studio-select:focus {
    border-color: var(--border-strong);
  }
</style>
