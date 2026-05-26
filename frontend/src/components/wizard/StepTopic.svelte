<!-- frontend/src/components/wizard/StepTopic.svelte -->
<script lang="ts">
  import { Sparkles, Check, ArrowRight } from '@lucide/svelte';
  import { postStore } from '../../lib/stores/postStore';

  const recentTopics = [
    'Rust 1.95 e const generics', 
    'Edge runtime migrations', 
    'Liderança técnica pragmática', 
    'Burnout em times de produto'
  ];

  function selectRecentTopic(t: string) {
    postStore.setTopic(t);
  }

  function handleClearAll() {
    postStore.setTopic('');
    postStore.setPromptOverride('');
  }
</script>

<div class="step-content-layout {$postStore.postContent ? 'split' : 'centered'}">
  <!-- Input Panel -->
  <div class="studio-card form-panel">
    <div class="studio-field">
      <div class="studio-field-header">
        <label class="studio-label" for="topic-input">Sobre o que será sua publicação?</label>
      </div>
      <input 
        id="topic-input"
        type="text" 
        class="studio-input" 
        placeholder="Ex: Rust 1.95 release notes — destacar mudanças no borrow checker e const generics" 
        value={$postStore.topic}
        on:input={(e) => postStore.setTopic(e.currentTarget.value)}
        disabled={$postStore.generatingText}
      />
      <div class="studio-hint">Pode ser uma frase, link, ou apenas um tema. Quanto mais específico, melhor a IA performa.</div>
    </div>

    <div class="studio-field">
      <div class="studio-field-header">
        <label class="studio-label" for="guidelines-input">Diretrizes & tom de voz</label>
        <span class="studio-optional">opcional</span>
      </div>
      <textarea 
        id="guidelines-input"
        class="studio-textarea" 
        placeholder="Ex: Tom: casual, voz de engenheiro pragmático. Evitar jargão de marketing. Usar checklist."
        value={$postStore.promptOverride}
        on:input={(e) => postStore.setPromptOverride(e.currentTarget.value)}
        rows={4}
        disabled={$postStore.generatingText}
      ></textarea>
    </div>

    <!-- Recent Suggested Topics -->
    <div class="studio-field">
      <div class="studio-field-header">
        <label class="studio-label" for="suggested">Tópicos recentes</label>
        <span class="studio-optional">opcional</span>
      </div>
      <div class="recent-topics-row">
        {#each recentTopics as t}
          <button class="topic-tag-btn" on:click={() => selectRecentTopic(t)}>{t}</button>
        {/each}
      </div>
    </div>

    <div class="form-actions">
      <button 
        class="studio-btn studio-btn-accent" 
        on:click={() => postStore.generateText()}
        disabled={$postStore.generatingText || !$postStore.topic.trim()}
      >
        {#if $postStore.generatingText}
          <span class="mini-spinner"></span>
          <span>Gerando com IA...</span>
        {:else}
          <Sparkles size={12} />
          <span>Gerar texto com IA</span>
        {/if}
      </button>
      <button class="studio-btn studio-btn-ghost" on:click={handleClearAll}>Limpar tudo</button>
      <div class="flex-spacer"></div>
      <div class="token-info-badge">
        ~280 tokens · Gemini 2.5 Pro
      </div>
    </div>
  </div>

  <!-- Preview Column (Only if generated text exists) -->
  {#if $postStore.postContent}
    <div class="ai-preview-column">
      <div class="ai-preview-header">
        <div class="ai-badge">
          <Sparkles size={10} />
          <span>Gerado por IA</span>
        </div>
        <span class="ai-time">há poucos segundos</span>
        <div class="flex-spacer"></div>
        <button class="studio-btn studio-btn-ghost compact-btn" on:click={() => postStore.generateText()} disabled={$postStore.generatingText}>
          <Sparkles size={10} />
          <span>Regenerar</span>
        </button>
      </div>

      <div class="ai-preview-body">
        <div class="preview-label">Título interno</div>
        <div class="preview-title">{$postStore.postTitle}</div>
        
        <div class="preview-label">Corpo</div>
        <div class="preview-text-scroll">{$postStore.postContent}</div>
      </div>

      <div class="ai-preview-footer">
        <span class="ready-check">
          <Check size={11} /> Texto pronto para a próxima etapa
        </span>
        <button class="studio-btn studio-btn-primary" on:click={() => postStore.setStep(2)}>
          <span>Continuar para mídia</span>
          <ArrowRight size={12} />
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Local Scoped styles preserving wizard structure */
  .step-content-layout {
    min-height: 0;
    flex: 1;
  }

  .step-content-layout.centered {
    display: flex;
    justify-content: center;
    align-items: flex-start;
  }

  .step-content-layout.centered .form-panel {
    max-width: 680px;
    width: 100%;
  }

  .step-content-layout.split {
    display: grid;
    grid-template-columns: 1.1fr 1fr;
    gap: 24px;
  }

  @media (max-width: 1024px) {
    .step-content-layout.split {
      grid-template-columns: 1fr;
    }
  }

  .recent-topics-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 4px;
  }

  .topic-tag-btn {
    font-size: 12px;
    color: var(--text-muted);
    padding: 5px 10px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 99px;
    cursor: pointer;
    font-family: inherit;
    transition: background-color var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
  }

  .topic-tag-btn:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
    color: var(--text);
  }

  .token-info-badge {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    align-self: center;
  }

  /* AI Preview Column (Step 1 Right) */
  .ai-preview-column {
    background: var(--surface-alt);
    border: 1px dashed var(--border);
    border-radius: 14px;
    padding: 24px;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-height: calc(100vh - 310px);
    overflow: hidden;
  }

  .ai-preview-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .ai-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 9px;
    border-radius: 99px;
    background: var(--accent-muted);
    color: var(--accent);
    font-size: 11px;
    font-weight: 500;
    border: 1px solid rgba(163, 230, 53, 0.2);
  }

  :global(.theme-light) .ai-badge {
    background: var(--bg-inset);
    color: var(--text-muted);
    border-color: var(--border);
  }

  .ai-time {
    font-size: 11px;
    color: var(--text-dim);
    font-family: var(--font-mono);
  }

  .compact-btn {
    padding: 5px 8px;
    font-size: 11.5px;
  }

  .ai-preview-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .preview-label {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    letter-spacing: 0.12em;
    text-transform: uppercase;
    margin-bottom: 6px;
  }

  .preview-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: -0.02em;
    margin-bottom: 18px;
  }

  .preview-text-scroll {
    flex: 1;
    font-size: 13.5px;
    color: var(--text);
    line-height: 1.6;
    white-space: pre-wrap;
    overflow-y: auto;
    padding-right: 4px;
  }

  .ai-preview-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11.5px;
    color: var(--text-muted);
    margin-top: auto;
    padding-top: 10px;
    border-top: 1px solid var(--border);
  }

  .ready-check {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--accent);
  }

  :global(.theme-light) .ready-check {
    color: var(--text-muted);
  }

  .form-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }
</style>
