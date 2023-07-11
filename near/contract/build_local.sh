#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/hapi_core_near.wasm ./res/hapi_core_near_local.wasm
