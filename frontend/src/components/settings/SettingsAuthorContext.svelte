<!-- frontend/src/components/settings/SettingsAuthorContext.svelte -->
<script lang="ts">
  export let userContext = '';
</script>

<div class="keys-wrapper">
  <div class="card-title-row header-only">
    <div class="title-details">
      <h2>Perfil & Contexto do Autor</h2>
      <p class="card-desc">Adicione informações sobre você para que a inteligência artificial escreva posts personalizados baseados nas suas experiências, cargo e visão de mundo.</p>
    </div>
  </div>

  <div class="studio-field">
    <div class="studio-field-header">
      <label class="studio-label" for="user-context">
        Contexto do Autor (Sobre Mim)
      </label>
      <span class="status-indicator-inline {userContext ? 'valid' : 'empty'}">
        <span class="status-dot"></span>
        {userContext ? 'Ativo' : 'Não Configurado'}
      </span>
    </div>

    <textarea 
      id="user-context"
      class="studio-textarea" 
      bind:value={userContext}
      placeholder="Exemplo: Sou um desenvolvedor backend sênior especializado em Rust, com 6 anos de experiência em sistemas de alta escalabilidade e APIs financeiras. Gosto de focar na legibilidade do código, testes e boas práticas de arquitetura. Escrevo com um tom profissional, direto e explicativo, compartilhando aprendizados práticos do dia a dia..."
      rows={5}
    ></textarea>
    
    <div class="studio-hint">
      Dica: Descreva seu cargo, tecnologias principais, tom de voz desejado (ex: informal, didático, provocador) e suas convicções profissionais. A IA usará este texto como guia de personalidade para seus posts.
    </div>
  </div>
</div>

<style>
  .keys-wrapper {
    display: flex;
    flex-direction: column;
    gap: 14px;
    border-top: 1px solid var(--border);
    margin-top: 20px;
    padding-top: 20px;
  }

  .card-title-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
    padding-bottom: 14px;
  }

  .card-title-row.header-only {
    border-bottom: none;
    padding-bottom: 0;
  }

  .title-details h2 {
    font-size: 14.5px;
    font-weight: 600;
    color: var(--text);
  }

  .card-desc {
    font-size: 11.5px;
    color: var(--text-muted);
    line-height: 1.45;
    margin-top: 3px;
  }

  .studio-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .studio-field-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .studio-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .status-indicator-inline {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 11.5px;
  }

  .status-indicator-inline .status-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
  }

  .status-indicator-inline.valid {
    color: var(--accent);
  }
  .status-indicator-inline.valid .status-dot {
    background-color: var(--accent);
  }

  .status-indicator-inline.empty {
    color: var(--text-dim);
  }
  .status-indicator-inline.empty .status-dot {
    background-color: var(--border-strong);
  }

  .studio-textarea {
    width: 100%;
    padding: 10px 12px;
    background: var(--bg-inset);
    border: 1px solid var(--border);
    border-radius: 9px;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    line-height: 1.5;
    resize: vertical;
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .studio-textarea:focus {
    border-color: var(--border-strong);
  }

  .studio-hint {
    font-size: 11.5px;
    color: var(--text-dim);
    margin-top: 2px;
    line-height: 1.4;
  }
</style>
