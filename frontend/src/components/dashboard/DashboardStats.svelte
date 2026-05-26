<!-- frontend/src/components/dashboard/DashboardStats.svelte -->
<script lang="ts">
  export let stats = {
    total_posts: 0,
    draft_posts: 0,
    scheduled_posts: 0,
    published_posts: 0
  };
</script>

<div class="studio-stat-grid">
  <div class="studio-stat-card">
    <div class="studio-stat-header">
      <span class="studio-stat-label">Total de posts</span>
    </div>
    <div class="studio-stat-value">{stats.total_posts}</div>
    <div class="studio-stat-sub">Desde a criação da conta</div>
  </div>

  <div class="studio-stat-card">
    <div class="studio-stat-header">
      <span class="studio-stat-label">Rascunhos</span>
    </div>
    <div class="studio-stat-value">{stats.draft_posts}</div>
    <div class="studio-stat-sub">Aguardando edição ou envio</div>
  </div>

  <div class="studio-stat-card">
    <div class="studio-stat-header">
      <span class="studio-stat-label">Agendados</span>
    </div>
    <div class="studio-stat-value">{stats.scheduled_posts}</div>
    <div class="studio-stat-sub">Próximo post programado</div>
  </div>

  <div class="studio-stat-card">
    <div class="studio-stat-header">
      <span class="studio-stat-label">Publicados</span>
    </div>
    <div class="studio-stat-value">{stats.published_posts}</div>
    <div class="studio-stat-sub">Sucesso no feed do LinkedIn</div>
  </div>
</div>

<style>
  .studio-stat-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
  }

  @media (max-width: 1024px) {
    .studio-stat-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 640px) {
    .studio-stat-grid {
      grid-template-columns: 1fr;
    }
  }

  .studio-stat-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 16px 20px;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-card);
  }

  .studio-stat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .studio-stat-label {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .studio-stat-value {
    font-size: 2.25rem;
    font-weight: 500;
    letter-spacing: -0.03em;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    color: var(--text);
  }

  .studio-stat-sub {
    font-size: 11.5px;
    color: var(--text-dim);
    margin-top: 8px;
  }
</style>
