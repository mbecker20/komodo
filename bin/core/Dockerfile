# Build Core
FROM rust:1.80.0-bookworm AS core-builder
WORKDIR /builder
COPY . .
RUN cargo build -p monitor_core --release

# Build Frontend
FROM node:20.12-alpine AS frontend-builder
WORKDIR /builder
COPY ./frontend ./frontend
COPY ./client/core/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link @monitor/client && yarn && yarn build

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git curl unzip ca-certificates && \
	curl -SL https://github.com/docker/compose/releases/download/v2.29.1/docker-compose-linux-x86_64 -o /usr/local/bin/docker-compose && \
	curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
	unzip awscliv2.zip && \
	./aws/install
	
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