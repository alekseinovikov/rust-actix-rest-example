FROM rust:1.65 AS builder 
COPY . .
RUN cargo clean
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /target/release/rust-actix-rest-example /
EXPOSE 8080
CMD ["./rust-actix-rest-example"]
