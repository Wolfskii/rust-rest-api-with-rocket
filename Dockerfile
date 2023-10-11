# Use the Rust image as the builder stage
FROM rust as builder

# Create a new directory to work in
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY /Cargo.toml /Cargo.lock ./

# Create an empty dummy source file to satisfy the build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build a dummy application to cache dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm -r src

# Copy the rest of the application source code
COPY ./ ./

# Build the actual application
RUN cargo build --release

# Use the same Rust builder image for the final runtime image
FROM builder

# Set the entry point
CMD ["target/release/rust-rest-api-with-rocket"]
