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
RUN apt update && apt install -y ca-certificates
COPY ./config_example/core.config.example.toml /config/config.toml
COPY --from=core-builder /builder/target/release/core /
COPY --from=frontend-builder /builder/frontend/dist /frontend
EXPOSE 9000
CMD ["./core"]