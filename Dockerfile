# Use the Rust image as the builder stage
FROM rust:1.74-alpine as builder

# Install musl-dev to provide necessary development files
RUN apk add --no-cache musl-dev

# Create a new directory to work in
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY /Cargo.toml /Cargo.lock ./

# Copy the rest of the application source code
COPY ./ ./

# Build the actual application
RUN cargo build --release

# Start a new stage
FROM rust:1.74-alpine

# Set the working directory
WORKDIR /usr/src/app

# Copy only the necessary artifacts from the previous stage
COPY --from=builder /usr/src/app/target/release/ ./target/release/

ENV RUST_BACKTRACE=full
ENV DATABASE_URL=mysql://

# Set the entry point
CMD ["/usr/src/app/target/release/rust-rest-api-with-rocket"]