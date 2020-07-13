FROM rust:alpine3.10 as builder
RUN USER=root cargo init --name rust-intro
COPY Cargo.toml .
RUN cargo build --release
COPY src src
RUN cargo build --release
CMD ["app"]

FROM alpine:3.10
COPY --from=builder /target/release/rust-intro /bin/
CMD rust-intro
