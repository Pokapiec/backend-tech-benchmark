FROM rust:1.75-slim as builder

RUN USER=root cargo new --bin axum_app
WORKDIR /axum_app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./.env ./.env
COPY ./.sqlx ./.sqlx

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/axum_app*
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=builder /axum_app/target/release/axum_app .

CMD ["./axum_app"]