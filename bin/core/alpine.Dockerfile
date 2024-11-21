## This one produces smaller images,
## but alpine uses `musl` instead of `glibc`.
## This makes it take longer / more resources to build,
## and may negatively affect runtime performance.

# Build Core
FROM rust:1.82.0-alpine AS core-builder
RUN apk update && apk --no-cache add musl-dev openssl-dev openssl-libs-static
WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./bin/core ./bin/core
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery
RUN cargo build -p komodo_core --release

# Build Frontend
FROM node:20.12-alpine AS frontend-builder
WORKDIR /builder
COPY ./frontend ./frontend
COPY ./client/core/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link komodo_client && yarn && yarn build

# Final Image
FROM alpine:3.20

# Install Deps
RUN apk update && apk add --no-cache --virtual .build-deps \
	openssl ca-certificates git git-lfs curl

# Setup an application directory
WORKDIR /app

# Copy
COPY ./config/core.config.toml /config/config.toml
COPY --from=core-builder /builder/target/release/core /app
COPY --from=frontend-builder /builder/frontend/dist /app/frontend
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

# Using ENTRYPOINT allows cli args to be passed, eg using "command" in docker compose.
ENTRYPOINT [ "/app/core" ]