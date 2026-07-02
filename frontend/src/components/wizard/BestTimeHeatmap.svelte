<!-- frontend/src/components/wizard/BestTimeHeatmap.svelte -->
<!--
  Mapa de calor de melhores horários para postar no LinkedIn.
  IMPORTANTE: os valores são um BENCHMARK geral do LinkedIn (não são dados
  reais do público do usuário — a API do LinkedIn não expõe impressões de
  perfil pessoal). Encoding sequencial de UM tom (verde do design system):
  quanto mais forte, maior o engajamento típico naquele horário.
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  // Colunas (blocos de horário) e linhas (dias da semana).
  const hours = [6, 8, 10, 12, 14, 16, 18, 20];
  const days = [
    { label: 'Seg', key: 'segunda' },
    { label: 'Ter', key: 'terça' },
    { label: 'Qua', key: 'quarta' },
    { label: 'Qui', key: 'quinta' },
    { label: 'Sex', key: 'sexta' },
    { label: 'Sáb', key: 'sábado' },
    { label: 'Dom', key: 'domingo' },
  ];

  // Matriz de engajamento típico (0–100), alinhada a [dia][hora].
  // Benchmark: meio de semana de manhã concentra o pico; fim de semana é fraco.
  const scores: number[][] = [
    [20, 55, 65, 60, 45, 40, 50, 35], // Seg
    [30, 80, 90, 75, 55, 50, 60, 40], // Ter
    [30, 85, 95, 78, 58, 52, 62, 42], // Qua
    [28, 78, 88, 72, 55, 50, 60, 40], // Qui
    [25, 60, 68, 62, 45, 38, 42, 30], // Sex
    [10, 20, 28, 30, 25, 22, 20, 15], // Sáb
    [12, 22, 30, 32, 28, 25, 28, 20], // Dom
  ];

  const maxScore = Math.max(...scores.flat());

  // Melhor célula (para destaque e atalho "Usar").
  let bestD = 0, bestH = 0;
  scores.forEach((row, d) => row.forEach((v, h) => {
    if (v > scores[bestD][bestH]) { bestD = d; bestH = h; }
  }));

  // Alpha sequencial: baixo recende à superfície, alto satura o verde.
  function alphaFor(v: number): number {
    return +(0.06 + (v / maxScore) * 0.94).toFixed(3);
  }

  function levelLabel(v: number): string {
    if (v >= 80) return 'Alto';
    if (v >= 55) return 'Médio';
    if (v >= 35) return 'Baixo';
    return 'Fraco';
  }

  function fmtHour(h: number): string {
    return `${String(h).padStart(2, '0')}:00`;
  }

  let hovered: { d: number; h: number } | null = null;

  function pick(d: number, h: number) {
    dispatch('select', { day: days[d].key, time: fmtHour(hours[h]) });
  }
</script>

<div class="heatmap">
  <div class="heatmap-grid" style="grid-template-columns: 34px repeat({hours.length}, 1fr);">
    <!-- Cabeçalho de horas -->
    <div class="corner"></div>
    {#each hours as h}
      <div class="col-head">{h}h</div>
    {/each}

    <!-- Linhas por dia -->
    {#each days as day, d}
      <div class="row-head">{day.label}</div>
      {#each hours as _h, h}
        {@const v = scores[d][h]}
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <button
          type="button"
          class="cell {d === bestD && h === bestH ? 'best' : ''}"
          style="background-color: rgba(var(--heat-rgb), {alphaFor(v)});"
          title="{day.label} · {fmtHour(hours[h])} — engajamento {levelLabel(v)} ({v}/100)"
          on:click={() => pick(d, h)}
          on:mouseenter={() => (hovered = { d, h })}
          on:mouseleave={() => (hovered = null)}
          aria-label="Agendar {day.label} às {fmtHour(hours[h])}, engajamento {levelLabel(v)}"
        ></button>
      {/each}
    {/each}
  </div>

  <!-- Rodapé: leitura ao vivo + legenda -->
  <div class="heatmap-footer">
    {#if hovered}
      <span class="readout">
        <b>{days[hovered.d].label} às {fmtHour(hours[hovered.h])}</b>
        · {levelLabel(scores[hovered.d][hovered.h])} ({scores[hovered.d][hovered.h]}/100)
      </span>
    {:else}
      <span class="readout muted">
        Melhor horário: <b>{days[bestD].label} às {fmtHour(hours[bestH])}</b> · clique numa célula para agendar
      </span>
    {/if}

    <div class="legend">
      <span class="legend-label">menos</span>
      <span class="legend-bar"></span>
      <span class="legend-label">mais</span>
    </div>
  </div>
</div>

<style>
  .heatmap {
    /* RGB do verde por tema (mesma família de --accent) para escala sequencial */
    --heat-rgb: 163, 230, 53;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  :global(.theme-light) .heatmap {
    --heat-rgb: 63, 98, 18;
  }

  .heatmap-grid {
    display: grid;
    gap: 2px; /* fresta na cor da superfície separando as células */
  }

  .corner {
    background: transparent;
  }

  .col-head,
  .row-head {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--text-dim);
    display: flex;
    align-items: center;
  }

  .col-head {
    justify-content: center;
    padding-bottom: 2px;
  }

  .row-head {
    justify-content: flex-start;
    padding-right: 4px;
  }

  .cell {
    aspect-ratio: 1 / 1;
    min-height: 20px;
    border: none;
    border-radius: 4px; /* extremidades arredondadas 4px */
    padding: 0;
    cursor: pointer;
    transition: transform var(--transition-fast), box-shadow var(--transition-fast);
  }

  .cell:hover {
    transform: scale(1.08);
    box-shadow: 0 0 0 2px var(--surface), 0 0 0 3px var(--text-dim);
    position: relative;
    z-index: 2;
  }

  .cell.best {
    box-shadow: 0 0 0 2px var(--surface), 0 0 0 3px var(--accent);
  }

  .heatmap-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }

  .readout {
    font-size: 11.5px;
    color: var(--text);
  }

  .readout.muted {
    color: var(--text-muted);
  }

  .readout b {
    color: var(--text);
  }

  .legend {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .legend-label {
    font-size: 10px;
    color: var(--text-dim);
  }

  .legend-bar {
    width: 70px;
    height: 8px;
    border-radius: 4px;
    background: linear-gradient(
      to right,
      rgba(var(--heat-rgb), 0.06),
      rgba(var(--heat-rgb), 1)
    );
  }
</style>
