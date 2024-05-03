FROM rust:1.71.1 as builder
WORKDIR /builder

COPY . .

RUN cargo build -p update_logger --release

FROM gcr.io/distroless/cc

COPY --from=builder /builder/target/release/update_logger /

EXPOSE 7000

CMD ["./update_logger"]