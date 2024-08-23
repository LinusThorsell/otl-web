FROM rust:1.80.1-slim-bullseye

WORKDIR /app

RUN apt update && \
    apt install -y wget

RUN wget https://github.com/LinusThorsell/otl-web/releases/download/v1.1/main
