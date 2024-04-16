# Stage 1: Build the application
# Use a specific version of the Rust image to ensure compatibility
FROM rust:1.58 as builder

# Create a new empty shell project
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock to cache the dependencies
COPY ./Cargo.toml ./Cargo.lock ./

# This step ensures that your dependencies are cached, speeding up subsequent builds
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    cargo clean -p myapp

# Now copy your actual source code files into the image
COPY ./src ./src

# Build your application for release
RUN cargo build --release

# Stage 2: Setup the runtime environment
# Use a minimal Debian-based image with only the essentials
FROM debian:buster-slim

# Install SSL certificates (necessary for many web applications)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/myapp/target/release/rust_microservice /usr/local/bin/rust_microservice

# Expose the port the server is listening on
EXPOSE 8080

# Command to run the executable
CMD ["rust_microservice"]
