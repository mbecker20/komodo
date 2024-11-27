## Builds the Komodo Core and Periphery binaries
## for a specific architecture.

# FROM rust:1.82.0-bullseye AS builder

# WORKDIR /builder
# COPY Cargo.toml Cargo.lock ./
# COPY ./lib ./lib
# COPY ./client/core/rs ./client/core/rs
# COPY ./client/periphery ./client/periphery
# COPY ./bin/core ./bin/core
# COPY ./bin/periphery ./bin/periphery

# # Compile bin
# RUN \
#   cargo build -p komodo_core --release && \
#   cargo build -p komodo_periphery --release

# # Copy just the binaries to scratch image
# FROM scratch

# COPY --from=builder /builder/target/release/core /core
# COPY --from=builder /builder/target/release/periphery /periphery

## Caching deps

FROM rust:1.82.0-bullseye AS builder

WORKDIR /builder

COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery

# Compile Core
COPY ./bin/core/Cargo.toml ./bin/core/Cargo.toml
RUN mkdir ./bin/core/src && \
  echo "fn main() {}" >> ./bin/core/src/main.rs && \
  cargo build -p komodo_core --release && \
  rm -r ./bin/core
COPY ./bin/core ./bin/core
RUN cargo build -p komodo_core --release

# Compile Periphery
COPY ./bin/periphery/Cargo.toml ./bin/periphery/Cargo.toml
RUN mkdir ./bin/periphery/src && \
  echo "fn main() {}" >> ./bin/periphery/src/main.rs && \
  cargo build -p komodo_periphery --release && \
  rm -r ./bin/periphery
COPY ./bin/periphery ./bin/periphery
RUN cargo build -p komodo_periphery --release

# Copy just the binaries to scratch image
FROM scratch

COPY --from=builder /builder/target/release/core /core
COPY --from=builder /builder/target/release/periphery /periphery