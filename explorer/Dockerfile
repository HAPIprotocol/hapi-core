# Builder image
FROM rust:slim AS builder

# Install OpenSSL
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables to enable static linking for OpenSSL with Rust
ENV OPENSSL_STATIC=true
ENV PKG_CONFIG_ALLOW_CROSS=1

# Copy source code
COPY client.rs /usr/src/hapi-core/client.rs
COPY explorer /usr/src/hapi-core/explorer
COPY indexer /usr/src/hapi-core/indexer
COPY solana /usr/src/hapi-core/solana
COPY near /usr/src/hapi-core/near

# Set working directory
WORKDIR /usr/src/hapi-core/explorer

# Build from source code
RUN cargo build --release --locked

# # Runtime image
FROM scratch AS runtime

# # Copy the built binary
COPY --from=builder /usr/src/hapi-core/explorer/target/release/hapi-explorer /bin/hapi-explorer

# # Make it the default command
ENTRYPOINT ["/bin/hapi-explorer"]
