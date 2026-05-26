<!-- frontend/src/components/automation/AutomationCampaignForm.svelte -->
<script lang="ts">
  import { Zap } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let topicSeed = '';
  export let quantity = 10;
  export let cadence = 'daily';
  export let windows: string[] = ['morning'];
  export let tone = '';
  export let generating = false;

  export let cadenceOptions: any[] = [];
  export let windowOptions: any[] = [];
  export let tonePresets: any[] = [];

  function toggleWindow(id: string) {
    if (windows.includes(id)) {
      if (windows.length > 1) {
        windows = windows.filter(w => w !== id);
      }
    } else {
      windows = [...windows, id];
    }
  }

  function applyTonePreset(presetValue: string) {
    tone = presetValue;
  }
</script>

<div class="studio-card config-card">
  <div class="card-title-row">
    <div class="title-details">
      <span class="panel-eyebrow-small">Gerador de Campanha</span>
      <h2 class="section-title">Parâmetros da Automação</h2>
    </div>
  </div>

  <!-- Semente do Tópico -->
  <div class="studio-field">
    <label class="studio-label" for="topic-seed">Semente de ideias para I.A</label>
    <textarea 
      id="topic-seed"
      class="studio-input studio-textarea" 
      bind:value={topicSeed}
      placeholder="Escreva sobre o que deseja que a IA publique (ex: Engenharia de software, Rust, Kubernetes...)"
      rows="2"
    ></textarea>
    <div class="studio-hint">A IA criará variações de posts baseados neste tema principal.</div>
  </div>

  <!-- Quantidade -->
  <div class="studio-field">
    <label class="studio-label" for="quantity-slider">Quantidade de posts na esteira</label>
    <div class="quantity-row">
      <div class="quantity-value">{quantity}</div>
      <input 
        id="quantity-slider"
        type="range" 
        min="3" 
        max="30" 
        class="quantity-slider" 
        bind:value={quantity}
      />
      <span class="quantity-hint">posts</span>
    </div>
  </div>

  <!-- Cadência -->
  <div class="studio-field">
    <span class="studio-label">Cadência de publicações</span>
    <div class="cadence-list">
      {#each cadenceOptions as opt}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div 
          class="cadence-card {cadence === opt.id ? 'selected' : ''}"
          on:click={() => cadence = opt.id}
        >
          <span class="cadence-label {cadence === opt.id ? 'selected' : ''}">{opt.label}</span>
          <span class="cadence-desc">{opt.desc}</span>
        </div>
      {/each}
    </div>
  </div>

  <!-- Janela de Horário -->
  <div class="studio-field">
    <span class="studio-label">Janelas de horário preferenciais</span>
    <div class="window-chips">
      {#each windowOptions as opt}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <button 
          type="button"
          class="window-chip {windows.includes(opt.id) ? 'active' : ''}"
          on:click={() => toggleWindow(opt.id)}
        >
          <span class="window-label">{opt.label}</span>
          <span class="window-range">{opt.range}</span>
        </button>
      {/each}
    </div>
    <div class="studio-hint">Multi-seleção. A esteira distribuirá as publicações nestas janelas.</div>
  </div>

  <!-- Tom de Voz -->
  <div class="studio-field">
    <label class="studio-label" for="tone-textarea">Tom de voz & Diretrizes da I.A</label>
    
    <!-- Botões de Tom de Voz rápidos (Presets) -->
    <div class="presets-row">
      {#each tonePresets as preset}
        <button 
          type="button" 
          class="preset-btn {tone === preset.value ? 'active' : ''}" 
          on:click={() => applyTonePreset(preset.value)}
        >
          {preset.label}
        </button>
      {/each}
    </div>

    <textarea 
      id="tone-textarea"
      class="studio-input studio-textarea" 
      bind:value={tone}
      placeholder="Escreva diretrizes adicionais para o estilo de escrita da IA. Deixe em branco para usar o tom padrão LinkedMaker (limpo, pragmático e direto)."
      rows="3"
    ></textarea>
  </div>

  <!-- Ação de Gerar -->
  <button 
    class="studio-btn studio-btn-accent generate-btn" 
    on:click={() => dispatch('generate')}
    disabled={generating || !topicSeed.trim()}
  >
    {#if generating}
      <span class="spinner"></span>
      <span>Gerando publicações...</span>
    {:else}
      <Zap size={14} class="zap-icon" />
      <span>Gerar Automação Completa</span>
    {/if}
  </button>
</div>

<style>
  .config-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 20px;
    box-shadow: var(--shadow-card);
  }

  .panel-eyebrow-small {
    font-family: var(--font-mono);
    font-size: 10.5px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--text-dim);
  }

  .section-title {
    font-size: 17px;
    margin-top: 4px;
    color: var(--text);
    font-weight: 600;
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

  .studio-input {
    width: 100%;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 10px 12px;
    color: var(--text);
    font-family: inherit;
    font-size: var(--type-body-size);
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .studio-input:focus {
    border-color: var(--border-strong);
  }

  .studio-textarea {
    resize: vertical;
    line-height: 1.45;
  }

  .studio-hint {
    font-size: 11.5px;
    color: var(--text-dim);
    margin-top: 2px;
  }

  /* Slider */
  .quantity-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .quantity-value {
    font-size: 36px;
    font-weight: 500;
    letter-spacing: -0.03em;
    color: var(--accent);
    font-variant-numeric: tabular-nums;
    min-width: 64px;
    line-height: 1;
  }

  .quantity-slider {
    flex: 1;
    appearance: none;
    -webkit-appearance: none;
    height: 4px;
    background: var(--surface-alt);
    border-radius: 2px;
    outline: none;
    accent-color: var(--accent);
  }

  .quantity-hint {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
  }

  /* Cadence */
  .cadence-list {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .cadence-card {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px 14px;
    cursor: pointer;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 10px;
    transition: all var(--transition-fast);
  }

  .cadence-card:hover {
    border-color: var(--border-strong);
  }

  .cadence-card.selected {
    background: var(--accent-muted);
    border-color: var(--accent);
  }

  .cadence-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .cadence-label.selected {
    color: var(--accent);
  }

  .cadence-desc {
    font-size: 11.5px;
    color: var(--text-muted);
  }

  /* Windows Chips */
  .window-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .window-chip {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 99px;
    cursor: pointer;
    font-size: 12px;
    font-family: inherit;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    color: var(--text-muted);
    transition: all var(--transition-fast);
  }

  .window-chip:hover {
    border-color: var(--border-strong);
    color: var(--text);
  }

  .window-chip.active {
    background: var(--accent-muted);
    border-color: var(--accent);
    color: var(--accent);
  }

  .window-range {
    font-family: var(--font-mono);
    font-size: 10px;
    opacity: 0.65;
  }

  /* Tone Presets */
  .presets-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 8px;
  }

  .preset-btn {
    border: 1px solid var(--border);
    background: var(--surface-alt);
    color: var(--text-muted);
    padding: 5px 9px;
    border-radius: 6px;
    font-size: 11.5px;
    cursor: pointer;
    font-family: inherit;
    transition: all var(--transition-fast);
  }

  .preset-btn:hover {
    border-color: var(--border-strong);
    color: var(--text);
  }

  .preset-btn.active {
    background: var(--accent-muted);
    border-color: var(--accent);
    color: var(--accent);
  }

  /* Button Actions */
  .studio-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 10px 16px;
    border-radius: var(--radius-lg);
    font-family: inherit;
    font-size: 13.5px;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all var(--transition-fast);
    white-space: nowrap;
  }

  .studio-btn-accent {
    background: var(--accent);
    color: var(--accent-ink);
  }

  .studio-btn-accent:hover:not(:disabled) {
    background: var(--accent);
    opacity: 0.9;
  }

  .studio-btn-accent:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .studio-btn-accent :global(.zap-icon) {
    color: var(--accent-ink);
  }

  .generate-btn {
    width: 100%;
    margin-top: 10px;
  }

  /* Spinner */
  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(0, 0, 0, 0.1);
    border-top-color: var(--accent-ink);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
