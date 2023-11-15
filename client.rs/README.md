# HAPI Client

This project provides a cli for interacting with HAPI smart contracts, that serves Solana, EVM and NEAR blockchains.

---

## Build

```bash
cargo build
```

## Usage

### Commands:

| Command       | Description                                               |
| ------------- | --------------------------------------------------------- |
| authority     | Authority commands                                        |
| configuration | Configuration commands                                    |
| reporter      | Reporter commands                                         |
| case          | Case commands                                             |
| address       | Address commands                                          |
| asset         | Asset commands                                            |
| token         | Token operations                                          |
| help          | Print this message or the help of the given subcommand(s) |

### Subcommands:

1. Authority subcommands:

| Subcommand | Description               |
| ---------- | ------------------------- |
| get        | Get authority address     |
| set        | Set new authority address |

2. Configuration subcommands:

| Subcommand    | Description                 |
| ------------- | --------------------------- |
| get-stake     | Get stake configuration     |
| update-stake  | Update stake configuration  |
| get-reward    | Get reward configuration    |
| update-reward | Update reward configuration |

3. Reporter subcommands:

| Subcommand | Description         |
| ---------- | ------------------- |
| create     | Create reporter     |
| update     | Update reporter     |
| get        | Get reporter        |
| count      | Get reporter count  |
| list       | Get reporter list   |
| activate   | Activate reporter   |
| deactivate | Deactivate reporter |
| unstake    | Unstake reporter    |

4. Case subcommands:

| Subcommand | Description    |
| ---------- | -------------- |
| create     | Create case    |
| update     | Update case    |
| get        | Get case       |
| count      | Get case count |
| list       | Get case list  |

5. Address subcommands:

| Subcommand | Description       |
| ---------- | ----------------- |
| create     | Create address    |
| update     | Update address    |
| confirm    | Confirm address   |
| get        | Get address       |
| count      | Get address count |
| list       | Get address list  |

6. Asset subcommands:

| Subcommand | Description     |
| ---------- | --------------- |
| create     | Create asset    |
| update     | Update asset    |
| confirm    | Confirm address |
| get        | Get asset       |
| count      | Get asset count |
| list       | Get asset list  |

7. Token subcommands:

| Subcommand | Description             |
| ---------- | ----------------------- |
| transfer   | Transfer token          |
| approve    | Approve token allowance |
| balance    | Get token balance       |

### Options:

| Flag                                      | Description                                                                            |
| ----------------------------------------- | -------------------------------------------------------------------------------------- |
| -n, --network <NETWORK>                   | Network to use [env: NETWORK=] [possible values: ethereum, bsc, solana, bitcoin, near] |
| -p, --provider-url <PROVIDER_URL>         | Network-specific provider URL (e.g. RPC node URL) [env: PROVIDER_URL=]                 |
| -c, --contract-address <CONTRACT_ADDRESS> | Network-specific HAPI Core contract address [env: CONTRACT_ADDRESS=]                   |
| -k, --private-key <PRIVATE_KEY>           | Private key to sign transactions                                                       |
| --chain-id <CHAIN_ID>                     | [OPTIONAL] Chain ID for EVM-based networks [env: CHAIN_ID=]                            |
| --account-id <ACCOUNT_ID>                 | [OPTIONAL] Account ID for NEAR network [env: ACCOUNT_ID=]                              |
| -o, --output <OUTPUT>                     | [OPTIONAL] Command output format [env: OUTPUT=] [possible values: json, text]          |
| -h, --help                                | Print help                                                                             |

---

Run cli with:

```
cargo run
```

or:

```bash
/target/debug/hapi-core-cli
```

## Testing

Utils needed for testing: solana and anchor toolchains, docker, npm.
To test all features run with --all-features

```bash
cargo test
```

## License

HAPI client is distributed under the terms of the MIT license.
