# Build Periphery
FROM rust:1.80.1-bookworm AS builder
WORKDIR /builder
COPY . .
RUN cargo build -p monitor_periphery --release

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git curl ca-certificates && \
	curl -fsSL https://get.docker.com | sh && \
	chmod +x /usr/local/bin/docker

# Copy
COPY ./config_example/periphery.config.example.toml /config/config.toml
COPY --from=builder /builder/target/release/periphery /

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/monitor
LABEL org.opencontainers.image.description="Monitor Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

ENTRYPOINT [ "./periphery" ]