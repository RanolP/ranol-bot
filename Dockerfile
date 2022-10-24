FROM --platform=${TARGETARCH} rust:alpine AS chef

RUN apk update
RUN apk add --no-cache musl-dev

WORKDIR /app
RUN cargo install cargo-chef@0.1.46

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:latest

COPY --from=builder \
    /app/target/release/ranol-bot \
    /usr/bin/ranol-bot

ENTRYPOINT [ "/usr/bin/ranol-bot" ]
