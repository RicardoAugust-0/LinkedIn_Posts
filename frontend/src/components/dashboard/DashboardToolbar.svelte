<!-- frontend/src/components/dashboard/DashboardToolbar.svelte -->
<script lang="ts">
  import { Filter, ChevronDown } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let filterStatus = 'all';
  export let sortBy = 'newest';
  export let posts: any[] = [];

  let showSortDropdown = false;

  function setFilter(status: string) {
    filterStatus = status;
    dispatch('filter', status);
  }

  function setSort(sort: string) {
    sortBy = sort;
    showSortDropdown = false;
    dispatch('sort', sort);
  }
</script>

<div class="studio-toolbar">
  <div class="segmented-control">
    <button 
      class="segment-btn {filterStatus === 'all' ? 'active' : ''}" 
      on:click={() => setFilter('all')}
    >
      Todos <span class="segment-count">{posts.length}</span>
    </button>
    <button 
      class="segment-btn {filterStatus === 'draft' ? 'active' : ''}" 
      on:click={() => setFilter('draft')}
    >
      Rascunhos <span class="segment-count">{posts.filter(p => p.status === 'draft').length}</span>
    </button>
    <button 
      class="segment-btn {filterStatus === 'scheduled' ? 'active' : ''}" 
      on:click={() => setFilter('scheduled')}
    >
      Agendados <span class="segment-count">{posts.filter(p => p.status === 'scheduled').length}</span>
    </button>
    <button 
      class="segment-btn {filterStatus === 'published' ? 'active' : ''}" 
      on:click={() => setFilter('published')}
    >
      Publicados <span class="segment-count">{posts.filter(p => p.status === 'published').length}</span>
    </button>
    <button 
      class="segment-btn {filterStatus === 'failed' ? 'active' : ''}" 
      on:click={() => setFilter('failed')}
    >
      Falhas <span class="segment-count">{posts.filter(p => p.status === 'failed').length}</span>
    </button>
  </div>

  <div class="sort-btn-wrapper">
    <button class="studio-btn studio-btn-secondary sort-btn" on:click={() => showSortDropdown = !showSortDropdown}>
      <Filter size={13} class="filter-icon" />
      <span>
        {#if sortBy === 'newest'}
          Ordenar: mais recente
        {:else if sortBy === 'oldest'}
          Ordenar: mais antigo
        {:else if sortBy === 'title_asc'}
          Ordenar: A-Z
        {:else if sortBy === 'title_desc'}
          Ordenar: Z-A
        {/if}
      </span>
      <ChevronDown size={11} />
    </button>
    
    {#if showSortDropdown}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="sort-dropdown" on:click={() => showSortDropdown = false}>
        <button class="sort-option-btn {sortBy === 'newest' ? 'active' : ''}" on:click={() => setSort('newest')}>Mais recente</button>
        <button class="sort-option-btn {sortBy === 'oldest' ? 'active' : ''}" on:click={() => setSort('oldest')}>Mais antigo</button>
        <button class="sort-option-btn {sortBy === 'title_asc' ? 'active' : ''}" on:click={() => setSort('title_asc')}>Título (A-Z)</button>
        <button class="sort-option-btn {sortBy === 'title_desc' ? 'active' : ''}" on:click={() => setSort('title_desc')}>Título (Z-A)</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .studio-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  /* Segmented Control */
  .segmented-control {
    display: flex;
    background: var(--surface);
    border: 1px solid var(--border);
    padding: 2.5px;
    border-radius: var(--radius-md);
  }

  .segment-btn {
    border: none;
    background: transparent;
    padding: 5px 12px;
    font-family: inherit;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    display: flex;
    align-items: center;
    gap: 6px;
    transition: all var(--transition-fast);
  }

  .segment-btn:hover {
    color: var(--text);
  }

  .segment-btn.active {
    background: var(--surface-alt);
    color: var(--text);
    box-shadow: var(--shadow-sm);
  }

  .segment-count {
    font-family: var(--font-mono);
    font-size: 10.5px;
    opacity: 0.7;
    background: var(--bg-inset);
    padding: 1px 5px;
    border-radius: 4px;
    font-weight: 400;
  }

  /* Sort dropdown styling */
  .sort-btn-wrapper {
    position: relative;
  }

  .sort-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }

  .sort-btn :global(.filter-icon) {
    color: var(--text-dim);
  }

  .sort-dropdown {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    background: var(--surface);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-popover);
    z-index: 150;
    padding: 4px;
    min-width: 140px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sort-option-btn {
    border: none;
    background: transparent;
    text-align: left;
    padding: 6px 10px;
    font-family: inherit;
    font-size: 12.5px;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .sort-option-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .sort-option-btn.active {
    background-color: var(--surface-alt);
    color: var(--accent);
    font-weight: 500;
  }

  /* Shared buttons */
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
</style>
