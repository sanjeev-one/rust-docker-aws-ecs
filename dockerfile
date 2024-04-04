FROM rust:1.58 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/local/cargo/bin/rust_microservice /usr/local/bin/rust_microservice
CMD ["rust_microservice"]
