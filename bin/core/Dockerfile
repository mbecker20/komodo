# Build Core
FROM rust:1.78.0-bookworm as core-builder
WORKDIR /builder
COPY . .
RUN cargo build -p monitor_core --release

# Build Frontend
FROM node:20.12-alpine as frontend-builder
WORKDIR /builder
COPY ./frontend ./frontend
COPY ./client/core/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link @monitor/client && yarn && yarn build

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git ca-certificates

# Copy
COPY ./config_example/core.config.example.toml /config/config.toml
COPY --from=core-builder /builder/target/release/core /
COPY --from=frontend-builder /builder/frontend/dist /frontend

# Hint at the port
EXPOSE 9000

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/monitor
LABEL org.opencontainers.image.description="A tool to build and deploy software across many servers"
LABEL org.opencontainers.image.licenses=GPL-3.0

CMD ["./core"]