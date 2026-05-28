// frontend/src/lib/stores/postStore.ts
import { writable } from 'svelte/store';
import { API_URL } from '../api';

export interface PostState {
  step: number;
  topic: string;
  promptOverride: string;
  postTitle: string;
  postContent: string;
  imageSource: 'google' | 'ai' | 'upload' | 'none';
  selectedImageUrl: string | null;
  googleSearchQuery: string;
  googleImages: Array<{ title: string; link: string; thumbnail: string }>;
  aiImagePrompt: string;
  generatedImageUrl: string | null;
  scheduleOption: 'now' | 'schedule';
  scheduleDate: string;
  scheduleTime: string;
  generatingText: boolean;
  searchingImages: boolean;
  generatingImage: boolean;
  savingPost: boolean;
  errorMsg: string | null;
  successMsg: string | null;
  recentTopics: string[];
  editingPostId: string | null;
}

const initialState: PostState = {
  step: 1,
  topic: '',
  promptOverride: '',
  postTitle: '',
  postContent: '',
  imageSource: 'none',
  selectedImageUrl: null,
  googleSearchQuery: '',
  googleImages: [],
  aiImagePrompt: '',
  generatedImageUrl: null,
  scheduleOption: 'now',
  scheduleDate: '',
  scheduleTime: '',
  generatingText: false,
  searchingImages: false,
  generatingImage: false,
  savingPost: false,
  errorMsg: null,
  successMsg: null,
  recentTopics: [],
  editingPostId: null,
};

