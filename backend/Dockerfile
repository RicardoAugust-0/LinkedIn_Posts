# Estágio de compilação
FROM rust:1.85-slim AS builder

WORKDIR /app

# Instalar dependências de build necessárias para compilação (OpenSSL)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar arquivos do projeto
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compilar para release
RUN cargo build --release

# Estágio de execução
FROM debian:bookworm-slim

WORKDIR /app

# Instalar certificados CA (para requisições HTTPS funcionarem) e OpenSSL runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copiar o binário gerado
COPY --from=builder /app/target/release/backend /app/backend

# Criar pasta para o volume de dados do SQLite
RUN mkdir -p /data

# Expor a porta que a aplicação escuta
EXPOSE 3000

# Rodar a aplicação
CMD ["/app/backend"]
