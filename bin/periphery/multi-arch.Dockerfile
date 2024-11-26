## Assumes the latest binaries for multiple x86_64 and aarch64 are already built (by binaries.Dockerfile).
## Sets up the necessary runtime container dependencies for Komodo Periphery.
## Since theres no heavy build here, QEMU multi-arch builds are fine for this image.

ARG REGISTRY_AND_NAMESPACE=ghcr.io/mbecker20
ARG IMAGE_TAG=latest
ARG X86_64_BINARIES=${REGISTRY_AND_NAMESPACE}/binaries:${IMAGE_TAG}-x86_64 
ARG AARCH64_BINARIES=${REGISTRY_AND_NAMESPACE}/binaries:${IMAGE_TAG}-aarch64

# This is required to work with COPY --from
FROM ${X86_64_BINARIES} AS x86_64
FROM ${AARCH64_BINARIES} AS aarch64

FROM debian:bullseye-slim

## Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

WORKDIR /app

## Copy both binaries initially, but only keep appropriate one for the TARGETPLATFORM.
COPY --from=x86_64 /periphery /app/arch/linux/amd64
COPY --from=aarch64 /periphery /app/arch/linux/arm64
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