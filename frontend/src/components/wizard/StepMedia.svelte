<!-- frontend/src/components/wizard/StepMedia.svelte -->
<script lang="ts">
  import { Sparkles, Search, Image as ImageIcon, ArrowRight, ArrowLeft, Trash2 } from '@lucide/svelte';
  import ImageGrid from '../ImageGrid.svelte';
  import { postStore } from '../../lib/stores/postStore';
  import { API_URL } from '../../lib/api';

  let activeMediaTab: 'search' | 'ai' | 'upload' = 'search';

  let selectedMode: 'professional' | 'storytelling' | 'direct' | 'persuasive' = 'professional';
  let improvingText = false;
  let improvedContent: string | null = null;

  async function improvePostContent() {
    if (!$postStore.postContent.trim()) {
      postStore.setError("Escreva algo no post antes de tentar melhorar.");
      return;
    }
    improvingText = true;
    improvedContent = null;
    postStore.setError(null);
    try {
      const res = await fetch(`${API_URL}/api/generate/improve`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          content: $postStore.postContent,
          mode: selectedMode
        })
      });
      if (res.ok) {
        const data = await res.json();
        improvedContent = data.content;
      } else {
        const txt = await res.text();
        let parsed;
        try { parsed = JSON.parse(txt); } catch { parsed = null; }
        const errorMsg = parsed && parsed.error ? parsed.error : txt;
        postStore.setError(`Erro ao melhorar texto: ${errorMsg}`);
      }
    } catch (e) {
      postStore.setError("Erro de conexão ao melhorar o texto.");
    } finally {
      improvingText = false;
    }
  }

  function applyImprovement() {
    if (improvedContent) {
      postStore.setPostContent(improvedContent);
      improvedContent = null;
    }
  }

  function discardImprovement() {
    improvedContent = null;
  }

  function handleSelectGoogleImage(event: CustomEvent<string>) {
    postStore.setSelectedImageUrl(event.detail);
    postStore.setImageSource('google');
  }

  function handleLocalUpload(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      const file = target.files[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target && typeof e.target.result === 'string') {
          postStore.setSelectedImageUrl(e.target.result);
          postStore.setImageSource('upload');
        }
      };
      reader.readAsDataURL(file);
    }
  }

  function removeImage() {
    postStore.setSelectedImageUrl(null);
    postStore.setImageSource('none');
  }
</script>

