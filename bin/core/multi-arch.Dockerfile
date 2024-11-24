ARG REGISTRY_AND_NAMESPACE=ghcr.io/mbecker20

ARG X86_64_IMAGE=${REGISTRY_AND_NAMESPACE}/komodo:latest-x86_64 
ARG AARCH64_IMAGE=${REGISTRY_AND_NAMESPACE}/komodo:latest-aarch64

# This is required to work with COPY --from
FROM ${X86_64_IMAGE} AS x86_64
FROM ${AARCH64_IMAGE} AS aarch64

FROM debian:bullseye-slim

# Install Deps
RUN apt update && \
  apt install -y git ca-certificates && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

## Copy both binaries initially, but only keep appropriate one for the TARGETPLATFORM.
COPY --from=x86_64 /app/core /app/arch/linux/amd64
COPY --from=aarch64 /app/core /app/arch/linux/arm64
ARG TARGETPLATFORM
RUN mv /app/arch/${TARGETPLATFORM} /app/core && rm -r /app/arch

# Copy default config / static frontend / deno binary
COPY ./config/core.config.toml /config/config.toml
COPY --from=x86_64 /app/frontend /app/frontend
COPY --from=denoland/deno:bin /deno /usr/local/bin/deno

# Set $DENO_DIR and preload external Deno deps
ENV DENO_DIR=/action-cache/deno
RUN mkdir /action-cache && \
  cd /action-cache && \
  deno install jsr:@std/yaml jsr:@std/toml

# Hint at the port
EXPOSE 9120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Core"
LABEL org.opencontainers.image.licenses=GPL-3.0

ENTRYPOINT [ "/app/core" ]