# HAPI indexer

This project provides a transaction indexing application for HAPI smart contracts, that serves Solana, EVM and NEAR blockchains.

---

State machine of the indexer:

1. Initialize application: load persisted application state from a file (or database).
2. Check if something has been added to the contract.
3. Process received events and submit new data to output endpoint.
4. Wait for the updates.

---

## Usage

It is required to set indexer configuration before start. Define config file in CONFIG_PATH env variable.
Configuration file must contain fields:

```toml

log_level                           # Tracing level
is_json_logging                     # Tracing format
listener                            # Address for the listener server

[indexer]
    network                         # Indexed network [Sepolia, Ethereum, Bsc, Solana, Bitcoin, Near]
    rpc_node_url                    # HTTP URL of the rpc node for the network
    webhook_url                     # HTTP URL of the webhook server
    contract_address                # The HAPI Core contract address
    wait_interval_ms                # Timeout in milliseconds between wait checks (default 1000 millis)
    state_file                      # The file to persist the indexer state in (default data/state.json)

```

Run indexer with:

```
cargo run
```

## Testing

To enable indexer tracing in tests, set the ENABLE_TRACING env variable to 1

```
cargo test
```

## Manual testing

Steps:

1. Prepare the environment for the selected network: run validator, deploy contract and create test data (all instructions are in the contact directory).
2. Start a listener server for webhooks (a simple listener server is available in tests, run `cargo test run_simple_listener -- --nocapture`).
3. Set indexer configuration and define cfg path in CONFIG_PATH env variable.
4. Run indexer with ` cargo run` command.
5. Compare the resulting payload with the test data

## License

HAPI indexer is distributed under the terms of the MIT license.
