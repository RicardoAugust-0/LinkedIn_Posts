<!-- frontend/src/components/wizard/StepSchedule.svelte -->
<script lang="ts">
  import { Calendar, Clock, Send, ArrowLeft, Sparkles } from '@lucide/svelte';
  import PostPreview from '../PostPreview.svelte';
  import { postStore } from '../../lib/stores/postStore';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  const aiSuggestedTimes = [
    { day: 'Quarta', time: '07:00', tag: 'Pico de tech', best: true },
    { day: 'Quinta', time: '09:30', tag: 'Alto alcance', best: false },
    { day: 'Sexta',  time: '12:15', tag: 'Boa retenção', best: false }
  ];

  function selectSuggestedTime(dayText: string, timeStr: string) {
    postStore.setScheduleOption('schedule');
    
    // Calcular data correspondente ao dia da semana sugerido
    const daysMap: { [key: string]: number } = { 'domingo': 0, 'segunda': 1, 'terça': 2, 'quarta': 3, 'quinta': 4, 'sexta': 5, 'sábado': 6 };
    const targetDay = daysMap[dayText.toLowerCase()] ?? 3; // Padrão Quarta-feira
    
    const today = new Date();
    const resultDate = new Date(today);
    
    const currentDay = today.getDay();
    let distance = targetDay - currentDay;
    if (distance <= 0) {
      distance += 7; // Próxima semana
    }
    resultDate.setDate(today.getDate() + distance);
    
    const yyyy = resultDate.getFullYear();
    const mm = String(resultDate.getMonth() + 1).padStart(2, '0');
    const dd = String(resultDate.getDate()).padStart(2, '0');
    
    postStore.setScheduleDate(`${yyyy}-${mm}-${dd}`);
    postStore.setScheduleTime(timeStr);
  }

  function handleSave(publishImmediately: boolean) {
    postStore.savePost(publishImmediately, () => {
      dispatch('saved');
    });
  }
</script>

