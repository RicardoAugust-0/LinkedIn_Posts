<script lang="ts">
  import { ThumbsUp, MessageSquare, Share2, Send, Globe, MoreHorizontal } from '@lucide/svelte';
  
  export let content: string = '';
  export let imageUrl: string | null = null;
  export let title: string = '';
  export let compact: boolean = false;
  export let imageSource: 'google' | 'ai' | 'upload' | 'none' = 'none';

  // Formatador para converter quebras de linha em HTML e destacar hashtags/menções com o estilo do Studio
  function formatContent(text: string) {
    if (!text) return 'O conteúdo do post aparecerá aqui...';
    
    // Escapar HTML básico para segurança
    let escaped = text
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");
      
    // Colorir hashtags (#Rust) com a cor cyan do tema
    escaped = escaped.replace(/(#[a-zA-Z0-9À-ÿ_]+)/g, '<span class="hashtag">$1</span>');
    
    // Colorir menções (@LinkedIn) com a cor cyan do tema
    escaped = escaped.replace(/(@[a-zA-Z0-9_]+)/g, '<span class="mention">$1</span>');
    
    // Preservar quebras de linha
    return escaped.replace(/\n/g, '<br>');
  }

  $: formattedHtml = formatContent(content);
</script>

<div class="feed-preview-card {compact ? 'compact' : ''}">
  <!-- Top Bar / Author Section -->
  <div class="author-section">
    <div class="avatar">
      <span class="avatar-text">RA</span>
    </div>
    <div class="author-info">
      <div class="author-name-row">
        <span class="author-name">Ricardo Augusto</span>
        <span class="author-badge">· Você</span>
      </div>
      <span class="author-headline">Engenheiro de Software · Escrevendo sobre sistemas distribuídos</span>
      <div class="post-time-row">
        <span class="post-time">agora</span>
        <span class="dot-separator">·</span>
        <Globe size={11} />
      </div>
    </div>
    <button class="more-btn" aria-label="Opções">
      <MoreHorizontal size={16} />
    </button>
  </div>

  <!-- Content Section -->
  <div class="post-content">
    <p class="post-text">{@html formattedHtml}</p>
  </div>

  <!-- Media Section -->
  {#if imageUrl && imageSource !== 'none'}
    <div class="post-media">
      <img 
        src={imageUrl.startsWith('/uploads') ? `http://localhost:3000${imageUrl}` : imageUrl} 
        alt={title || "Mídia do Post"} 
        class="media-image"
      />
    </div>
  {:else if imageSource === 'ai' || imageSource === 'google'}
    <!-- Se o post indica que tem imagem por IA/Google mas não carregou a URL ainda, mostramos o pattern placeholder -->
    <div class="post-media-placeholder">
      <svg width="100%" height="100%" preserveAspectRatio="none" class="placeholder-svg">
        <defs>
          <pattern id="pattern-preview" width="9" height="9" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
            <line x1="0" y1="0" x2="0" y2="9" stroke="var(--border-strong)" stroke-width="1" opacity="0.5"/>
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#pattern-preview)"/>
      </svg>
      <div class="placeholder-text">
        {imageSource === 'ai' ? 'GERANDO IMAGEM POR IA...' : 'BUSCANDO IMAGEM NO GOOGLE...'}
      </div>
    </div>
  {/if}

  <!-- Reactions Section -->
  <div class="reactions-section">
    <div class="reactions-left">
      <div class="reaction-dots">
        <span class="react-dot cyan-dot"></span>
        <span class="react-dot accent-dot"></span>
        <span class="react-dot amber-dot"></span>
      </div>
      <span class="reactions-count-text">Você e outras 42 pessoas</span>
    </div>
    <div class="reactions-right">
      <span>8 comentários</span>
      <span class="separator">·</span>
      <span>3 compart.</span>
    </div>
  </div>

  <!-- Action Buttons Section -->
  <div class="action-buttons">
    <button class="action-btn">
      <ThumbsUp size={15} />
      <span>Curtir</span>
    </button>
    <button class="action-btn">
      <MessageSquare size={15} />
      <span>Comentar</span>
    </button>
    <button class="action-btn">
      <Share2 size={15} />
      <span>Compartilhar</span>
    </button>
    <button class="action-btn">
      <Send size={15} />
      <span>Enviar</span>
    </button>
  </div>
</div>

<style>
  .feed-preview-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 10px;
    overflow: hidden;
    font-family: var(--font-main);
    color: var(--text);
    font-size: 13.5px;
    text-align: left;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    transition: background-color var(--transition-normal), border-color var(--transition-normal);
  }

  .feed-preview-card.compact {
    font-size: 13px;
  }

  .author-section {
    padding: 14px 16px 8px;
    display: flex;
    align-items: flex-start;
    gap: 12px;
    position: relative;
  }

  .avatar {
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--avatar-a), var(--avatar-b));
    color: #ffffff;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 15.84px;
    letter-spacing: -0.02em;
    flex-shrink: 0;
    box-shadow: inset 0 0 0 1px rgba(255,255,255,0.04);
  }

  :global(.theme-light) .avatar {
    color: #ffffff;
  }

  .avatar-text {
    line-height: 1;
  }

  .author-info {
    flex: 1;
    min-width: 0;
  }

  .author-name-row {
    font-weight: 600;
    font-size: 14px;
    line-height: 1.2;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .author-badge {
    font-weight: 400;
    color: var(--text-muted);
    font-size: 12px;
  }

  .author-headline {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 3px;
    line-height: 1.35;
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .post-time-row {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 3px;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .dot-separator {
    font-weight: bold;
  }

  .more-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--transition-fast);
  }

  .more-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }

  .post-content {
    padding: 4px 16px 12px;
  }

  .post-text {
    line-height: 1.55;
    white-space: pre-wrap;
    color: var(--text);
  }

  :global(.post-text .hashtag) {
    color: var(--cyan);
    font-weight: 500;
    cursor: pointer;
  }
  :global(.post-text .hashtag:hover) {
    text-decoration: underline;
  }

  :global(.post-text .mention) {
    color: var(--cyan);
    font-weight: 500;
  }

  /* Media Section */
  .post-media {
    width: 100%;
    max-height: 260px;
    background-color: var(--bg-inset);
    display: flex;
    align-items: center;
    justify-content: center;
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    overflow: hidden;
  }

  .feed-preview-card.compact .post-media {
    max-height: 180px;
  }

  .media-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* Media Placeholder Pattern */
  .post-media-placeholder {
    width: 100%;
    height: 260px;
    position: relative;
    background: var(--surface-alt);
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    overflow: hidden;
  }

  .feed-preview-card.compact .post-media-placeholder {
    height: 180px;
  }

  .placeholder-svg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .placeholder-text {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.14em;
    color: var(--text-dim);
    text-align: center;
    padding: 8px;
    text-shadow: 0 0 8px var(--bg-app);
  }

  /* Reactions Section */
  .reactions-section {
    padding: 10px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
    font-size: 12px;
    color: var(--text-muted);
  }

  .reactions-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .reaction-dots {
    display: flex;
    padding-left: 4px;
  }

  .react-dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid var(--surface);
    display: inline-block;
    margin-left: -4px;
  }

  .react-dot.cyan-dot {
    background-color: var(--cyan);
    z-index: 3;
    margin-left: 0;
  }
  .react-dot.accent-dot {
    background-color: var(--accent);
    z-index: 2;
  }
  .react-dot.amber-dot {
    background-color: var(--amber);
    z-index: 1;
  }

  .reactions-right {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .separator {
    color: var(--text-dim);
  }

  /* Action Buttons */
  .action-buttons {
    padding: 2px 4px;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
  }

  .action-btn {
    border: none;
    background: transparent;
    color: var(--text-muted);
    padding: 10px 6px;
    font-size: 12.5px;
    font-family: inherit;
    font-weight: 500;
    cursor: pointer;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .action-btn:hover {
    background-color: var(--surface-hover);
    color: var(--text);
  }
</style>
