<!-- frontend/src/components/automation/AutomationActiveBanner.svelte -->
<script lang="ts">
  import { Play, Pause, Trash2 } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let campaignActive = false;
  export let campaignName = '';
  export let campaignNextIn = '';
  export let queue: any[] = [];

  $: total = queue.length;
  $: published = queue.filter(p => p.status === 'published').length;
  $: progressPercentage = total > 0 ? (published / total) * 100 : 0;
</script>

<div class="active-banner {campaignActive ? 'active' : ''}">
  <div class="banner-left">
    <div class="pulse-dot {campaignActive ? 'active' : ''}"></div>
    <div class="banner-info">
      <div class="banner-title">{campaignActive ? campaignName : 'Automação Pausada'}</div>
      <div class="banner-meta">
        {#if campaignActive}
          <span>próximo envio em {campaignNextIn}</span>
          <span>•</span>
          <span>progresso: {published}/{total} posts</span>
        {:else}
          <span>Campanha inativa · Configure e gere posts abaixo</span>
        {/if}
      </div>
    </div>
  </div>

  <div class="banner-actions">
    {#if total > 0}
      <div class="progress-bar" title="Progresso da Campanha">
        <div class="progress-bar-fill" style="width: {progressPercentage}%"></div>
      </div>
      
      <button 
        class="studio-btn {campaignActive ? 'studio-btn-secondary' : 'studio-btn-accent'}" 
        on:click={() => dispatch('toggleActive')}
      >
        {#if campaignActive}
          <Pause size={12} />
          <span>Pausar</span>
        {:else}
          <Play size={12} />
          <span>Retomar</span>
        {/if}
      </button>
      
      <button class="studio-btn studio-btn-danger" on:click={() => dispatch('cancel')}>
        <Trash2 size={12} />
        <span>Cancelar</span>
      </button>
    {/if}
  </div>
</div>

<style>
  .active-banner {
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    gap: 20px;
    padding: 14px 20px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 12px;
    transition: background-color var(--transition-normal), border-color var(--transition-normal);
  }

  .active-banner.active {
    background: var(--accent-muted);
    border: 1px solid rgba(163, 230, 53, 0.20);
  }

  @media (max-width: 768px) {
    .active-banner {
      grid-template-columns: 1fr;
    }
    .banner-actions {
      width: 100%;
      justify-content: space-between;
    }
  }

  .banner-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .pulse-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--amber);
    box-shadow: 0 0 0 4px rgba(251, 191, 36, 0.12);
  }

  .pulse-dot.active {
    background: var(--accent);
    box-shadow: 0 0 0 4px rgba(163, 230, 53, 0.15);
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.55; }
  }

  .banner-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: -0.01em;
  }

  .banner-meta {
    display: flex;
    gap: 12px;
    margin-top: 4px;
    font-family: var(--font-mono);
    font-size: 11.5px;
    color: var(--text-muted);
  }

  .banner-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .progress-bar {
    width: 240px;
    height: 4px;
    background: var(--bg-app);
    border-radius: 2px;
    overflow: hidden;
  }

  @media (max-width: 640px) {
    .progress-bar {
      display: none;
    }
  }

  .progress-bar-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width var(--transition-normal);
  }

  /* Buttons */
  .studio-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 7px 12px;
    border-radius: var(--radius-lg);
    font-family: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all var(--transition-fast);
    white-space: nowrap;
  }

  .studio-btn-secondary {
    background: var(--surface);
    border-color: var(--border);
    color: var(--text);
  }

  .studio-btn-secondary:hover {
    background: var(--surface-hover);
    border-color: var(--border-strong);
  }

  .studio-btn-accent {
    background: var(--accent);
    color: var(--accent-ink);
  }

  .studio-btn-accent:hover {
    background: var(--accent);
    opacity: 0.9;
  }

  .studio-btn-danger {
    background: rgba(251, 113, 133, 0.04);
    border-color: rgba(251, 113, 133, 0.15);
    color: var(--rose);
  }

  .studio-btn-danger:hover {
    background: rgba(251, 113, 133, 0.08);
    border-color: var(--rose);
  }
</style>
