FROM rust:1.84.1-slim-bullseye as builder
WORKDIR /usr/src/shrink
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /shrink
COPY --from=builder /usr/src/shrink/target/release/shrink .
ENTRYPOINT ["./shrink"]
