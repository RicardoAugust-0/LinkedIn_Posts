<!-- frontend/src/components/automation/AutomationQueueTable.svelte -->
<script lang="ts">
  import { Calendar } from '@lucide/svelte';
  import AutomationQueueItem from './AutomationQueueItem.svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let queue: any[] = [];
  export let expandedId: string | null = null;
  export let topicSeed = '';
  export let tone = '';

  function handleExpand(id: string) {
    if (expandedId === id) {
      expandedId = null;
    } else {
      expandedId = id;
    }
  }
</script>

<div class="studio-card queue-card">
  <div class="card-title-row">
    <div class="title-details">
      <span class="panel-eyebrow-small">Fila de Envios</span>
      <h2 class="section-title">Programação da Esteira</h2>
    </div>
  </div>

  <div class="queue-workspace">
    {#if queue.length === 0}
      <div class="queue-empty-state">
        <Calendar size={32} class="empty-icon" />
        <h3>Nenhuma publicação gerada</h3>
        <p>Preencha os parâmetros e clique em 'Gerar Automação Completa' para criar posts.</p>
      </div>
    {:else}
      <div class="queue-list">
        {#each queue as post, i (post.id)}
          <AutomationQueueItem 
            {post} 
            {i} 
            expanded={expandedId === post.id} 
            {topicSeed} 
            {tone} 
            on:expand={(e) => handleExpand(e.detail)} 
            on:edit={(e) => dispatch('edit', e.detail)}
            on:regenerate={(e) => dispatch('regenerate', e.detail)}
            on:skip={(e) => dispatch('skip', e.detail)}
            on:changeImage={(e) => dispatch('changeImage', e.detail)}
            on:reschedule={(e) => dispatch('reschedule', e.detail)}
            on:remove={(e) => dispatch('remove', e.detail)}
          />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .queue-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 20px;
    box-shadow: var(--shadow-card);
    height: 100%;
    min-height: 500px;
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

  .queue-workspace {
    flex: 1;
    overflow-y: auto;
    max-height: calc(100vh - 250px);
    padding-right: 4px;
  }

  /* Empty state */
  .queue-empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 80px 20px;
    color: var(--text-dim);
  }

  .queue-empty-state :global(.empty-icon) {
    color: var(--text-dim);
    margin-bottom: 12px;
  }

  .queue-empty-state h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 6px;
  }

  .queue-empty-state p {
    font-size: 12.5px;
    color: var(--text-muted);
    max-width: 250px;
    line-height: 1.45;
  }

  /* List */
  .queue-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
