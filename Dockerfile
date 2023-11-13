# Using cargo-chef to speed up Docker builds
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
FROM chef AS planner
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release

# Run actual application
FROM public.ecr.aws/docker/library/rust:latest AS runtime
WORKDIR /app
EXPOSE 3000
COPY --from=builder /app/target/release/api /usr/local/bin
CMD ["/usr/local/bin/api"]