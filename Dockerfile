FROM --platform=${TARGETARCH} instrumentisto/rust:nightly-alpine3.15-2022-10-31 AS chef

RUN apk update
RUN apk add --no-cache musl-dev

WORKDIR /app
RUN cargo install cargo-chef@0.1.46 --locked -Z sparse-registry

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json -Z sparse-registry

COPY . .
RUN cargo build --release -Z sparse-registry

FROM --platform=${TARGETARCH} alpine:3.15

COPY --from=builder \
    /app/target/release/ranol-bot \
    /usr/bin/ranol-bot

ENTRYPOINT [ "/usr/bin/ranol-bot" ]
