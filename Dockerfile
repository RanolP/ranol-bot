FROM --platform=${TARGETARCH} rust:1.67.1-alpine3.17 AS chef

RUN apk update
RUN apk add --no-cache musl-dev

WORKDIR /app
RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo install cargo-chef --locked

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release

FROM --platform=${TARGETARCH} alpine:3.17

COPY --from=builder \
    /app/target/release/ranol-bot \
    /usr/bin/ranol-bot

ENTRYPOINT [ "/usr/bin/ranol-bot" ]
