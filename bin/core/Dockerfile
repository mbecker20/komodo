FROM rust:latest as builder
WORKDIR /builder

COPY . .

RUN cd bin/core && cargo build --release

FROM gcr.io/distroless/cc

# COPY ./frontend/build /frontend

COPY --from=builder /builder/target/release/core /

EXPOSE 9000

CMD ["./core"]