FROM rust:1.81-slim AS builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/app/backend
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend /app/backend
# If you have a .env file or rely on env vars, Render sets them

EXPOSE 8080
CMD ["./backend"]
