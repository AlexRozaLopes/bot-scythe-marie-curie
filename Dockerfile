FROM rust:slim-bullseye

WORKDIR /marie-curie

ARG DISCORD_TOKEN

ENV DISCORD_TOKEN=${DISCORD_TOKEN}

RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler cmake

COPY . .

RUN cargo build --release

CMD ["./target/release/bot-scythe-marie-curie"]
