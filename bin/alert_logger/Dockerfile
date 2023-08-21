FROM rust:latest as builder
WORKDIR /builder

COPY . .

RUN cargo build -p alert_logger --release

FROM gcr.io/distroless/cc

# COPY ./frontend/build /frontend

COPY --from=builder /builder/target/release/alert_logger /

EXPOSE 7777

CMD ["./alert_logger"]