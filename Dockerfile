FROM lukemathwalker/cargo-chef:latest-rust-1.56.1-alpine3.14 AS chef

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
RUN ls -la ./target/release

FROM gcr.io/distroless/base
WORKDIR app
COPY --from=builder /build/target/release/main /
ENTRYPOINT ["./main"]
