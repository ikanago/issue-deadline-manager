FROM rust:1.56 AS chef
RUN cargo install cargo-chef \
    && rm -rf "${CARGO_HOME}/registry"

FROM chef AS planner
WORKDIR plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR build
COPY --from=planner /plan/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/base
WORKDIR app
COPY --from=builder /build/target/release/main /
ENTRYPOINT ["./main"]
