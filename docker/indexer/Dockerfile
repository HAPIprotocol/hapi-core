# Builder image
FROM rust:1.75-slim AS builder

# Install OpenSSL
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY client.rs /usr/src/hapi-core/client.rs
COPY explorer /usr/src/hapi-core/explorer
COPY indexer /usr/src/hapi-core/indexer
COPY solana /usr/src/hapi-core/solana
COPY near /usr/src/hapi-core/near

# Set working directory
WORKDIR /usr/src/hapi-core/indexer

# Build from source code
RUN cargo build --release --locked

# Runtime image
FROM busybox AS runtime

# Copy the built binary
COPY --from=builder /usr/src/hapi-core/indexer/target/release/hapi-indexer /bin/hapi-indexer

# Copy libssl from the builder stage
COPY --from=builder /etc/ssl /etc/ssl
COPY --from=builder /usr/lib/ssl /usr/lib/ssl
COPY --from=builder \
    /lib/x86_64-linux-gnu/libssl.so \
    /lib/x86_64-linux-gnu/libssl.so.3 \
    /lib/x86_64-linux-gnu/libcrypto.so.3 \
    /lib/x86_64-linux-gnu/libgcc_s.so.1 \
    /lib/

# Make it the default command
CMD ["/bin/hapi-indexer"]
