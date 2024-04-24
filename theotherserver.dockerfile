FROM rust:1 AS base

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

FROM base AS chef

RUN cargo binstall -y cargo-chef

FROM chef AS planner

WORKDIR /app
COPY . /app
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . /app
RUN cargo build --release --package theotherserver

FROM ubuntu:22.04 AS runtime

# non-root user:
RUN adduser --disabled-password appuser
RUN mkdir /app && chown appuser /app
USER appuser

COPY --from=builder /app/target/release/theotherserver /app/theotherserver
WORKDIR /app

CMD ["/app/theotherserver"]
