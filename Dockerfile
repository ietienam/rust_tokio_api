# Use the official Rust image as the base
FROM rust:1.73 AS builder


# Set the working directory inside the container
WORKDIR /app


# Copy the Cargo manifest and source code into the container
COPY Cargo.toml .
COPY src ./src


# Build the dependencies to cache them
RUN cargo build --release


# Final stage: Use a smaller base image to run the app
FROM debian:buster-slim


# Install necessary dependencies for running the Rust binary
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*


# Set the working directory inside the container
WORKDIR /app


# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/rust_crud_api /app/rust_crud_api


# Set environment variables
ENV RUST_BACKTRACE=1


# Expose the port that Actix web will run on
EXPOSE 8080


# Run the Rust application
CMD ["./rust_crud_api"]