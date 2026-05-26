<!-- frontend/src/pages/CreatePost.svelte -->
<script lang="ts">
  import { Check, AlertCircle } from '@lucide/svelte';
  import { postStore } from '../lib/stores/postStore';
  import StepTopic from '../components/wizard/StepTopic.svelte';
  import StepMedia from '../components/wizard/StepMedia.svelte';
  import StepSchedule from '../components/wizard/StepSchedule.svelte';
  import { createEventDispatcher, onMount } from 'svelte';

  export let theme: 'dark' | 'light' = 'dark';

  const dispatch = createEventDispatcher();

  onMount(() => {
    postStore.reset();
  });

  function handleSaved() {
    dispatch('navigate', 'dashboard');
  }

  function handleCancel() {
    postStore.reset();
    dispatch('navigate', 'dashboard');
  }
</script>

<div class="studio-page-header">
  <div class="studio-page-header-info">
    <div class="studio-eyebrow">Novo post</div>
    <h1>
      Criar com IA <span class="header-divider">—</span> <span class="header-step">{['Tópico & Texto','Mídia do post','Revisão & Agendamento'][$postStore.step-1]}</span>
    </h1>
  </div>
  
  <div class="header-actions">
    <button class="studio-btn studio-btn-ghost" on:click={handleCancel}>Cancelar</button>
    <button class="studio-btn studio-btn-secondary" on:click={() => postStore.savePost(false, handleSaved)} disabled={!$postStore.postTitle}>Salvar rascunho</button>
  </div>
</div>

<div class="studio-page-body">
  <!-- Steps Indicator Progress Bar -->
  <div class="wizard-progress-bar">
    {#each [
      { n: 1, title: 'Tópico & Texto', desc: 'O assunto e o tom' },
      { n: 2, title: 'Mídia', desc: 'Imagem que acompanha' },
      { n: 3, title: 'Revisão', desc: 'Quando publicar' }
    ] as s, i}
      <div class="progress-step-item {$postStore.step === s.n ? 'active' : ''} {$postStore.step > s.n ? 'completed' : ''}">
        <div class="step-circle">
          {#if $postStore.step > s.n}
            <Check size={14} />
          {:else}
            {String(s.n).padStart(2, '0')}
          {/if}
        </div>
        <div class="step-label-group">
          <span class="step-label-title">{s.title}</span>
          <span class="step-label-desc">{s.desc}</span>
        </div>
      </div>
      {#if i < 2}
        <div class="progress-line {$postStore.step > s.n ? 'completed' : ''}"></div>
      {/if}
    {/each}
  </div>

  <!-- Messages Feedback -->
  {#if $postStore.errorMsg}
    <div class="studio-toast studio-toast-error static-toast">
      <AlertCircle size={16} />
      <span>{$postStore.errorMsg}</span>
    </div>
  {/if}
  {#if $postStore.successMsg}
    <div class="studio-toast studio-toast-success static-toast">
      <Check size={16} />
      <span>{$postStore.successMsg}</span>
    </div>
  {/if}

  <!-- Render current Step -->
  {#if $postStore.step === 1}
    <StepTopic />
  {:else if $postStore.step === 2}
    <StepMedia />
  {:else if $postStore.step === 3}
    <StepSchedule on:saved={handleSaved} />
  {/if}
</div>

<style>
  /* Wizard Progress Steps Indicator */
  .wizard-progress-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0;
    padding: 10px 16px 20px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 8px;
  }

  .progress-step-item {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .step-circle {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    color: var(--text-dim);
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 600;
    box-sizing: content-box;
    transition: all var(--transition-fast);
  }

  .progress-step-item.active .step-circle {
    background: var(--surface);
    border: 2px solid var(--text);
    color: var(--text);
  }

  :global(.theme-dark) .progress-step-item.active .step-circle {
    border-color: var(--accent);
    color: var(--text);
  }

  .progress-step-item.completed .step-circle {
    background: var(--text);
    color: var(--bg-app);
    border-color: var(--text);
  }

  :global(.theme-dark) .progress-step-item.completed .step-circle {
    background: var(--accent);
    color: var(--accent-ink);
    border-color: var(--accent);
  }

  .step-label-group {
    display: flex;
    flex-direction: column;
    line-height: 1.2;
  }

  .step-label-title {
    font-size: 13.5px;
    font-weight: 500;
    color: var(--text-muted);
  }

  .progress-step-item.active .step-label-title,
  .progress-step-item.completed .step-label-title {
    color: var(--text);
  }

  .step-label-desc {
    font-size: 11.5px;
    color: var(--text-dim);
    margin-top: 3px;
  }

  .progress-line {
    flex: 1;
    height: 1px;
    background: var(--border);
    margin: 0 18px;
    transition: background-color var(--transition-normal);
  }

  .progress-line.completed {
    background: var(--text);
  }

  :global(.theme-dark) .progress-line.completed {
    background: var(--accent);
  }

  /* Static feedback */
  .static-toast {
    position: relative;
    max-width: 100%;
    box-shadow: none;
    margin-bottom: 12px;
    top: auto;
    right: auto;
    width: 100%;
  }

  .header-divider {
    color: var(--text-dim);
    font-weight: 400;
  }

  .header-step {
    color: var(--text-muted);
    font-weight: 400;
  }

  .header-actions {
    display: flex;
    gap: 8px;
  }

  :global(.mini-spinner) {
    width: 10px;
    height: 10px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: rotate 1s linear infinite;
    display: inline-block;
  }

  @keyframes rotate {
    to { transform: rotate(360deg); }
  }
</style>
