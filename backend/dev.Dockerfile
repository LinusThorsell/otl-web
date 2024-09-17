FROM rust:1.80.1-bullseye

WORKDIR /app

RUN apt update && apt upgrade -y
RUN apt install libpq-dev -y

RUN cargo install cargo-watch

COPY ./ ./

EXPOSE 8000

