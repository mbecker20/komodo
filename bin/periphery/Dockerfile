# Build Periphery
FROM rust:1.80.1-bookworm AS builder
WORKDIR /builder
COPY . .
RUN cargo build -p komodo_periphery --release

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git curl ca-certificates && \
	curl -fsSL https://get.docker.com | sh

# Copy
COPY --from=builder /builder/target/release/periphery /

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "./periphery" ]