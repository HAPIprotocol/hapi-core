#!/bin/bash
set -e

printf "\033[0;34m=> [pre-build] formatting and clippy\n\033[0m"
cargo fmt && cargo clippy -- -D warnings

CONTAINER_NAME="hapi-core-near-builder"
CARGO_CACHE_VOLUME="$CONTAINER_NAME-cargo-cache"

# Get the directory path of the current script
SCRIPT_DIR=$(dirname "$0")

# Change the current directory to the script directory
cd "$SCRIPT_DIR"

# Make sure we have the latest version of the base image
docker build -t $CONTAINER_NAME .

# Check if the container is running
if [ "$(docker ps -q -f name=$CONTAINER_NAME)" ]; then
    # If the container is running, stop it.
    echo "Container $CONTAINER_NAME is running. Attempting to stop..."
    docker stop $CONTAINER_NAME
fi

# Check if the container exists
if [ "$(docker ps -aq -f status=exited -f name=$CONTAINER_NAME)" ]; then
    # If the container exists, but is stopped, remove it.
    echo "Container $CONTAINER_NAME exists, but is not running. Removing..."
    docker rm $CONTAINER_NAME
fi

# Run the builder and mount the ./res directory for artifact output
docker run \
    -it \
    -v $PWD/res:/output \
    --name $CONTAINER_NAME \
    --volume $CARGO_CACHE_VOLUME:/var/cache/cargo \
    $CONTAINER_NAME