<div class="step-content-layout split">
  <!-- Col 1: Text Editor -->
  <div class="studio-card editor-panel">
    <div class="editor-header">
      <span class="editor-status-dot"></span>
      <span>Editor · alterações salvas automaticamente</span>
    </div>

    <div class="studio-field">
      <div class="studio-field-header">
        <label class="studio-label" for="edit-title">Título interno</label>
      </div>
      <input 
        id="edit-title"
        type="text" 
        class="studio-input" 
        value={$postStore.postTitle}
        on:input={(e) => postStore.setPostTitle(e.currentTarget.value)}
      />
      <div class="studio-hint">Apenas para sua organização — não aparece na publicação do LinkedIn.</div>
    </div>

    <div class="studio-field flex-grow-field">
      <div class="studio-field-header">
        <label class="studio-label" for="edit-content">Conteúdo do post</label>
        <span class="studio-optional">{$postStore.postContent.length} / 3000</span>
      </div>
      <textarea 
        id="edit-content"
        class="studio-textarea editor-textarea" 
        value={$postStore.postContent}
        on:input={(e) => postStore.setPostContent(e.currentTarget.value)}
        rows={10}
      ></textarea>
    </div>

    <!-- AI Post Enricher -->
    <div class="ai-enricher-card">
      <div class="enricher-header">
        <div class="enricher-title">
          <Sparkles size={14} class="suggestion-spark" />
          <span>Enriquecedor de Post</span>
        </div>
        <span class="enricher-badge">Gemini 3.5 Flash</span>
      </div>

      {#if !improvedContent}
        <div class="enricher-body">
          <p class="enricher-desc">Ajuste o tom e o estilo do seu post de forma inteligente.</p>
          <div class="enricher-modes">
            <button 
              type="button"
              class="mode-pill {selectedMode === 'professional' ? 'active' : ''}" 
              on:click={() => selectedMode = 'professional'}
            >
              💼 Profissional
            </button>
            <button 
              type="button"
              class="mode-pill {selectedMode === 'storytelling' ? 'active' : ''}" 
              on:click={() => selectedMode = 'storytelling'}
            >
              📖 Storytelling
            </button>
            <button 
              type="button"
              class="mode-pill {selectedMode === 'direct' ? 'active' : ''}" 
              on:click={() => selectedMode = 'direct'}
            >
              🎯 Direto
            </button>
            <button 
              type="button"
              class="mode-pill {selectedMode === 'persuasive' ? 'active' : ''}" 
              on:click={() => selectedMode = 'persuasive'}
            >
              🔥 Persuasivo
            </button>
          </div>
          
          <button 
            type="button"
            class="studio-btn studio-btn-accent enricher-btn" 
            on:click={improvePostContent}
            disabled={improvingText || !$postStore.postContent.trim()}
          >
            {#if improvingText}
              <span class="mini-spinner text-accent-foreground"></span>
              <span>Melhorando...</span>
            {:else}
              <span>Melhorar Texto</span>
            {/if}
          </button>
        </div>
      {:else}
        <div class="enricher-compare-view">
          <div class="compare-header">
            <span>Sugestão da IA (Estilo {selectedMode === 'professional' ? 'Profissional' : selectedMode === 'storytelling' ? 'Storytelling' : selectedMode === 'direct' ? 'Direto' : 'Persuasivo'}):</span>
          </div>
          <div class="compare-box">
            <div class="compare-scroll">
              <p class="compare-text-content">{improvedContent}</p>
            </div>
          </div>
          <div class="compare-actions">
            <button 
              type="button"
              class="studio-btn studio-btn-ghost compact-btn discard-btn" 
              on:click={discardImprovement}
            >
              Descartar
            </button>
            <button 
              type="button"
              class="studio-btn studio-btn-primary compact-btn apply-btn" 
              on:click={applyImprovement}
            >
              Aceitar e Substituir
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- Col 2: Media Picker Tabs -->
  <div class="studio-card media-panel">
    <div class="media-tabs-header">
      <button 
        class="tab-btn {activeMediaTab === 'search' ? 'active' : ''}" 
        on:click={() => activeMediaTab = 'search'}
      >
        <Search size={13} />
        <span>Buscar no Google</span>
      </button>
      <button 
        class="tab-btn {activeMediaTab === 'ai' ? 'active' : ''}" 
        on:click={() => activeMediaTab = 'ai'}
      >
        <Sparkles size={12} class="tab-ai-icon" />
        <span>Gerar com IA</span>
      </button>
      <button 
        class="tab-btn {activeMediaTab === 'upload' ? 'active' : ''}" 
        on:click={() => activeMediaTab = 'upload'}
      >
        <ImageIcon size={13} />
        <span>Upload</span>
      </button>
    </div>

    <div class="tab-content flex-spacer">
      <!-- Google Search Tab -->
      {#if activeMediaTab === 'search'}
        <div class="search-tab-content">
          <div class="search-input-row">
            <input 
              type="text" 
              class="studio-input" 
              placeholder="Ex: rust language code concept" 
              value={$postStore.googleSearchQuery}
              on:input={(e) => postStore.setGoogleSearchQuery(e.currentTarget.value)}
            />
            <button class="studio-btn studio-btn-accent search-action-btn" on:click={() => postStore.searchImages()} disabled={$postStore.searchingImages}>
              {#if $postStore.searchingImages}<span class="mini-spinner"></span>{:else}Buscar{/if}
            </button>
          </div>

          {#if $postStore.googleImages.length > 0}
            <div class="grid-scroll-container">
              <ImageGrid 
                images={$postStore.googleImages} 
                selectedUrl={$postStore.imageSource === 'google' ? $postStore.selectedImageUrl : null} 
                on:select={handleSelectGoogleImage}
              />
            </div>
          {:else}
            <div class="media-empty-state">
              <Search size={24} class="empty-icon" />
              <p>Digite termos de busca acima para encontrar referências no Google Images.</p>
            </div>
          {/if}
        </div>
      {/if}

      <!-- AI Generate Tab -->
      {#if activeMediaTab === 'ai'}
        <div class="ai-tab-content">
          <div class="studio-field">
            <div class="studio-field-header">
              <label class="studio-label" for="prompt">Prompt da imagem (Imagen 4)</label>
            </div>
            <textarea 
              id="prompt"
              class="studio-textarea" 
              placeholder="Descreva detalhadamente a ilustração tecnológica..."
              value={$postStore.aiImagePrompt}
              on:input={(e) => postStore.setAiImagePrompt(e.currentTarget.value)}
              rows={4}
              disabled={$postStore.generatingImage}
            ></textarea>
          </div>

          <button class="studio-btn studio-btn-accent" on:click={() => postStore.generateImage()} disabled={$postStore.generatingImage || !$postStore.aiImagePrompt.trim()}>
            {#if $postStore.generatingImage}
              <span class="mini-spinner"></span>
              <span>Renderizando com Imagen 4...</span>
            {:else}
              <ImageIcon size={12} />
              <span>Gerar imagem por IA</span>
            {/if}
          </button>

          {#if $postStore.generatedImageUrl}
            <div class="generated-preview-box">
              <img src={$postStore.generatedImageUrl.startsWith('/uploads') ? `${API_URL}${$postStore.generatedImageUrl}` : $postStore.generatedImageUrl} alt="Gerada por IA" class="generated-preview-img" />
              <div class="generated-badge-overlay">
                <span class="badge-tag">gerada via Imagen 4</span>
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Upload Tab -->
      {#if activeMediaTab === 'upload'}
        <div class="upload-tab-content">
          <label class="drag-upload-box">
            <ImageIcon size={32} class="upload-icon" />
            <span class="upload-title">Clique para selecionar imagem</span>
            <span class="upload-subtitle">Formatos aceitos: png, jpeg, webp. Limite local 5MB.</span>
            <input type="file" accept="image/*" class="file-input-raw" on:change={handleLocalUpload} />
          </label>
          
          {#if $postStore.imageSource === 'upload' && $postStore.selectedImageUrl}
            <div class="generated-preview-box">
              <img src={$postStore.selectedImageUrl} alt="Imagem de upload" class="generated-preview-img" />
              <button class="studio-btn studio-btn-danger remove-btn-overlay" on:click={removeImage}>
                <Trash2 size={12} /> Remover mídia
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Footer Tab Selected Media Status -->
    <div class="media-panel-footer">
      <div class="media-footer-status">
        {#if $postStore.selectedImageUrl}
          <img src={$postStore.selectedImageUrl.startsWith('/uploads') ? `${API_URL}${$postStore.selectedImageUrl}` : $postStore.selectedImageUrl} alt="Thumbnail selecionado" class="footer-thumb" />
          <div class="footer-thumb-info">
            <div class="footer-thumb-title">1 imagem selecionada</div>
            <div class="footer-thumb-subtitle uppercase-text">{$postStore.imageSource} source · ativo</div>
          </div>
          <button class="studio-btn studio-btn-ghost compact-btn remove-image-btn" on:click={removeImage}>
            <Trash2 size={11} />
          </button>
        {:else}
          <div class="footer-thumb empty-thumb-icon">
            <ImageIcon size={14} />
          </div>
          <div class="footer-thumb-info">
            <div class="footer-thumb-title">Nenhuma mídia selecionada</div>
            <div class="footer-thumb-subtitle">Apenas texto puro (opcional)</div>
          </div>
        {/if}
      </div>

      <div class="footer-actions">
        <button class="studio-btn studio-btn-secondary" on:click={() => postStore.setStep(1)}>
          <ArrowLeft size={12} />
          <span>Voltar</span>
        </button>
        <button class="studio-btn studio-btn-primary" on:click={() => postStore.setStep(3)}>
          <span>Próximo: Revisão</span>
          <ArrowRight size={12} />
        </button>
      </div>
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

  .editor-panel {
    display: flex;
    flex-direction: column;
    gap: 14px;
    max-height: calc(100vh - 310px);
    overflow: hidden;
  }

  .editor-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11.5px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    margin-bottom: -4px;
  }

  .editor-status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: var(--accent);
  }

  .flex-grow-field {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .editor-textarea {
    flex: 1;
    min-height: 180px;
    font-size: 13.5px;
  }

  .ai-enricher-card {
    padding: 14px;
    background: var(--surface-alt);
    border: 1px solid var(--border);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    transition: all 0.3s ease;
  }

  :global(.theme-dark) .ai-enricher-card {
    background: rgba(25, 25, 25, 0.6);
    border-color: rgba(255, 255, 255, 0.08);
  }

  .enricher-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 13px;
  }

  .enricher-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-weight: 600;
    color: var(--text);
  }

  :global(.suggestion-spark) {
    color: var(--accent);
    flex-shrink: 0;
  }

  :global(.theme-light) :global(.suggestion-spark) {
    color: var(--text-muted);
  }

  .enricher-badge {
    font-size: 10px;
    color: var(--accent);
    background: rgba(163, 230, 53, 0.1);
    padding: 2px 6px;
    border-radius: 20px;
    font-family: var(--font-mono);
  }

  :global(.theme-light) .enricher-badge {
    color: var(--text-strong);
    background: var(--surface);
    border: 1px solid var(--border);
  }

  .enricher-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .enricher-desc {
    font-size: 12px;
    color: var(--text-muted);
    margin: 0;
  }

  .enricher-modes {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 6px;
  }

  .mode-pill {
    padding: 8px 10px;
    font-size: 11.5px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    color: var(--text-muted);
    cursor: pointer;
    text-align: center;
    transition: all 0.2s ease;
    font-family: inherit;
  }

  .mode-pill:hover {
    color: var(--text);
    border-color: var(--border-strong);
    background: var(--surface-hover);
  }

  .mode-pill.active {
    color: var(--accent);
    border-color: var(--accent);
    background: rgba(163, 230, 53, 0.05);
    font-weight: 500;
  }

  :global(.theme-light) .mode-pill.active {
    color: var(--text);
    border-color: var(--text);
    background: var(--surface-alt);
  }

  .enricher-btn {
    width: 100%;
    justify-content: center;
    font-weight: 600;
  }

  .enricher-compare-view {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .compare-header {
    font-size: 11.5px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .compare-box {
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--surface);
    padding: 10px;
    max-height: 140px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .compare-scroll {
    overflow-y: auto;
    flex: 1;
  }

  .compare-text-content {
    font-size: 12.5px;
    color: var(--text);
    white-space: pre-wrap;
    margin: 0;
    line-height: 1.5;
  }

  .compare-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 4px;
  }

  .discard-btn {
    font-size: 11.5px;
  }

  .apply-btn {
    font-size: 11.5px;
    font-weight: 600;
  }

  /* Col 2 Media Panel */
  .media-panel {
    display: flex;
    flex-direction: column;
    padding: 0;
    max-height: calc(100vh - 310px);
    overflow: hidden;
  }

  .media-tabs-header {
    display: flex;
    border-bottom: 1px solid var(--border);
    padding: 0 8px;
    background: var(--surface);
  }

  .tab-btn {
    padding: 14px;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 12.5px;
    font-weight: 400;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: -1px;
    transition: all var(--transition-fast);
  }

  .tab-btn:hover {
    color: var(--text);
  }

  .tab-btn.active {
    color: var(--text);
    font-weight: 500;
    border-bottom-color: var(--text);
  }

  :global(.theme-dark) .tab-btn.active {
    border-bottom-color: var(--accent);
  }

  :global(.tab-ai-icon) {
    color: var(--text-dim);
  }

  .tab-btn.active :global(.tab-ai-icon) {
    color: var(--accent);
  }

  .tab-content {
    padding: 16px;
    overflow-y: auto;
    flex: 1;
  }

  .search-input-row {
    display: flex;
    gap: 8px;
    margin-bottom: 10px;
  }

  .search-action-btn {
    font-weight: 500;
  }

  .grid-scroll-container {
    max-height: 240px;
    overflow-y: auto;
  }

  .media-empty-state {
    padding: 40px 20px;
    text-align: center;
    color: var(--text-dim);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  :global(.empty-icon) {
    color: var(--text-dim);
  }

  .generated-preview-box {
    margin-top: 14px;
    position: relative;
    border-radius: 9px;
    overflow: hidden;
    aspect-ratio: 1.6;
    border: 1px solid var(--border);
    max-height: 200px;
  }

  .generated-preview-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .generated-badge-overlay {
    position: absolute;
    top: 8px;
    left: 8px;
    padding: 4px 8px;
    background: rgba(0,0,0,0.6);
    border-radius: 4px;
    font-size: 11px;
    color: #ffffff;
    font-family: var(--font-mono);
  }

  /* File upload */
  .drag-upload-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border: 2px dashed var(--border);
    border-radius: 9px;
    padding: 30px;
    cursor: pointer;
    background: var(--surface-alt);
    transition: border-color var(--transition-fast);
  }

  .drag-upload-box:hover {
    border-color: var(--border-strong);
  }

  :global(.upload-icon) {
    color: var(--text-dim);
  }

  .upload-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .upload-subtitle {
    font-size: 11.5px;
    color: var(--text-dim);
    text-align: center;
  }

  .file-input-raw {
    display: none;
  }

  .remove-btn-overlay {
    position: absolute;
    bottom: 8px;
    right: 8px;
    font-size: 11.5px;
    padding: 6px 10px;
  }

  /* Media panel footer */
  .media-panel-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--border);
    background: var(--surface-alt);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .media-footer-status {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .footer-thumb {
    width: 44px;
    height: 44px;
    border-radius: 6px;
    object-fit: cover;
    border: 1px solid var(--border);
    flex-shrink: 0;
  }

  .empty-thumb-icon {
    background: var(--surface);
    border: 1px dashed var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
  }

  .footer-thumb-info {
    min-width: 0;
  }

  .footer-thumb-title {
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .footer-thumb-subtitle {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .uppercase-text {
    text-transform: uppercase;
  }
</style>
