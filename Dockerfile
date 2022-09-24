FROM clux/muslrust:1.63.0-stable AS chef

WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner

COPY ./bot-any ./bot-any
COPY ./bot-any-telegram ./bot-any-telegram
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./bot-any ./bot-any
COPY ./bot-any-telegram ./bot-any-telegram
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release

FROM alpine:3.16.2

COPY --from=builder \
    /app/target/x86_64-unknown-linux-musl/release/ranol-bot \
    /usr/bin/ranol-bot

ENTRYPOINT [ "/usr/bin/ranol-bot" ]