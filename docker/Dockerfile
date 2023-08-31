# Use the rustlang/rust image as the base image for nightly Rust
FROM rustlang/rust:nightly as builder
LABEL version="1.74.0"


# Create a new directory to work in
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build a dummy application to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the rest of the application source code
COPY . .

# Build the actual application
RUN cargo build --release

# Use a smaller base image for the final runtime image
FROM debian:buster-slim

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/rust-rest-api-with-rocket /usr/local/bin/

# Set the entry point
CMD ["rust-rest-api-with-rocket"]
