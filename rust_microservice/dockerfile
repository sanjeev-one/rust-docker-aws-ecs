# Use an official Rust image
FROM rust:1.62 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy our manifests
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy our source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/rust_microservice*
RUN cargo build --release

# Final base
FROM debian:buster-slim

# Copy the build artifact from the build stage and set the working directory
COPY --from=builder /app/target/release/rust_microservice .

# Set the CMD to your binary
CMD ["./rust_microservice"]
