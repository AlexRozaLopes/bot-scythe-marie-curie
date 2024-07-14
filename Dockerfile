FROM rust:slim-bullseye

WORKDIR /marie-curie

ARG DISCORD_TOKEN

ENV DISCORD_TOKEN=${DISCORD_TOKEN}

RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler cmake libopus-dev build-essential autoconf automake libtool m4 youtube-dl libssl-dev libasound2-dev libdbus-1-dev python3-pip && pip3 install -U yt-dlp

COPY . .

RUN cargo build --release

CMD ["./target/release/bot-scythe-marie-curie"]
