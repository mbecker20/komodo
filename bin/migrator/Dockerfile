# Build Core
FROM rust:1.79.0-bookworm as builder
WORKDIR /builder
COPY . .
RUN cargo build -p migrator --release

# Final Image
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /builder/target/release/migrator /

CMD ["./migrator"]