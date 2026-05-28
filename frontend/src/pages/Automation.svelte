<!-- frontend/src/pages/Automation.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    CheckCircle2, AlertCircle
  } from '@lucide/svelte';
  import { API_URL } from '../lib/api';
  import AutomationActiveBanner from '../components/automation/AutomationActiveBanner.svelte';
  import AutomationCampaignForm from '../components/automation/AutomationCampaignForm.svelte';
  import AutomationQueueTable from '../components/automation/AutomationQueueTable.svelte';

  // State configurations
  let topicSeed = 'Engenharia de software pragmática: Rust, edge runtimes, liderança técnica.';
  let quantity = 10;
  let cadence = 'daily';
  let windows = ['morning'];
  let tone = '';
  let generating = false;

  // Campaign State
  let campaignActive = false;
  let campaignName = '';
  let campaignNextIn = '';

  // Feedback Toasts local
  let showSuccessToast = false;
  let successToastMsg = "";
  let showErrorToast = false;
  let errorToastMsg = "";

  const cadenceOptions = [
    { id: 'daily',      label: '1× / dia',      desc: 'Uma publicação por dia' },
    { id: 'twice',      label: '2× / dia',      desc: 'Manhã e tarde' },
    { id: 'alt',        label: 'A cada 2 dias', desc: 'Espaçamento maior' },
    { id: 'weekly',     label: 'Semanal',       desc: 'Toda terça às 09h' },
  ];

  const windowOptions = [
    { id: 'morning',   label: 'Manhã',   range: '07–10h' },
    { id: 'lunch',     label: 'Almoço',  range: '12–13h' },
    { id: 'afternoon', label: 'Tarde',   range: '14–17h' },
    { id: 'evening',   label: 'Noite',   range: '19–21h' },
  ];

  const tonePresets = [
    { id: 'casual',    label: 'Casual técnico',         value: 'Casual, conversa de engenheiro. Linguagem direta, exemplos práticos, sem jargão de marketing.' },
    { id: 'checklist', label: 'Pragmático com checklist', value: 'Tom pragmático. Sempre incluir checklist no final. Foco no que funciona em produção.' },
    { id: 'story',     label: 'Storytelling de bastidores', value: 'Storytelling de bastidores. Começar com um problema real, mostrar a investigação, fechar com o academicismo.' },
    { id: 'opinion',   label: 'Opinião forte e direta', value: 'Opinião forte e direta. Tomar lado. Defender a posição com 2 a 3 argumentos concretos.' },
    { id: 'tutorial',  label: 'Tutorial passo-a-passo', value: 'Tutorial passo-a-passo. Numerado. Cada passo com 1 a 2 frases. Trechos de código quando fizer sentido.' },
  ];

  let queue: any[] = [];
  let expandedId: string | null = null;

  onMount(async () => {
    await loadSettings();
    await fetchPosts();
  });

  async function loadSettings() {
    try {
      const res = await fetch(`${API_URL}/api/settings`);
      if (res.ok) {
        const settings = await res.json();
        campaignActive = settings.campaign_active;
        topicSeed = settings.campaign_topic || topicSeed;
        quantity = settings.campaign_quantity || quantity;
        cadence = settings.campaign_cadence || cadence;
        windows = settings.campaign_windows ? settings.campaign_windows.split(',') : windows;
        tone = settings.campaign_tone || tone;
      }
    } catch (e) {
      console.error("Erro ao carregar configurações de campanha", e);
    }
  }

  async function saveCampaignSettings() {
    try {
      await fetch(`${API_URL}/api/settings`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          campaign_active: campaignActive,
          campaign_topic: topicSeed,
          campaign_quantity: quantity,
          campaign_cadence: cadence,
          campaign_windows: windows.join(','),
          campaign_tone: tone
        })
      });
    } catch (e) {
      console.error("Erro ao salvar configurações de campanha", e);
    }
  }

  async function fetchPosts() {
    try {
      const res = await fetch(`${API_URL}/api/posts?automated=true`);
      if (res.ok) {
        const posts = await res.json();
        queue = posts.map((p: any) => ({
          id: p.id,
          title: p.title,
          status: p.status,
          scheduled: p.scheduled_at ? formatDateTime(p.scheduled_at) : formatDateTime(p.created_at),
          imageSource: p.image_source || 'none',
          snippet: p.content
        }));
        
        if (campaignActive && queue.length > 0) {
          const firstPost = posts[0];
          if (firstPost && firstPost.topic) {
            campaignName = `Campanha: ${firstPost.topic}`;
          } else {
            campaignName = 'Campanha Ativa';
          }
          
          // Simular hora de envio
          const minutes = 22 + Math.floor(Math.random() * 30);
          campaignNextIn = `${minutes} min`;
        } else if (campaignActive) {
          campaignName = 'Esteira Ativa (Vazia)';
          campaignNextIn = '—';
        } else {
          campaignName = '';
          campaignNextIn = '';
        }
      }
    } catch (e) {
      console.error(e);
    }
  }

  function triggerSuccessToast(msg: string) {
    successToastMsg = msg;
    showSuccessToast = true;
    setTimeout(() => {
      showSuccessToast = false;
    }, 4000);
  }

  function triggerErrorToast(msg: string) {
    errorToastMsg = msg;
    showErrorToast = true;
    setTimeout(() => {
      showErrorToast = false;
    }, 4000);
  }

  function formatDateTime(isoString: string) {
    const date = new Date(isoString);
    const dd = String(date.getDate()).padStart(2, '0');
    const mm = String(date.getMonth() + 1).padStart(2, '0');
    const hh = String(date.getHours()).padStart(2, '0');
    const mi = String(date.getMinutes()).padStart(2, '0');
    return `${dd}/${mm} às ${hh}:${mi}`;
  }

  async function generateCampaign() {
    generating = true;
    try {
      // 1. Chamar o backend para gerar os tópicos
      const topicsRes = await fetch(`${API_URL}/api/generate/topics?seed=${encodeURIComponent(topicSeed)}&quantity=${quantity}`);
      if (!topicsRes.ok) throw new Error("Falha ao sugerir tópicos de IA.");
      const topics = await topicsRes.json();

      // Pegar até a quantidade solicitada
      const targetTopics = topics.slice(0, quantity);

      // Limpar posts antigos se for campanha nova
      const clearRes = await fetch(`${API_URL}/api/posts?automated=true`, { method: 'DELETE' });
      if (!clearRes.ok) throw new Error("Falha ao reiniciar banco de dados.");

      // 2. Para cada tópico, gerar o texto do post e a imagem correspondente
      let generatedCount = 0;
      for (const topic of targetTopics) {
        // Chamar o backend para gerar o texto do post
        const textRes = await fetch(`${API_URL}/api/generate/text`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            topic: topic,
            prompt_override: tone ? `Escreva no tom: ${tone}` : null
          })
        });

        if (textRes.ok) {
          const generatedData = await textRes.json();
          
          // Gerar imagem com IA para o post
          let imageUrl: string | null = null;
          let imageSource = 'none';
          try {
            const imgPrompt = `Professional futuristic technology illustration depicting ${topic}, 3d render digital art, corporate color palette, clean vector style`;
            const imageRes = await fetch(`${API_URL}/api/generate/image`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ prompt: imgPrompt })
            });
            if (imageRes.ok) {
              const imgData = await imageRes.json();
              imageUrl = imgData.image_url;
              imageSource = 'ai';
            }
          } catch (imgErr) {
            console.error("Falha ao gerar imagem para o tópico: " + topic, imgErr);
          }
          
          // Criar rascunho no banco com o texto e imagem gerados
          const createRes = await fetch(`${API_URL}/api/posts`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              title: generatedData.title,
              topic: topic,
              content: generatedData.content,
              image_url: imageUrl,
              image_source: imageSource,
              status: 'draft',
              scheduled_at: null,
              is_automated: true
            })
          });

          if (createRes.ok) {
            generatedCount++;
          }
        }
      }

      // Ativar e salvar estado da campanha no banco
      campaignActive = true;
      await saveCampaignSettings();

      triggerSuccessToast(`Campanha gerada com sucesso! ${generatedCount} posts programados.`);
      await fetchPosts();
    } catch (e: any) {
      console.error(e);
      triggerErrorToast(e.message || "Falha ao gerar a campanha com IA.");
    } finally {
      generating = false;
    }
  }

  async function toggleCampaignState() {
    campaignActive = !campaignActive;
    await saveCampaignSettings();
    if (campaignActive) {
      triggerSuccessToast("Automação de publicações retomada com sucesso.");
    } else {
      triggerSuccessToast("Automação pausada temporariamente.");
    }
    await fetchPosts();
  }

  async function cancelCampaign() {
    if (!confirm("Deseja realmente cancelar toda a esteira de publicações? Todos os posts serão excluídos.")) return;
    
    try {
      const clearRes = await fetch(`${API_URL}/api/posts?automated=true`, { method: 'DELETE' });
      if (clearRes.ok) {
        queue = [];
        campaignActive = false;
        campaignName = '';
        await saveCampaignSettings();
        triggerSuccessToast("Campanha e esteira canceladas com sucesso.");
      }
    } catch (e) {
      console.error(e);
      triggerErrorToast("Erro ao cancelar campanha.");
    }
  }

  // Ações de itens individuais da Fila
  function handleEditPost(event: CustomEvent<any> | any) {
    const detail = event.detail !== undefined ? event.detail : event;
    const { id } = detail;
    triggerSuccessToast(`Redirecionando para editor manual do post #${id.substring(0, 4)}...`);
  }

  async function handleRegeneratePost(event: CustomEvent<any> | any) {
    const detail = event.detail !== undefined ? event.detail : event;
    const { id } = detail;
    triggerSuccessToast("Regenerando texto do post via Gemini...");
    
    // Obter o post atual para pegar o tópico
    const index = queue.findIndex(p => p.id === id);
    if (index === -1) return;
    
    try {
      const generateRes = await fetch(`${API_URL}/api/generate/text`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          topic: queue[index].title,
          prompt_override: tone ? `Regere no tom: ${tone}` : null
        })
      });

      if (generateRes.ok) {
        const data = await generateRes.json();
        // Atualizar rascunho com o texto regenerado
        await fetch(`${API_URL}/api/posts/${id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            title: data.title || queue[index].title,
            topic: queue[index].title,
            content: data.content,
            image_url: null,
            image_source: 'none',
            status: 'draft',
            scheduled_at: null,
            is_automated: true
          })
        });

        triggerSuccessToast("Publicação regenerada com sucesso!");
        await fetchPosts();
      }
    } catch (e) {
      console.error(e);
      triggerErrorToast("Falha ao regenerar publicação.");
    }
  }

  function handleSkipPost(event: CustomEvent<any> | any) {
    const detail = event.detail !== undefined ? event.detail : event;
    const { id } = detail;
    triggerSuccessToast(`Post #${id.substring(0, 4)} pulado. Próximo post adiantado.`);
  }

  function handleChangeImage(event: CustomEvent<string> | any) {
    const id = event.detail !== undefined ? event.detail : event;
    triggerSuccessToast(`Buscando nova imagem para o post #${id.substring(0, 4)}...`);
  }

  function handleReschedule(event: CustomEvent<string> | any) {
    const id = event.detail !== undefined ? event.detail : event;
    triggerSuccessToast(`Reagendando post #${id.substring(0, 4)}...`);
  }

  async function handleRemovePost(event: CustomEvent<string> | any) {
    const id = event.detail !== undefined ? event.detail : event;
    if (!confirm("Excluir esta publicação da fila de automação?")) return;

    try {
      const res = await fetch(`${API_URL}/api/posts/${id}`, {
        method: 'DELETE'
      });
      if (res.ok) {
        triggerSuccessToast("Publicação removida com sucesso.");
        await fetchPosts();
      }
    } catch (e) {
      console.error(e);
      triggerErrorToast("Falha ao remover publicação.");
    }
  }
</script>

<div class="studio-page-header">
  <div class="studio-page-header-info">
    <div class="studio-eyebrow"> LinkedMaker Automação </div>
    <h1>Esteira de Conteúdo com I.A</h1>
    <p class="studio-subtitle">
      Configure diretrizes e cadência para que a Inteligência Artificial planeje, crie e publique posts recorrentemente.
    </p>
  </div>
</div>

<!-- Toast Notifications -->
<div class="studio-toast-container">
  {#if showSuccessToast}
    <div class="studio-toast studio-toast-success">
      <CheckCircle2 size={16} />
      <span>{successToastMsg}</span>
    </div>
  {/if}
  {#if showErrorToast}
    <div class="studio-toast studio-toast-error">
      <AlertCircle size={16} />
      <span>{errorToastMsg}</span>
    </div>
  {/if}
</div>

<div class="studio-page-body">
  <!-- Campanha Ativa Banner -->
  <AutomationActiveBanner 
    {campaignActive} 
    {campaignName} 
    {campaignNextIn} 
    {queue} 
    on:toggleActive={toggleCampaignState} 
    on:cancel={cancelCampaign} 
  />

  <div class="config-grid">
    <!-- Left Column: Inputs Config -->
    <AutomationCampaignForm 
      bind:topicSeed 
      bind:quantity 
      bind:cadence 
      bind:windows 
      bind:tone 
      {generating} 
      {cadenceOptions} 
      {windowOptions} 
      {tonePresets} 
      on:generate={generateCampaign} 
    />

    <!-- Right Column: Queue List timeline -->
    <AutomationQueueTable 
      {queue} 
      bind:expandedId 
      {topicSeed} 
      {tone} 
      on:edit={handleEditPost} 
      on:regenerate={handleRegeneratePost} 
      on:skip={handleSkipPost} 
      on:changeImage={handleChangeImage} 
      on:reschedule={handleReschedule} 
      on:remove={handleRemovePost} 
    />
  </div>
</div>

<style>
  .studio-page-header {
    padding: 24px 32px 22px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-app);
  }

  .studio-page-header h1 {
    font-size: 1.875rem;
    font-weight: 600;
    letter-spacing: -0.03em;
    line-height: 1.15;
    color: var(--text);
  }

  .studio-eyebrow {
    font-family: var(--font-mono);
    font-size: 11.5px;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 4px;
  }

  .studio-subtitle {
    font-size: 13.5px;
    color: var(--text-muted);
    margin-top: 6px;
    max-width: 600px;
    line-height: 1.45;
  }

  /* Page Body */
  .studio-page-body {
    flex: 1;
    padding: 24px 32px 28px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    min-height: 0;
  }

  /* Config Grid Layout */
  .config-grid {
    display: grid;
    grid-template-columns: 1.4fr 2fr;
    gap: 20px;
  }

  @media (max-width: 1024px) {
    .config-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
