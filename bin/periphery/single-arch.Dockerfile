# Build Periphery for a specific architecture
FROM rust:1.82.0-bullseye AS builder

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery

# Pre compile dependencies
COPY ./bin/periphery/Cargo.toml ./bin/periphery/Cargo.toml
RUN mkdir ./bin/periphery/src && \
  echo "fn main() {}" >> ./bin/periphery/src/main.rs && \
  cargo build -p komodo_periphery --release && \
  rm -r ./bin/periphery
COPY ./bin/periphery ./bin/periphery

# Compile app
RUN cargo build -p komodo_periphery --release

# Final Image
FROM debian:bullseye-slim

# # Install Deps
COPY ./bin/periphery/debian-deps.sh .
RUN sh ./debian-deps.sh && rm ./debian-deps.sh

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