<div class="step-content-layout split">
  <!-- Col 1: Schedule -->
  <div class="studio-card schedule-panel">
    <div class="panel-section-header">
      <span class="panel-eyebrow">quando publicar</span>
      <h2>Defina o agendamento</h2>
    </div>

    <!-- Radio Selection -->
    <div class="schedule-options-group">
      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label class="schedule-radio-card {$postStore.scheduleOption === 'now' ? 'selected' : ''}">
        <input 
          type="radio" 
          name="schedule-choice" 
          value="now" 
          checked={$postStore.scheduleOption === 'now'}
          on:change={() => postStore.setScheduleOption('now')}
          class="radio-input-raw" 
        />
        <div class="radio-circle"></div>
        <div class="radio-content">
          <div class="radio-title">Publicar agora</div>
          <div class="radio-desc">Vai para a fila imediata de publicação do LinkedIn.</div>
        </div>
      </label>

      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label class="schedule-radio-card {$postStore.scheduleOption === 'schedule' ? 'selected' : ''}">
        <input 
          type="radio" 
          name="schedule-choice" 
          value="schedule" 
          checked={$postStore.scheduleOption === 'schedule'}
          on:change={() => postStore.setScheduleOption('schedule')}
          class="radio-input-raw" 
        />
        <div class="radio-circle"></div>
        <div class="radio-content">
          <div class="radio-title">Agendar para mais tarde</div>
          <div class="radio-desc">Programação exata de postagem em background.</div>
        </div>
      </label>
    </div>

    <!-- Date & Time Selectors -->
    {#if $postStore.scheduleOption === 'schedule'}
      <div class="date-time-fields-row">
        <div class="studio-field">
          <label class="studio-label" for="date">Data de publicação</label>
          <input 
            type="date" 
            id="date" 
            class="studio-input" 
            value={$postStore.scheduleDate} 
            on:input={(e) => postStore.setScheduleDate(e.currentTarget.value)}
          />
        </div>
        <div class="studio-field">
          <label class="studio-label" for="time">Hora</label>
          <input 
            type="time" 
            id="time" 
            class="studio-input" 
            value={$postStore.scheduleTime} 
            on:input={(e) => postStore.setScheduleTime(e.currentTarget.value)}
          />
        </div>
      </div>
      <div class="timezone-hint">fuso local: America/Sao_Paulo (UTC-3)</div>
    {/if}

    <!-- AI suggested times -->
    <div class="ai-suggested-card">
      <div class="suggested-header">
        <Sparkles size={11} class="spark-icon" />
        <span><b>Sugestão da IA</b> · com base na atividade do seu público</span>
      </div>

      <div class="suggested-times-list">
        {#each aiSuggestedTimes as sugg}
          <div class="suggested-time-item {sugg.best ? 'best' : ''}">
            <span class="suggested-time-label">{sugg.day} às {sugg.time}</span>
            <span class="suggested-time-tag {sugg.best ? 'best-tag' : ''}">{sugg.tag}</span>
            <div class="flex-spacer"></div>
            <button class="studio-btn studio-btn-secondary compact-btn use-time-btn" on:click={() => selectSuggestedTime(sugg.day, sugg.time)}>
              Usar
            </button>
          </div>
        {/each}
      </div>
    </div>

    <div class="flex-spacer"></div>

    <div class="form-actions">
      <button class="studio-btn studio-btn-secondary" on:click={() => postStore.setStep(2)} disabled={$postStore.savingPost}>
        <ArrowLeft size={12} />
        <span>Voltar</span>
      </button>
      
      {#if $postStore.scheduleOption === 'now'}
        <button class="studio-btn studio-btn-accent flex-spacer" on:click={() => handleSave(true)} disabled={$postStore.savingPost}>
          {#if $postStore.savingPost}
            <span class="mini-spinner"></span>
            <span>Publicando...</span>
          {:else}
            <Send size={12} />
            <span>Confirmar publicação agora</span>
          {/if}
        </button>
      {:else}
        <button class="studio-btn studio-btn-accent flex-spacer" on:click={() => handleSave(false)} disabled={$postStore.savingPost}>
          {#if $postStore.savingPost}
            <span class="mini-spinner"></span>
            <span>Agendando...</span>
          {:else}
            <Calendar size={12} />
            <span>Confirmar agendamento</span>
          {/if}
        </button>
      {/if}
    </div>
  </div>

  <!-- Col 2: Live LinkedIn Mockup -->
  <div class="studio-card live-preview-panel">
    <div class="panel-section-header">
      <span class="panel-eyebrow">preview ao vivo</span>
      <h2>Visualização no feed</h2>
    </div>

    <div class="feed-preview-scroller flex-spacer">
      <PostPreview 
        content={$postStore.postContent}
        imageUrl={$postStore.selectedImageUrl}
        title={$postStore.postTitle}
        imageSource={$postStore.imageSource}
        compact={true}
      />
    </div>
  </div>
</div>

<style>
  /* Scoped layout rules from original CreatePost */
  .step-content-layout {
    min-height: 0;
    flex: 1;
  }

  .step-content-layout.split {
    display: grid;
    grid-template-columns: 1.1fr 1fr;
    gap: 24px;
  }

  @media (max-width: 1024px) {
    .step-content-layout.split {
      grid-template-columns: 1fr;
    }
  }

  .schedule-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
    max-height: calc(100vh - 310px);
    overflow-y: auto;
  }

  .panel-section-header {
    margin-bottom: 4px;
  }

  .panel-section-header h2 {
    font-size: 17px;
  }

  .schedule-options-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .schedule-radio-card {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 12px 14px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 10px;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .schedule-radio-card:hover {
    border-color: var(--border-strong);
  }

  .schedule-radio-card.selected {
    border-color: var(--text);
    background: var(--surface-alt);
    box-shadow: 0 0 0 1px var(--text);
  }

  :global(.theme-dark) .schedule-radio-card.selected {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent);
  }

  .radio-input-raw {
    display: none;
  }

  .radio-circle {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid var(--text-dim);
    margin-top: 1px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    transition: all var(--transition-fast);
  }

  .schedule-radio-card.selected .radio-circle {
    border-color: var(--text);
    background: var(--text);
  }

  :global(.theme-dark) .schedule-radio-card.selected .radio-circle {
    border-color: var(--accent);
    background: var(--accent);
  }

  .schedule-radio-card.selected .radio-circle::after {
    content: '';
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--bg-app);
  }

  .radio-content {
    flex: 1;
    line-height: 1.35;
  }

  .radio-title {
    font-size: 13.5px;
    font-weight: 500;
    color: var(--text);
  }

  .radio-desc {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 3px;
  }

  .date-time-fields-row {
    display: grid;
    grid-template-columns: 1.4fr 1fr;
    gap: 10px;
    margin-bottom: -10px;
  }

  .timezone-hint {
    font-family: var(--font-mono);
    font-size: 11.5px;
    color: var(--text-dim);
  }

  /* Suggested Times */
  .ai-suggested-card {
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 14px;
    margin-top: 4px;
  }

  .suggested-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 10px;
  }

  .suggested-header .spark-icon {
    color: var(--accent);
  }

  :global(.theme-light) .suggested-header .spark-icon {
    color: var(--text-muted);
  }

  .suggested-times-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .suggested-time-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 8px;
    border-radius: 7px;
    background: transparent;
    border: 1px solid transparent;
  }

  .suggested-time-item.best {
    background: var(--surface);
    border-color: var(--border);
  }

  .suggested-time-label {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text);
    width: 110px;
  }

  .suggested-time-tag {
    font-size: 11px;
    color: var(--text-muted);
    padding: 2px 7px;
    border-radius: 99px;
    background: var(--bg-app);
    border: 1px solid var(--border);
  }

  .suggested-time-tag.best-tag {
    background: var(--accent-muted);
    border-color: rgba(163, 230, 53, 0.2);
    color: var(--accent);
  }

  :global(.theme-light) .suggested-time-tag.best-tag {
    background: var(--bg-inset);
    border-color: var(--border);
    color: var(--text-muted);
  }

  .use-time-btn {
    font-size: 11px;
    padding: 3px 9px;
  }

  /* Live Preview (Step 3 Right) */
  .live-preview-panel {
    display: flex;
    flex-direction: column;
    background: var(--surface-alt);
    max-height: calc(100vh - 310px);
    overflow: hidden;
  }

  .feed-preview-scroller {
    overflow-y: auto;
    padding: 10px 0;
  }

  .form-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }
</style>
