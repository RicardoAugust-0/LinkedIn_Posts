<!-- frontend/src/components/wizard/StepTopic.svelte -->
<script lang="ts">
  import { Sparkles, Check, ArrowRight } from '@lucide/svelte';
  import { postStore } from '../../lib/stores/postStore';
  import { onMount } from 'svelte';

  let loadingTopics = false;

  async function fetchSuggestedTopics() {
    loadingTopics = true;
    try {
      const res = await fetch('http://localhost:3000/api/generate/topics');
      if (res.ok) {
        const topics = await res.json();
        postStore.setRecentTopics(topics);
      } else {
        postStore.setRecentTopics([
          'Rust 1.95 e const generics', 
          'Edge runtime migrations', 
          'Liderança técnica pragmática', 
          'Burnout em times de produto'
        ]);
      }
    } catch (e) {
      console.error("Erro ao buscar tópicos sugeridos pela IA:", e);
      postStore.setRecentTopics([
        'Rust 1.95 e const generics', 
        'Edge runtime migrations', 
        'Liderança técnica pragmática', 
        'Burnout em times de produto'
      ]);
    } finally {
      loadingTopics = false;
    }
  }

  onMount(() => {
    if ($postStore.recentTopics.length === 0) {
      fetchSuggestedTopics();
    }
  });

  function selectRecentTopic(t: string) {
    postStore.setTopic(t);
  }

  const tonePresets = [
    { id: 'casual',    label: 'Casual técnico',         value: 'Casual, conversa de engenheiro. Linguagem direta, exemplos práticos, sem jargão de marketing.' },
    { id: 'checklist', label: 'Pragmático com checklist', value: 'Tom pragmático. Sempre incluir checklist no final. Foco no que funciona em produção.' },
    { id: 'story',     label: 'Storytelling de bastidores', value: 'Storytelling de bastidores. Começar com um problema real, mostrar a investigação, fechar com o aprendizado.' },
    { id: 'opinion',   label: 'Opinião forte e direta', value: 'Opinião forte e direta. Tomar lado. Defender a posição com 2 a 3 argumentos concretos.' },
    { id: 'tutorial',  label: 'Tutorial passo-a-passo', value: 'Tutorial passo-a-passo. Numerado. Cada passo com 1 a 2 frases. Trechos de código quando fizer sentido.' },
  ];

  function selectTonePreset(val: string) {
    if ($postStore.promptOverride === val) {
      postStore.setPromptOverride('');
    } else {
      postStore.setPromptOverride(val);
    }
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
        <span class="studio-optional">opcional · escolha um preset ou escreva</span>
      </div>
      <div class="tone-presets-row">
        {#each tonePresets as p}
          <button 
            type="button" 
            class="tone-tag-btn {$postStore.promptOverride === p.value ? 'selected' : ''}" 
            on:click={() => selectTonePreset(p.value)}
          >
            {p.label}
          </button>
        {/each}
      </div>
      <textarea 
        id="guidelines-input"
        class="studio-textarea" 
        style="margin-top: 8px;"
        placeholder="Ou escreva seu próprio: tom de voz, regras de escrita, frases para evitar..."
        value={$postStore.promptOverride}
        on:input={(e) => postStore.setPromptOverride(e.currentTarget.value)}
        rows={4}
        disabled={$postStore.generatingText}
      ></textarea>
    </div>

    <!-- Recent Suggested Topics -->
    <div class="studio-field">
      <div class="studio-field-header">
        <label class="studio-label" for="suggested">Tópicos sugeridos por I.A.</label>
        <button type="button" class="refresh-topics-btn" on:click={fetchSuggestedTopics} disabled={loadingTopics}>
          {#if loadingTopics}
            <span class="mini-spinner inline-spinner"></span>
          {:else}
            <span>Atualizar ↻</span>
          {/if}
        </button>
        <span class="studio-optional">opcional</span>
      </div>
      <div class="recent-topics-row">
        {#if loadingTopics && $postStore.recentTopics.length === 0}
          <div class="topics-loader">Carregando sugestões da IA...</div>
        {:else}
          {#each $postStore.recentTopics as t}
            <button class="topic-tag-btn" on:click={() => selectRecentTopic(t)}>{t}</button>
          {/each}
        {/if}
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

  .tone-presets-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 4px;
    margin-bottom: 8px;
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

  .tone-tag-btn {
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

  .tone-tag-btn:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
    color: var(--text);
  }

  .tone-tag-btn.selected {
    background: var(--accent-muted);
    border-color: rgba(163, 230, 53, 0.2);
    color: var(--accent);
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

  .refresh-topics-btn {
    background: transparent;
    border: none;
    color: var(--accent);
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    padding: 0 4px;
    margin-left: 8px;
    display: inline-flex;
    align-items: center;
    transition: color var(--transition-fast);
  }

  .refresh-topics-btn:hover {
    color: var(--accent-hover, #bef264);
  }

  .refresh-topics-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .topics-loader {
    font-size: 12px;
    color: var(--text-dim);
    padding: 5px 0;
  }

  .inline-spinner {
    width: 10px;
    height: 10px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: rotate 1s linear infinite;
    display: inline-block;
  }
</style>
