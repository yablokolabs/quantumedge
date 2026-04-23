FROM rust:1.88 as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/quantumedge-rust-api /usr/local/bin/quantumedge-rust-api
EXPOSE 8098
CMD ["quantumedge-rust-api"]
