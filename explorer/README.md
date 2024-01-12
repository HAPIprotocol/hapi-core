# hapi-explorer migrate

HAPI explorer multichain backend

---

## Setup

In order to be able to use this client, you will need to use Postgres database.
It is required to set explorer configuration before start. Define config file in CONFIG_PATH env variable.
Configuration file must contain fields:

```toml
log_level                           # Log level for the application layer, default: info
is_json_logging                     # Whether to use JSON logging, default: true
enable_metrics                      # Whether to enable metrics, default: true
listener                            # Address for the listener server
database_url                        # The database url
```

## Usage

To run cli in the repo root:

```sh
cargo run
```

or to run directly the binary:

```sh
cd ./target/debug && hapi-explorer
```

---

HAPI explorer cli includes the following commands:

| Command    | Description                                              |
| ---------- | -------------------------------------------------------- |
| server     | Runs HAPI Explorer multichain backend                    |
| network    | Contains a set of subcommands for for network management |
| migrations | Contains a set of subcommands for managing migrations    |
| help       | Display available commands                               |

### Running explorer server

To run HAPI Explorer multichain backend that will be handling client GraphQL requests:

```sh
hapi-explorer server
```

### Manage explorer migrations

To manage migrations for HAPI Explorer multichain backend run:

```sh
hapi-explorer migrate
```

with subcommands:

| Subommand         | Description                                                    |
| ----------------- | -------------------------------------------------------------- |
| fresh             | Drop all tables from the database, then reapply all migrations |
| refresh           | Rollback all applied migrations, then reapply all migrations   |
| reset             | Rollback all applied migrations                                |
| status            | Check the status of all migrations                             |
| up -n `<COUNT>`   | Apply pending migrations                                       |
| down -n `<COUNT>` | Rollback applied migrations                                    |

### Manage networks

- To create new network:

  ```sh
  hapi-explorer network create --id <ID> --name <NAME> --backend <BACKEND> --authority <AUTHORITY> --stake-token <STAKE_TOKEN> --chain-id <CHAIN_ID>
  ```

  (where chain-id is optional)

- To update existing network
  ```sh
  hapi-explorer network update [OPTIONS] --id <ID> --name <NAME> --authority <AUTHORITY> --stake-token <STAKE_TOKEN>
  ```
  (where name, authority and stake-token is optional)

---

Network options:

| Option        | Description                                                         |
| ------------- | ------------------------------------------------------------------- |
| --id          | Network string identifier                                           |
| --name        | Network display name                                                |
| --backend     | Network backend type: sepolia, ethereum, bsc, solana, bitcoin, near |
| --authority   | Network authority address                                           |
| --stake-token | Stake token contract address                                        |
| --chain-id    | Optional chain id                                                   |

## Running tests

Currently due to the peculiarities of test execution, the launch should take place in one thread:

```sh
cargo test -- --test-threads=1
```

## License

HAPI explorer is distributed under the terms of the MIT license.
