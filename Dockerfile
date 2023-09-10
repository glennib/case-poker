FROM rust:1.72.0 AS builder-base

RUN cargo install cargo-chef --locked


FROM builder-base AS planner

COPY ./server/src /var/server/src
COPY ./server/Cargo.toml /var/server/Cargo.toml
COPY ./server/Cargo.lock /var/server/Cargo.lock

WORKDIR /var/server
RUN cargo chef prepare --recipe-path recipe.json


FROM builder-base AS builder

WORKDIR /var/server
COPY --from=planner /var/server/recipe.json /var/server/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./server/src /var/server/src
COPY ./server/Cargo.toml /var/server/Cargo.toml
COPY ./server/Cargo.lock /var/server/Cargo.lock

RUN cargo build --release --bin server


FROM ubuntu:latest AS production-server
WORKDIR /app
COPY --from=builder /var/server/target/release/server /app/server
ENV RUST_LOG="info,server=debug"
CMD [ "./server" ]
