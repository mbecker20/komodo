FROM rust:1.79.0-bookworm AS builder
WORKDIR /builder
COPY . .
RUN cargo build -p migrator --release

# Final Image
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /builder/target/release/migrator /

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/monitor
LABEL org.opencontainers.image.description="Database migrator for Monitor version upgrades"
LABEL org.opencontainers.image.licenses=GPL-3.0

CMD ["./migrator"]