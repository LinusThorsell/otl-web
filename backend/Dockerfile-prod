FROM rust:1.80.1-slim-bullseye

WORKDIR /app

RUN apt update && \
    apt install -y libpq-dev && \
    update-ca-certificates

COPY ./ ./

RUN cargo build --release
