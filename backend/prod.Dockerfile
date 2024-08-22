FROM rust:1.80.1-slim-bullseye

WORKDIR /app

RUN apt update && \
    apt install -y curl

RUN curl wget https://github.com/LinusThorsell/otl-web/releases/download/v1.0/main
