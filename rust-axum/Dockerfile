FROM rust:latest as builder

RUN USER=root cargo new --bin axum_app
WORKDIR /axum_app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/axum_app*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /axum_app/target/release/axum_app .

CMD ["./axum_app"]