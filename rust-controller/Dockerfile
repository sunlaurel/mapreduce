# ---------- Build Stage ----------
FROM rust:latest AS builder

RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y musl-tools
RUN cargo build --release --target aarch64-unknown-linux-musl

# ---------- Runtime Stage ----------
FROM scratch

WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/rust-controller .

CMD ["./rust-controller"]
