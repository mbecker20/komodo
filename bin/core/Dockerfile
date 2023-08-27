FROM rust:1.71.1 as builder
WORKDIR /builder

COPY . .

RUN cargo build -p monitor_core --release

FROM gcr.io/distroless/cc

# COPY ./frontend/build /frontend

COPY --from=builder /builder/target/release/core /

EXPOSE 9000

CMD ["./core"]