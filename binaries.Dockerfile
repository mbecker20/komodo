FROM rust:1.82.0-bullseye AS builder

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery

# Pre compile dependencies
COPY ./bin/core/Cargo.toml ./bin/core/Cargo.toml
COPY ./bin/periphery/Cargo.toml ./bin/periphery/Cargo.toml
RUN \
  mkdir ./bin/core/src && \
  echo "fn main() {}" >> ./bin/core/src/main.rs && \
  mkdir ./bin/periphery/src && \
  echo "fn main() {}" >> ./bin/periphery/src/main.rs && \
  cargo build --release && \
  rm -r ./bin/core && \
  rm -r ./bin/periphery

COPY ./bin/core ./bin/core
COPY ./bin/periphery ./bin/periphery

# Compile bin
RUN cargo build --release

# Copy just the binaries to scratch image
FROM scratch

COPY --from=builder /builder/target/release/core /app
COPY --from=builder /builder/target/release/periphery /app