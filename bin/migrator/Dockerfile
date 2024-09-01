FROM rust:1.80.1-bookworm AS builder
WORKDIR /builder
COPY . .
RUN cargo build -p migrator --release

# Final Image
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /builder/target/release/migrator /

# Label for Ghcr
LABEL org.opencontainers.image.source=https://github.com/mbecker20/komodo
LABEL org.opencontainers.image.description="Database migrator for Komodo version upgrades"
LABEL org.opencontainers.image.licenses=GPL-3.0

CMD ["./migrator"]