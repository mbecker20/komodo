# Build Periphery
FROM rust:1.82.0-bullseye AS builder

# Install cross-compilation tools
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu

# Set environment for cross-compilation
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./bin/periphery ./bin/periphery
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery

# Build binaries for both architectures
RUN rustup target add x86_64-unknown-linux-gnu && \
  cargo build -p komodo_periphery --release --target x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu && \
  cargo build -p komodo_periphery --release --target aarch64-unknown-linux-gnu

# Final Image
FROM debian:bullseye-slim

# Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

# Setup an application directory
WORKDIR /app

COPY --from=builder /builder/target/x86_64-unknown-linux-gnu/release/periphery /usr/local/bin/periphery_amd64
COPY --from=builder /builder/target/aarch64-unknown-linux-gnu/release/periphery /usr/local/bin/periphery_arm64

# Copy the entrypoint script
COPY bin/periphery/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 8120

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Periphery"
LABEL org.opencontainers.image.licenses=GPL-3.0

ENTRYPOINT [ "entrypoint.sh" ]