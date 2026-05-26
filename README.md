# LinkedIn AI Post Automator & Scheduler 🚀

Uma plataforma full-stack moderna projetada para portfólio, criada com um robusto backend em **Rust** e um frontend interativo e de alto desempenho em **Svelte**. O objetivo do projeto é automatizar e agendar posts no LinkedIn utilizando inteligência artificial para redação de texto, busca inteligente de imagens no Google e geração de imagens originais por I.A.

---

## 🛠️ Tecnologias Utilizadas

### Backend (Rust)
- **Axum**: Framework HTTP moderno, rápido e assíncrono.
- **Tokio**: Runtime assíncrono de nível de produção.
- **SQLx & SQLite**: Persistência de dados embarcada e leve para posts, credenciais e logs.
- **Reqwest**: Cliente HTTP para chamadas rápidas e assíncronas às APIs externas.
- **Cron-scheduler/Tokio Loops**: Executa verificações periódicas em background para publicar posts agendados automaticamente.

### Frontend (Svelte)
- **Svelte 5 / Vite**: Compilado, rápido e com reatividade fina (Signals).
- **TypeScript**: Tipagem estática para maior segurança e robustez do código.
- **Vanilla CSS (Glassmorphism)**: Design System exclusivo em Dark Mode, com efeitos de transparência ("glassmorphism"), gradientes de neon e micro-animações interativas.
- **Iconografia**: Lucide Icons via `@lucide/svelte`.

### I.A. & Busca
- **Gemini API (Google AI Studio)**:
  - **Geração de Texto**: Gemini 1.5 Flash (redação persuasiva do post do LinkedIn).
  - **Geração de Imagens**: Google Imagen 3 (criação de imagens exclusivas direto do prompt da I.A.).
- **Google Custom Search API**: Busca de imagens de referência e inspiração.

---

## 🌟 Funcionalidades Principais

1. **Dashboard & Métricas**: Acompanhamento de posts totais, rascunhos, agendados e publicados com gráficos e badges informativas.
2. **Assistente de Post (Wizard em 3 Etapas)**:
   - **Etapa 1**: Definição do tema e geração automática de texto com o Gemini.
   - **Etapa 2**: Edição do post em tempo real e adição de mídia (busca no Google ou geração com Imagen 3).
   - **Etapa 3**: Escolha de postagem imediata ou agendamento de data/hora.
3. **LinkedIn Live Preview**: Prévia do post idêntica ao feed real do LinkedIn (com hashtags azuis, layout do autor, imagem e botões de reação).
4. **Modo de Simulação (Mock Mode)**: Funciona perfeitamente de forma simulada se o usuário não inserir chaves de API. Ideal para demonstrações públicas em portfólios, permitindo testar toda a jornada sem vazar segredos ou quebrar integrações.

---

## 🚀 Como Executar o Projeto Localmente

### Pré-requisitos
- Ter o **Rust** instalado (rustc e cargo)
- Ter o **Node.js** (versão 18+) e **npm** instalados

### Passo 1: Executando o Backend (Rust)
1. Navegue para a pasta `backend/`:
   ```bash
   cd backend
   ```
2. Inicialize o servidor:
   ```bash
   cargo run
   ```
   - *Nota:* O banco de dados SQLite (`posts.db`) e o diretório de uploads serão criados automaticamente.
   - O backend rodará em `http://localhost:3000`.

### Passo 2: Executando o Frontend (Svelte)
1. Abra outro terminal e navegue para a pasta `frontend/`:
   ```bash
   cd frontend
   ```
2. Inicie o servidor de desenvolvimento:
   ```bash
   npm run dev
   ```
3. Abra `http://localhost:5173` no navegador.

---

## ⚙️ Configuração de APIs (Opcional)

Para transicionar do **Modo Simulação** para o **Modo Real**, abra a página de **Configurações** na plataforma e adicione suas chaves:
- **Gemini API Key**: Obtenha no [Google AI Studio](https://aistudio.google.com/).
- **Google Search Key & CX**: Configure uma ferramenta de busca programática no [Google Custom Search Engine](https://programmablesearchengine.google.com/).
- **LinkedIn Credentials**: Crie uma conta no Portal de Desenvolvedores do LinkedIn, crie um aplicativo e adicione `http://localhost:3000/api/auth/linkedin/callback` como URL de redirecionamento autorizada.