function createPostStore() {
  const LOCAL_STORAGE_KEY = 'quill-post-wizard-state';
  
  let persisted = { ...initialState };
  if (typeof window !== 'undefined') {
    try {
      const saved = localStorage.getItem(LOCAL_STORAGE_KEY);
      if (saved) {
        persisted = {
          ...initialState,
          ...JSON.parse(saved),
          generatingText: false,
          searchingImages: false,
          generatingImage: false,
          savingPost: false,
          errorMsg: null,
          successMsg: null
        };
      }
    } catch (e) {
      console.error(persisted);
    }
  }

  const { subscribe, set, update } = writable<PostState>(persisted);

  // Subscribe to changes and persist
  if (typeof window !== 'undefined') {
    subscribe(state => {
      try {
        localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(state));
      } catch (e) {
        console.error(e);
      }
    });
  }

  return {
    subscribe,
    reset: () => {
      if (typeof window !== 'undefined') {
        localStorage.removeItem(LOCAL_STORAGE_KEY);
      }
      set({ ...initialState });
    },
    loadPost: (post: any) => {
      let scheduleOption: 'now' | 'schedule' = 'now';
      let scheduleDate = '';
      let scheduleTime = '';
      if (post.scheduled_at) {
        scheduleOption = 'schedule';
        const dateObj = new Date(post.scheduled_at);
        const year = dateObj.getFullYear();
        const month = String(dateObj.getMonth() + 1).padStart(2, '0');
        const day = String(dateObj.getDate()).padStart(2, '0');
        scheduleDate = `${year}-${month}-${day}`;
        scheduleTime = `${String(dateObj.getHours()).padStart(2, '0')}:${String(dateObj.getMinutes()).padStart(2, '0')}`;
      }
      update(s => ({
        ...s,
        step: 2,
        topic: post.topic,
        postTitle: post.title,
        postContent: post.content,
        imageSource: post.image_source || 'none',
        selectedImageUrl: post.image_url,
        generatedImageUrl: post.image_source === 'ai' ? post.image_url : null,
        scheduleOption,
        scheduleDate,
        scheduleTime,
        editingPostId: post.id
      }));
    },
    setStep: (step: number) => update(s => ({ ...s, step })),
    setTopic: (topic: string) => update(s => ({ ...s, topic })),
    setPromptOverride: (promptOverride: string) => update(s => ({ ...s, promptOverride })),
    setPostTitle: (postTitle: string) => update(s => ({ ...s, postTitle })),
    setPostContent: (postContent: string) => update(s => ({ ...s, postContent })),
    setImageSource: (imageSource: 'google' | 'ai' | 'upload' | 'none') => update(s => ({ ...s, imageSource })),
    setSelectedImageUrl: (url: string | null) => update(s => ({ ...s, selectedImageUrl: url })),
    setGoogleSearchQuery: (query: string) => update(s => ({ ...s, googleSearchQuery: query })),
    setAiImagePrompt: (prompt: string) => update(s => ({ ...s, aiImagePrompt: prompt })),
    setScheduleOption: (option: 'now' | 'schedule') => update(s => ({ ...s, scheduleOption: option })),
    setScheduleDate: (date: string) => update(s => ({ ...s, scheduleDate: date })),
    setScheduleTime: (time: string) => update(s => ({ ...s, scheduleTime: time })),
    setError: (msg: string | null) => update(s => ({ ...s, errorMsg: msg })),
    setSuccess: (msg: string | null) => update(s => ({ ...s, successMsg: msg })),
    setRecentTopics: (topics: string[]) => update(s => ({ ...s, recentTopics: topics })),

    generateText: async () => {
      let currentTopic = '';
      let currentPromptOverride = '';

      update(s => {
        currentTopic = s.topic;
        currentPromptOverride = s.promptOverride;
        return { ...s, generatingText: true, errorMsg: null };
      });

      if (!currentTopic.trim()) {
        update(s => ({ ...s, generatingText: false, errorMsg: "Por favor, insira o tema principal do post." }));
        return;
      }

      try {
        const res = await fetch(`${API_URL}/api/generate/text`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            topic: currentTopic,
            prompt_override: currentPromptOverride || null
          })
        });

        if (res.ok) {
          const data = await res.json();
          update(s => ({
            ...s,
            postTitle: data.title,
            postContent: data.content,
            googleSearchQuery: currentTopic,
            aiImagePrompt: `Professional futuristic technology illustration depicting ${currentTopic}, 3d render digital art, corporate color palette, clean vector style`
          }));
        } else {
          const txt = await res.text();
          let parsed;
          try { parsed = JSON.parse(txt); } catch { parsed = null; }
          const errorMsg = parsed && parsed.error ? parsed.error : txt;
          update(s => ({ ...s, errorMsg: `Erro na geração: ${errorMsg}` }));
        }
      } catch (e) {
        update(s => ({ ...s, errorMsg: "Erro de conexão ao servidor backend." }));
      } finally {
        update(s => ({ ...s, generatingText: false }));
      }
    },

    searchImages: async () => {
      let currentQuery = '';
      update(s => {
        currentQuery = s.googleSearchQuery;
        return { ...s, searchingImages: true, errorMsg: null };
      });

      if (!currentQuery.trim()) {
        update(s => ({ ...s, searchingImages: false }));
        return;
      }

      try {
        const res = await fetch(`${API_URL}/api/search/images?query=${encodeURIComponent(currentQuery)}`);
        if (res.ok) {
          const images = await res.json();
          update(s => ({ ...s, googleImages: images, imageSource: 'google' }));
        } else {
          const txt = await res.text();
          let parsed;
          try { parsed = JSON.parse(txt); } catch { parsed = null; }
          const errorMsg = parsed && parsed.error ? parsed.error : txt;
          update(s => ({ ...s, errorMsg: `Erro na busca do Google: ${errorMsg}` }));
        }
      } catch (e) {
        update(s => ({ ...s, errorMsg: "Erro de conexão na busca de imagens." }));
      } finally {
        update(s => ({ ...s, searchingImages: false }));
      }
    },

    generateImage: async () => {
      let currentPrompt = '';
      update(s => {
        currentPrompt = s.aiImagePrompt;
        return { ...s, generatingImage: true, errorMsg: null };
      });

      if (!currentPrompt.trim()) {
        update(s => ({ ...s, generatingImage: false }));
        return;
      }

      try {
        const res = await fetch(`${API_URL}/api/generate/image`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ prompt: currentPrompt })
        });

        if (res.ok) {
          const data = await res.json();
          update(s => ({
            ...s,
            generatedImageUrl: data.image_url,
            selectedImageUrl: data.image_url,
            imageSource: 'ai'
          }));
        } else {
          const txt = await res.text();
          let parsed;
          try { parsed = JSON.parse(txt); } catch { parsed = null; }
          const errorMsg = parsed && parsed.error ? parsed.error : txt;
          update(s => ({ ...s, errorMsg: `Erro na criação da imagem: ${errorMsg}` }));
        }
      } catch (e) {
        update(s => ({ ...s, errorMsg: "Erro de conexão ao gerar imagem." }));
      } finally {
        update(s => ({ ...s, generatingImage: false }));
      }
    },

    savePost: async (publishImmediately = false, navigateToDashboard: () => void) => {
      let state: PostState | null = null;
      update(s => {
        state = s;
        return { ...s, savingPost: true, errorMsg: null, successMsg: null };
      });

      if (!state) return;
      const { postTitle, topic, postContent, selectedImageUrl, imageSource, scheduleOption, scheduleDate, scheduleTime, editingPostId } = state as PostState;

      if (!postTitle || !postContent) {
        update(s => ({ ...s, savingPost: false }));
        return;
      }

      let status = 'draft';
      let scheduledAt: string | null = null;

      if (publishImmediately) {
        status = 'draft';
      } else if (scheduleOption === 'schedule') {
        if (!scheduleDate || !scheduleTime) {
          update(s => ({ ...s, savingPost: false, errorMsg: "Por favor, configure a data e a hora do agendamento." }));
          return;
        }
        status = 'scheduled';
        scheduledAt = new Date(`${scheduleDate}T${scheduleTime}`).toISOString();
      }

      const url = editingPostId 
        ? `${API_URL}/api/posts/${editingPostId}`
        : `${API_URL}/api/posts`;
      const method = editingPostId ? 'PUT' : 'POST';

      try {
        const res = await fetch(url, {
          method,
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            title: postTitle,
            topic,
            content: postContent,
            image_url: selectedImageUrl,
            image_source: imageSource === 'upload' ? 'google' : imageSource,
            status,
            scheduled_at: scheduledAt,
            is_automated: false
          })
        });

        if (res.ok) {
          const savedPost = await res.json();
          
          if (publishImmediately) {
            const pubRes = await fetch(`${API_URL}/api/posts/${savedPost.id}/publish`, {
              method: 'POST'
            });
            const pubData = await pubRes.json();
            
            if (pubRes.ok && pubData.success) {
              update(s => ({ ...s, successMsg: "Post criado e publicado com sucesso no LinkedIn!" }));
              setTimeout(() => {
                set({ ...initialState });
                navigateToDashboard();
              }, 1500);
            } else {
              const errorMsg = pubData.error || pubData.message || "Post salvo como rascunho, mas houve falha ao postar no LinkedIn.";
              update(s => ({ ...s, errorMsg }));
            }
          } else {
            const successMsg = scheduleOption === 'schedule' 
              ? `Post agendado com sucesso para ${scheduleDate} às ${scheduleTime}!`
              : "Post salvo com sucesso em seus rascunhos!";
            update(s => ({ ...s, successMsg }));
            setTimeout(() => {
              set({ ...initialState });
              navigateToDashboard();
            }, 1500);
          }
        } else {
          const txt = await res.text();
          let parsed;
          try { parsed = JSON.parse(txt); } catch { parsed = null; }
          const errorMsg = parsed && parsed.error ? parsed.error : txt;
          update(s => ({ ...s, errorMsg: `Erro ao salvar post: ${errorMsg}` }));
        }
      } catch (e) {
        update(s => ({ ...s, errorMsg: "Erro ao conectar ao servidor." }));
      } finally {
        update(s => ({ ...s, savingPost: false }));
      }
    }
  };
}

export const postStore = createPostStore();
