<script lang="ts">
  import { Check } from '@lucide/svelte';

  export let images: Array<{ title: string; link: string; thumbnail: string }> = [];
  export let selectedUrl: string | null = null;

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function selectImage(url: string) {
    dispatch('select', url);
  }
</script>

<div class="images-grid">
  {#each images as img}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div 
      class="image-card {selectedUrl === img.link ? 'selected' : ''}" 
      on:click={() => selectImage(img.link)}
    >
      <div class="img-container">
        <img src={img.thumbnail} alt={img.title} class="grid-img" />
        {#if selectedUrl === img.link}
          <div class="selected-overlay">
            <div class="check-badge">
              <Check size={12} />
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .images-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
    margin-top: 8px;
  }

  @media (max-width: 640px) {
    .images-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  .image-card {
    position: relative;
    aspect-ratio: 1;
    border-radius: 9px;
    overflow: hidden;
    cursor: pointer;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    transition: transform var(--transition-fast), border-color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .image-card:hover {
    transform: translateY(-2px);
    border-color: var(--border-strong);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .image-card.selected {
    border: 2px solid var(--text);
  }
  
  :global(.theme-light) .image-card.selected {
    border-color: #1a1a1a;
    box-shadow: 0 0 0 3px rgba(26, 26, 26, 0.08);
  }
  
  :global(.theme-dark) .image-card.selected {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px rgba(163, 230, 53, 0.12);
  }

  .img-container {
    width: 100%;
    height: 100%;
    position: relative;
  }

  .grid-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform var(--transition-normal);
  }

  .image-card:hover .grid-img {
    transform: scale(1.05);
  }

  .selected-overlay {
    position: absolute;
    inset: 0;
    background: rgba(26, 26, 26, 0.05);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.theme-dark) .selected-overlay {
    background: rgba(163, 230, 53, 0.05);
  }

  .check-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 6px rgba(0,0,0,0.15);
  }

  :global(.theme-dark) .check-badge {
    background: var(--accent);
    color: var(--accent-ink);
  }
  
  :global(.theme-light) .check-badge {
    background: #1a1a1a;
    color: #ffffff;
  }
</style>
