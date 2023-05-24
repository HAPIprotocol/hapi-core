#!/bin/bash

set -e

KEYPAIR=./tests/lib/test_keypair.json

exception() {
  echo "Error: $1"
  exit 1
}

run_validator() {
  echo "==> Running solana test validator"
  solana-test-validator -r 1> /dev/null &
  VALIDATOR_PID=$!
  echo "==> Solana-test-validator PID: $VALIDATOR_PID"
}

test() {
  sleep 5 
  echo "==> Deploying program to test validator and running tests"
  (anchor deploy --program-keypair $KEYPAIR 1> /dev/null && anchor test --skip-local-validator) || exception "Failed to run tests"
}

cleanup() {
  echo "==> Test validator shut down"
  kill -9 $VALIDATOR_PID
}

run_validator
test
cleanup
