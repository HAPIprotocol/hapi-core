# hapi-explorer

HAPI explorer multichain backend

## Running tests

```sh
cargo test -- --test-threads=1
```

## Running the server

```sh
cargo run server
```

## Creating a new indexer

This command will create a new indexer for the given network. The indexer will be added to the `indexers` table in the database.
Program, will log the indexer's jwt token to the console. This token should be used to create a new indexer client.

```sh
cargo run create-indexer --network=near
```

To use custom secret_phrase, set the `SECRET_PATH` environment variable to the path of the file containing the secret phrase. It must be .toml file with the following format:

```toml
jwt_secret="secret_phrase"
```

## License

HAPI explorer is distributed under the terms of the MIT license.
