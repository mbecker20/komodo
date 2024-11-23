# Build Periphery
FROM --platform=$BUILDPLATFORM rust:1.82.0-bullseye AS builder

ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  build-essential \
  gcc-aarch64-linux-gnu \
  gcc-x86-64-linux-gnu \
  cmake

RUN case "$TARGETPLATFORM" in \
  "linux/amd64") \
    echo "x86_64-unknown-linux-gnu" >> /RUSTTARGET && \
    echo "/" >> /PKG_CONFIG_SYSROOT_DIR && \
    echo "/usr/lib/x86_64-linux-gnu" >> /OPENSSL_DIR && \
    echo "gcc" >> /CC;; \
  "linux/arm64") \
    echo "aarch64-unknown-linux-gnu" >> /RUSTTARGET && \
    echo "/usr/aarch64-linux-gnu" >> /PKG_CONFIG_SYSROOT_DIR && \
    echo "/usr/aarch64-linux-gnu" >> /OPENSSL_DIR && \
    echo "aarch64-linux-gnu-gcc" >> /CC;; \
  *) echo "Unsupported TARGETPLATFORM=$TARGETPLATFORM" && exit 1;; \
  esac



WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery

ENV PKG_CONFIG_SYSROOT_DIR=/

# Pre compile dependencies
COPY ./bin/periphery/Cargo.toml ./bin/periphery/Cargo.toml
RUN mkdir ./bin/periphery/src && \
  echo "fn main() {}" >> ./bin/periphery/src/main.rs && \
  rustup target add $(cat /RUSTTARGET) && \
  CC=$(cat /CC) PKG_CONFIG_SYSROOT_DIR=$(cat /PKG_CONFIG_SYSROOT_DIR) OPENSSL_DIR=$(cat /OPENSSL_DIR)  \
    cargo build -p komodo_periphery --release --target $(cat /RUSTTARGET) && \
  rm -r ./bin/periphery
COPY ./bin/periphery ./bin/periphery

RUN CC=$(cat /CC) PKG_CONFIG_SYSROOT_DIR=$(cat /PKG_CONFIG_SYSROOT_DIR) OPENSSL_DIR=$(cat /OPENSSL_DIR)  \
  cargo build -p komodo_periphery --release --target $(cat /RUSTTARGET) && \
  cp ./target/$(cat /RUSTTARGET)/release/periphery .

# Final Image
FROM --platform=$BUILDPLATFORM debian:bullseye-slim

# # Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

# Setup an application directory
WORKDIR /app

# Copy
COPY --from=builder /builder/periphery /app

# Hint at the port
EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "/app/periphery" ]