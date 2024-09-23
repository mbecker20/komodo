# Build Periphery
FROM rust:1.81.0-bookworm AS builder
WORKDIR /builder
COPY . .
RUN cargo build -p komodo_periphery --release

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git curl ca-certificates && \
	curl -fsSL https://get.docker.com | sh

# Setup an application directory
WORKDIR /app

# Copy
COPY --from=builder /builder/target/release/periphery /app

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "/app/periphery" ]