ARG REGISTRY_AND_NAMESPACE=ghcr.io/mbecker20

ARG X86_64_IMAGE=${REGISTRY_AND_NAMESPACE}/periphery:latest-x86_64 
ARG AARCH64_IMAGE=${REGISTRY_AND_NAMESPACE}/periphery:latest-aarch64

FROM ${X86_64_IMAGE} AS x86_64
FROM ${AARCH64_IMAGE} AS aarch64

FROM debian:bullseye-slim

## Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

WORKDIR /app

COPY --from=x86_64 /app/peripher[y] /usr/local/bin/periphery-x86_64
COPY --from=aarch64 /app/peripher[y] /usr/local/bin/periphery-aarch64
RUN case "$TARGETPLATFORM" in \
  "linux/amd64") mv /usr/local/bin/periphery-x86_64 /usr/local/bin/periphery;; \
  "linux/arm64") mv /usr/local/bin/periphery-aarch64 /usr/local/bin/periphery;; \
  *) echo "Unsupported TARGETPLATFORM=$TARGETPLATFORM" && exit 1;; \
  esac

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "/app/periphery" ]