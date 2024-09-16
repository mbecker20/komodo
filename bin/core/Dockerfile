# Build Core
FROM rust:1.81.0-bookworm AS core-builder
WORKDIR /builder
COPY . .
RUN cargo build -p komodo_core --release

# Build Frontend
FROM node:20.12-alpine AS frontend-builder
WORKDIR /builder
COPY ./frontend ./frontend
COPY ./client/core/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd frontend && yarn link @komodo/client && yarn && yarn build

# Final Image
FROM debian:bookworm-slim

# Install Deps
RUN apt update && apt install -y git curl unzip ca-certificates && \
	curl -SL https://github.com/docker/compose/releases/download/v2.29.1/docker-compose-linux-x86_64 -o /usr/local/bin/docker-compose && \
	chmod +x /usr/local/bin/docker-compose && \
	curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
	unzip awscliv2.zip && \
	./aws/install
	
# Copy
COPY ./config/core.config.toml /config/config.toml
COPY --from=core-builder /builder/target/release/core /
COPY --from=frontend-builder /builder/frontend/dist /frontend

# Hint at the port
EXPOSE 9000

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Komodo Core"
LABEL org.opencontainers.image.licenses=GPL-3.0

CMD ["./core"]