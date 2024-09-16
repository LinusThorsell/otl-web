FROM rust:1.80.1-slim-bullseye

WORKDIR /app

RUN apt update && \
    apt install -y wget && \
    apt install -y libpq-dev

COPY ./ ./

RUN wget https://github.com/LinusThorsell/otl-web/releases/latest/download/main
RUN chmod +x ./main
