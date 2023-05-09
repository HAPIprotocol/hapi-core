# HAPI Core migration

HAPI Core migration script

## Dependencies

To install everything you need to work with this project, you'll need to install dependencies as described in [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) documentation.

### Build

To build the script, you need to execute this command:

```sh
cargo build
```

### Configure

To run migration script set configuration before start.
You can copy `config.sample.yaml` to `config.yaml` to use the template in the repo root or define other file in HAPI_CFG env variable to initialize the config file with required fields:
```yaml
                                  # This is configuration parameters for to HAPI CORE migration

program_id: ""                    # The public key of the account containing a program
                                  # (optional, default - program id from the HAPI CORE crate)
environment: ""                   # Solana environment cluster (must be one of [localnet, 
                                  # testnet, mainnet, devnet] or be an http or https url, default - localnet)
keypair_path: ""                  # Reporter keypair path
communities:                      # HAPI CORE communities public keys 
    - id: 1                       # New community id
      pubkey: ""                  # Community pubkey

migrate_accounts: []              # Define what accounts should be migrated (optional, default - All)
                                  # Variants:
                                  #   - "Community"
                                  #   - "Network"
                                  #   - "Reporter"
                                  #   - "ReporterReward"
                                  #   - "Case"
                                  #   - "Address"
                                  #   - "Asset"
                                  # example: ["Community", "Reporter"] 
input_path: ""                    # Path to the file to store the migration list
```

### Run

To run the script, you need to execute this command:

```sh
cargo run
```