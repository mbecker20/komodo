ARG REGISTRY_AND_NAMESPACE=ghcr.io/mbecker20

ARG X86_64_IMAGE=${REGISTRY_AND_NAMESPACE}/periphery:latest-x86_64 
ARG AARCH64_IMAGE=${REGISTRY_AND_NAMESPACE}/periphery:latest-aarch64

# This is required to work with COPY --from
FROM ${X86_64_IMAGE} AS x86_64
FROM ${AARCH64_IMAGE} AS aarch64

FROM debian:bullseye-slim

## Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

WORKDIR /app

## Copy both binaries initially, but only keep appropriate one for the TARGETPLATFORM.
COPY --from=x86_64 /app/periphery /app/arch/linux/amd64
COPY --from=aarch64 /app/periphery /app/arch/linux/arm64
ARG TARGETPLATFORM
RUN mv /app/arch/${TARGETPLATFORM} /app/periphery && rm -r /app/arch

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "/app/periphery" ]