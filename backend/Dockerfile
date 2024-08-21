# DEV
# FROM rust:1.80.1-slim-bullseye
#
# WORKDIR /app
#
# RUN apt-get update
# RUN apt-get install postgresql -y
# RUN apt-get install libpq-dev -y
#
# RUN cargo install cargo-watch
#
# COPY ./ ./
#
# EXPOSE 8000

# PROD
FROM rust:1.80.1-slim-bullseye

WORKDIR /app

RUN apt update && \
    apt install -y libpq-dev && \
    update-ca-certificates

COPY ./ ./

RUN cargo build --release
