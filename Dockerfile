# Stage 1: Build
FROM rust:slim-bullseye AS builder

WORKDIR /marie-curie

# Instalar dependências para build
RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler cmake libopus-dev build-essential autoconf automake libtool m4 youtube-dl libssl-dev libasound2-dev libdbus-1-dev python3-pip && pip3 install -U yt-dlp

# Copiar o código-fonte para o container
COPY . .

# Compilar o projeto em modo release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Definir o token do Discord como argumento
ARG DISCORD_TOKEN
ENV DISCORD_TOKEN=${DISCORD_TOKEN}

WORKDIR /marie-curie

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler cmake libopus-dev build-essential autoconf automake libtool m4 youtube-dl libssl-dev libasound2-dev libdbus-1-dev python3-pip && pip3 install -U yt-dlp

# Copiar apenas o binário compilado do estágio anterior
COPY --from=builder /marie-curie/target/release/bot-scythe-marie-curie .

# Configurar o comando padrão
CMD ["./bot-scythe-marie-curie"]
