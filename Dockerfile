FROM --platform=${TARGETARCH} rust:1.64.0-alpine3.16 AS chef

RUN apk update
RUN apk add --no-cache musl-dev

WORKDIR /app
RUN cargo install cargo-chef@0.1.46 --locked

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:3.16.2

COPY --from=builder \
    /app/target/release/ranol-bot \
    /usr/bin/ranol-bot

ENTRYPOINT [ "/usr/bin/ranol-bot" ]
