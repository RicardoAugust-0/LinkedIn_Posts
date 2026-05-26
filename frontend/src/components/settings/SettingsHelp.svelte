<!-- frontend/src/components/settings/SettingsHelp.svelte -->
<script lang="ts">
  import { Info } from '@lucide/svelte';
  import { API_URL } from '../../lib/api';
</script>

<!-- Help Documentation Card -->
<section class="studio-card developer-help-card">
  <div class="help-card-header">
    <Info size={16} class="info-icon" />
    <h3>Como conectar de verdade à API do LinkedIn?</h3>
  </div>
  <p class="help-card-desc">Para que suas publicações cheguem à sua conta pessoal do LinkedIn, siga o passo a passo abaixo:</p>
  <ol class="help-steps-list">
    <li>Crie uma conta de desenvolvedor em <a href="https://developer.linkedin.com" target="_blank" rel="noopener noreferrer">LinkedIn Developer Portal</a>.</li>
    <li>Crie um novo aplicativo e solicite o produto de publicação <strong>Share on LinkedIn</strong> (escopo <code>w_member_social</code>).</li>
    <li>Nas configurações do App do LinkedIn, registre o endereço callback autorizado: <code class="code-highlight">{API_URL}/api/auth/linkedin/callback</code>.</li>
    <li>Insira o Client ID e Client Secret gerados no formulário acima, clique em Salvar e depois clique em "Conectar Conta" no primeiro painel.</li>
  </ol>
</section>

<style>
  /* Help integrations card */
  .developer-help-card {
    background: rgba(255, 255, 255, 0.01);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
  }

  .help-card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--text);
    margin-bottom: 8px;
  }

  .help-card-header :global(.info-icon) {
    color: var(--text-muted);
  }

  .help-card-desc {
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.45;
    margin-bottom: 12px;
  }

  .help-steps-list {
    font-size: 12px;
    color: var(--text-muted);
    padding-left: 18px;
    line-height: 1.6;
  }

  .help-steps-list li {
    margin-bottom: 6px;
  }

  .help-steps-list a {
    color: var(--text);
    font-weight: 500;
    text-decoration: underline;
  }

  .help-steps-list a:hover {
    color: var(--accent);
  }

  .code-highlight {
    background: var(--surface-alt);
    border: 1px solid var(--border);
    padding: 1px 5px;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text);
  }
</style>
