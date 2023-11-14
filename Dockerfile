# Use an official Rust runtime as a parent image
FROM rust:1.73.0 as builder

# Set the working directory in the container to /usr/src/app
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Install musl-tools and OpenSSL for the musl target
RUN apt-get update && apt-get install -y musl-tools libssl-dev pkg-config

# Add the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Build the application in release mode for the musl target
RUN OPENSSL_DIR=/usr/local/musl/ \
    OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
    OPENSSL_INCLUDE_DIR=/usr/local/musl/include/ \
    PKG_CONFIG_PATH=/usr/local/musl/lib/pkgconfig/ \
    cargo build --release --target=x86_64-unknown-linux-musl

# Start a new build stage so that the final image does not contain the Rust toolchain
FROM scratch

# Copy the binary from the builder stage to the current stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/backend_wesleysoto_dev /usr/local/bin/

# Set the startup command to run your binary
CMD ["/usr/local/bin/backend_wesleysoto_dev"]
