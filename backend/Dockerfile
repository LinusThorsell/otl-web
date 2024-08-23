FROM --platform=linux/arm64 arm64v8/rust:1.80.1

WORKDIR /app

RUN apt update && \
    apt install -y libpq-dev && \
    update-ca-certificates

COPY ./ ./

RUN cargo build --jobs 8 --release

