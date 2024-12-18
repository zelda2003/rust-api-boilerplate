# Stage 1: Build the application
FROM rust:1.73 as builder

# Set working directory
WORKDIR /usr/src/app

# Copy the Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY . .
RUN cargo build --release

# Stage 2: Create a smaller production image
FROM debian:buster-slim

# Install necessary libraries for the compiled binary
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set working directory in the container
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/rust-api-boilerplate /app/

# Copy .env file for configuration (optional)
COPY .env /app/.env

# Expose the application port
EXPOSE 8080

# Command to run the application
CMD ["./rust-api-boilerplate"]
