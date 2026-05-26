<!-- frontend/src/components/settings/SettingsDiagnostics.svelte -->
<script lang="ts">
  import { CheckCircle2, AlertCircle } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  interface DiagnosticResult {
    success: boolean;
    message: string;
  }

  export let testResults: {
    gemini?: DiagnosticResult;
    pexels?: DiagnosticResult;
    linkedin?: DiagnosticResult;
  } | null = null;
</script>

{#if testResults}
  <div class="diagnostic-panel-card">
    <div class="diagnostic-panel-header">
      <h3>Diagnóstico das Conexões</h3>
      <button type="button" class="close-panel-btn" on:click={() => dispatch('close')}>Limpar</button>
    </div>
    
    <div class="diagnostic-list">
      <!-- Gemini Diagnostic -->
      {#if testResults.gemini}
        <div class="diagnostic-item-row {testResults.gemini.success ? 'success' : 'failed'}">
          <div class="diagnostic-meta">
            <div class="diagnostic-icon-wrapper">
              {#if testResults.gemini.success}
                <CheckCircle2 size={14} />
              {:else}
                <AlertCircle size={14} />
              {/if}
            </div>
            <div class="diagnostic-info-col">
              <span class="diagnostic-name">Gemini API</span>
              <span class="diagnostic-desc">{testResults.gemini.message}</span>
            </div>
          </div>
        </div>
      {/if}

      <!-- Pexels Diagnostic -->
      {#if testResults.pexels}
        <div class="diagnostic-item-row {testResults.pexels.success ? 'success' : 'failed'}">
          <div class="diagnostic-meta">
            <div class="diagnostic-icon-wrapper">
              {#if testResults.pexels.success}
                <CheckCircle2 size={14} />
              {:else}
                <AlertCircle size={14} />
              {/if}
            </div>
            <div class="diagnostic-info-col">
              <span class="diagnostic-name">Pexels API (Imagens)</span>
              <span class="diagnostic-desc">{testResults.pexels.message}</span>
            </div>
          </div>
        </div>
      {/if}

      <!-- LinkedIn Diagnostic -->
      {#if testResults.linkedin}
        <div class="diagnostic-item-row {testResults.linkedin.success ? 'success' : 'failed'}">
          <div class="diagnostic-meta">
            <div class="diagnostic-icon-wrapper">
              {#if testResults.linkedin.success}
                <CheckCircle2 size={14} />
              {:else}
                <AlertCircle size={14} />
              {/if}
            </div>
            <div class="diagnostic-info-col">
              <span class="diagnostic-name">LinkedIn API</span>
              <span class="diagnostic-desc">{testResults.linkedin.message}</span>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .diagnostic-panel-card {
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 9px;
    padding: 16px;
    margin-top: 18px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .diagnostic-panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .diagnostic-panel-header h3 {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }

  .close-panel-btn {
    background: transparent;
    border: none;
    color: var(--text-dim);
    font-size: 11px;
    cursor: pointer;
    transition: color var(--transition-fast);
  }

  .close-panel-btn:hover {
    color: var(--text);
  }

  .diagnostic-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .diagnostic-item-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    border-radius: 6px;
    border: 1px solid transparent;
  }

  .diagnostic-item-row.success {
    background: rgba(163, 230, 53, 0.03);
    border-color: rgba(163, 230, 53, 0.15);
  }

  .diagnostic-item-row.failed {
    background: rgba(251, 113, 133, 0.03);
    border-color: rgba(251, 113, 133, 0.15);
  }

  .diagnostic-meta {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .diagnostic-icon-wrapper {
    margin-top: 2px;
    flex-shrink: 0;
  }

  .diagnostic-item-row.success .diagnostic-icon-wrapper {
    color: var(--accent);
  }

  .diagnostic-item-row.failed .diagnostic-icon-wrapper {
    color: var(--rose);
  }

  .diagnostic-info-col {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .diagnostic-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--text);
  }

  .diagnostic-desc {
    font-size: 11px;
    color: var(--text-muted);
  }
</style>
