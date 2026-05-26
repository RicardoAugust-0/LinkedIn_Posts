<script lang="ts">
  export let status: 'draft' | 'scheduled' | 'published' | 'failed' | string = 'draft';
  export let mono: boolean = false;

  $: translatedStatus = (() => {
    switch (status) {
      case 'published': return 'Publicado';
      case 'scheduled': return 'Agendado';
      case 'draft': return 'Rascunho';
      case 'failed': return 'Falhou';
      default: return status;
    }
  })();
</script>

<span class="status-badge {status} {mono ? 'mono' : ''}">
  <span class="status-dot"></span>
  <span class="status-label">{translatedStatus}</span>
</span>

<style>
  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 12px;
    font-weight: 500;
    line-height: 1;
    white-space: nowrap;
    transition: all var(--transition-fast);
  }

  .status-badge.mono {
    padding: 3px 8px;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 0.04em;
    text-transform: lowercase;
    border: 1px solid transparent;
  }

  .status-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    transition: background-color var(--transition-fast);
  }

  /* Colors based on CSS variables */
  .status-badge.published {
    background-color: var(--accent-muted);
    color: var(--accent);
  }
  .status-badge.published .status-dot {
    background-color: var(--accent);
  }
  .status-badge.mono.published {
    border-color: rgba(163, 230, 53, 0.2);
  }
  :global(.theme-light) .status-badge.mono.published {
    border-color: rgba(63, 98, 18, 0.2);
  }

  .status-badge.scheduled {
    background-color: var(--cyan-muted);
    color: var(--cyan);
  }
  .status-badge.scheduled .status-dot {
    background-color: var(--cyan);
  }
  .status-badge.mono.scheduled {
    border-color: rgba(103, 232, 249, 0.2);
  }
  :global(.theme-light) .status-badge.mono.scheduled {
    border-color: rgba(14, 116, 144, 0.2);
  }

  .status-badge.failed {
    background-color: var(--rose-muted);
    color: var(--rose);
  }
  .status-badge.failed .status-dot {
    background-color: var(--rose);
  }
  .status-badge.mono.failed {
    border-color: rgba(251, 113, 133, 0.2);
  }
  :global(.theme-light) .status-badge.mono.failed {
    border-color: rgba(190, 18, 60, 0.2);
  }

  .status-badge.draft {
    background-color: var(--surface-alt);
    color: var(--text-muted);
  }
  .status-badge.draft .status-dot {
    background-color: var(--text-dim);
  }
  .status-badge.mono.draft {
    border-color: var(--border);
    background-color: var(--bg-inset);
  }
</style>
