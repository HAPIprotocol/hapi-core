# Use NEAR maintained image as a base
FROM nearprotocol/contract-builder:latest

LABEL description="Container for build hapi-core-near contract"

# Setup tooling
RUN rustup toolchain install stable-2023-04-20 && \
    rustup default stable-2023-04-20 && \
    rustup target add wasm32-unknown-unknown

# Define working directory (instead of root)
WORKDIR /builder

# Copy source code to the builder directory
COPY . /builder

# Define custom flags for Rust compiler
ENV RUSTFLAGS="-C link-arg=-s"

# Define custom cargo home directory
ENV CARGO_HOME=/var/cache/cargo

# Define a volume for built artifacts and dependency cache
VOLUME "/var/cache/cargo"

# Define a volume for the output artifacts
VOLUME "/output"

# Build the contract when the container is started
# Removing rlib first to reduce the size of wasm file
CMD perl -i -pe 's/\["cdylib", "rlib"\]/\["cdylib"\]/' ./contract/Cargo.toml && \
    cargo build --target wasm32-unknown-unknown --release && \
    cp target/wasm32-unknown-unknown/release/hapi_core_near.wasm /output/hapi_core_near.wasm
