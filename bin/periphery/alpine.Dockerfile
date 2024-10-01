# Build Periphery
FROM rust:1.81.0-alpine AS builder
WORKDIR /builder
COPY . .
RUN apk update && apk --no-cache add musl-dev openssl-dev openssl-libs-static
RUN cargo build -p komodo_periphery --release

# Final Image
FROM alpine:3.20

# Install Deps
RUN apk update && apk add --no-cache --virtual .build-deps \
	docker-cli docker-cli-compose openssl ca-certificates git git-lfs bash

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