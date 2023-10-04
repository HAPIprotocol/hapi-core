# HAPI Core - Solana

This is a HAPI Core contract for Solana blockchain.

## Dependencies

To install everything you need to work with this project, you'll need to install dependencies as described in [Anchor](https://www.anchor-lang.com/docs/installation) documentation.

## Build contract

To build the **hapi-core** program, you need to execute this command:

```sh

anchor build

```

## Test contract

To test the **hapi-core** program, you need to execute this command:

```sh

./tests/test_script.sh

```

## Local deployment

You should build the contract before proceeding

```sh
# 1. Run local node in a separate terminal
solana-test-validator -r

# 2. Deploy the contract with test keypair
anchor deploy --program-keypair ./tests/test_keypair.json --program-name hapi_core_solana

# 3. Get the deployed contract address from the output of deploy command
export CONTRACT_ADDRESS=FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk

```

## Testing with the Rust client

Repeat points 1 through 3 from "Local deployment" section to deploy the contract on a local node.

### Prepare accounts

```sh
# Make sure that dependencies are installed
npm install

# Create network for tests

export ANCHOR_WALLET=$(solana config get keypair | awk -F': ' '{print $2}' | tr -d '[:space:]')
export ANCHOR_PROVIDER_URL=http://localhost:8899
npm run create-network solana

# Create stake token
spl-token create-token

# Get the created stake token address from the output of previous command
export STAKE_TOKEN_ADDRESS=AcomSqAHNzJzHDnoSY9q9d99v9MHSUw1mF3TGJU9Kvz7

# Create reward token
spl-token create-token

# Get the created reward token address from the output of previous command
export REWARD_TOKEN_ADDRESS=7q4o7VL1ba8PWgBHhTEv85RHQiVZz3UgZ4ZznJ5aXrug

```

### Prepare client

Also, before testing, you need to build the evm and near parts

```sh
# Go to the client.rs directory
cd ../client.rs

# Build the client binary
cargo build --release

# Go to the release directory
cd target/release

# Check the client version
./hapi-core-cli --version

# Set up command environment variables (that can also be passed as arguments)
export NETWORK=solana
export PROVIDER_URL=http://127.0.0.1:8899/
```

### Contract authority

Get and set contract authority (aka owner).

```sh

export AUTHORITY_ADDR=QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ
export AUTHORITY_PK=AR6V6NmxBP1j4qiLjGDYvym5XzNPhg3TDCkRHG1qYhKZ

export PUBLISHER_ADDR=C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX
export PUBLISHER_PK=DT2ox7SDMVoSj5mTZSr9H4UhWMWahj6HN2w1BeEmhzjz

# Airdroping to reporters
solana airdrop 10 $AUTHORITY_ADDR
solana airdrop 10 $PUBLISHER_ADDR

# Check that it's our initial public key
./hapi-core-cli authority get

# Set it to another address
./hapi-core-cli authority set $PUBLISHER_ADDR

# Make sure it has been changed
./hapi-core-cli authority get

# Set it back
./hapi-core-cli authority set $AUTHORITY_ADDR --private-key $PUBLISHER_PK
```

### Contract configuration

Manipulate stake and reward configurations.

```sh
export PRIVATE_KEY=AR6V6NmxBP1j4qiLjGDYvym5XzNPhg3TDCkRHG1qYhKZ
export ADDRESS_CONFIRMATION_REWARD=1000000000 # 1e9
export ADDRESS_TRACER_REWARD=1000000000 # 1e9
export ASSET_CONFIRMATION_REWARD=1000000000 # 1e9
export ASSET_TRACER_REWARD=1000000000 # 1e9

# Should return an error, as we haven't configured it yet
./hapi-core-cli configuration get-reward

# Update settings
./hapi-core-cli configuration update-reward $REWARD_TOKEN_ADDRESS $ADDRESS_CONFIRMATION_REWARD \
$ADDRESS_TRACER_REWARD $ASSET_CONFIRMATION_REWARD $ASSET_TRACER_REWARD

# Make sure that reward configuration is now set
./hapi-core-cli configuration get-reward

export UNLOCK_DURATION=60 # 60 seconds
export VALIDATOR_STAKE=1000000000 # 1e9
export TRACER_STAKE=1000000000 # 1e9
export PUBLISHER_STAKE=1000000000 # 1e9
export AUTHORITY_STAKE=1000000000 # 1e9

# Should return an error, as we haven't configured it yet
./hapi-core-cli configuration get-stake

# Update settings
./hapi-core-cli configuration update-stake $STAKE_TOKEN_ADDRESS $UNLOCK_DURATION $VALIDATOR_STAKE \
$TRACER_STAKE $PUBLISHER_STAKE $AUTHORITY_STAKE

# Make sure that stake configuration is now set
./hapi-core-cli configuration get-stake
```

### Reporters

Create and activate a new reporter with authority role.

```sh
# Creating token accounts for reporters
spl-token create-account $STAKE_TOKEN_ADDRESS --owner $AUTHORITY_ADDR --fee-payer ~/.config/solana/id.json
spl-token create-account $STAKE_TOKEN_ADDRESS --owner $PUBLISHER_ADDR --fee-payer ~/.config/solana/id.json

# Minting to reporters accounts
spl-token mint $STAKE_TOKEN_ADDRESS 1000000000 --recipient-owner $AUTHORITY_ADDR
spl-token mint $STAKE_TOKEN_ADDRESS 1000000000 --recipient-owner $PUBLISHER_ADDR

# We'll need a UUID for our new reporter
export AUTHORITY_UUID="2163ddbf-cc88-409a-b7cf-7bc6a2ec4cd1"

# Create the reporter
./hapi-core-cli reporter create $AUTHORITY_UUID $AUTHORITY_ADDR authority "Authority reporter" "https://hapi.one/authority"

# Check on our new reporter, it's inactive
./hapi-core-cli reporter get $AUTHORITY_UUID

# Let's check that we have enough tokens for the stake
./hapi-core-cli token balance $STAKE_TOKEN_ADDRESS $AUTHORITY_ADDR

# Activate the reporter
./hapi-core-cli reporter activate

# Check the list of the reporters, now our reporter is active
./hapi-core-cli reporter list
```

Let's add another reporter with publisher role.

```sh
# Generate a UUID for the publisher
export PUBLISHER_UUID="2896e3be-c40d-4864-b035-4278ba19d4bd"

# Create the reporter
./hapi-core-cli reporter create $PUBLISHER_UUID $PUBLISHER_ADDR publisher "Publisher reporter" "https://hapi.one/publisher"

# Check that it has been added as inactive
./hapi-core-cli reporter get $PUBLISHER_UUID

# Let's transfer the stake amount of tokens from our authority to publisher's address
./hapi-core-cli token transfer $STAKE_TOKEN_ADDRESS $PUBLISHER_ADDR $PUBLISHER_STAKE

# Make sure that we now have enought tokens
./hapi-core-cli token balance $STAKE_TOKEN_ADDRESS $PUBLISHER_ADDR

# Activate the reporter, signed by the reporter
./hapi-core-cli reporter activate --private-key $PUBLISHER_PK

# Check the list of the reporters, both reporters should be active
./hapi-core-cli reporter list

# We can see that we now have 2 reporters
./hapi-core-cli reporter count
```

### Cases

We'll create a few cases.

```sh
export CASE_1_UUID="b414275b-1f1e-4083-b4dd-04dc41a9c301"
export CASE_2_UUID="4a044ed9-9e5e-4f8a-bbb2-0bf3c4fd7fb8"
export CASE_3_UUID="d4f41512-db3e-4306-9303-203ace5fe7e2"

# Create a general-purpose case
./hapi-core-cli case create $CASE_1_UUID "First case" "https://hapi.one/authority/1"

# Create a case by the publisher reporter
./hapi-core-cli case create $CASE_2_UUID "Second case" "https://hapi.one/publisher/2" --private-key $PUBLISHER_PK

# Make sure that publisher can't change authority's case
./hapi-core-cli case update $CASE_1_UUID "Modified first case" "https://hapi.one/publisher/1" open --private-key $PUBLISHER_PK

# See that nothing has changed
./hapi-core-cli case get $CASE_1_UUID

# Create a case that we'll then close
./hapi-core-cli case create $CASE_3_UUID "Third case" "https://hapi.one/authority/3"

# Close the third case
./hapi-core-cli case update $CASE_3_UUID "Third case" "https://hapi.one/authority/3" closed

# Now check out the list of cases
./hapi-core-cli case list

# We should have 3 cases by now
./hapi-core-cli case count
```

### Addresses

```sh
export ADDRESS_1="BpJYKGbDnTHJ1ZQ8UvxFkr9rJAaQX6J3MHvMxVhFmyR9"
export ADDRESS_2="6YqKPBrpKx7HsjE38W3ZFs39wpDvcYAoL5vz1ifjFvJM"
export ADDRESS_3="5ymaZVmNNHgZLaWptNAYWXJJWS2B44dnPcErNcxvXUt9"
export ADDRESS_4="GjPfA8rU3jcD97MkvDpgofbKgWEfib2zY4Akhr8Gk4mE"

# Create a few addresses by the authority
./hapi-core-cli address create $ADDRESS_1 $CASE_1_UUID theft 8
./hapi-core-cli address create $ADDRESS_2 $CASE_1_UUID theft 3

# ...and a few by the publisher
./hapi-core-cli address create $ADDRESS_3 $CASE_1_UUID theft 3 --private-key $PUBLISHER_PK
./hapi-core-cli address create $ADDRESS_4 $CASE_2_UUID theft 3 --private-key $PUBLISHER_PK

# See the list of created addresses
./hapi-core-cli address list

# Check that now we have 4 addresses in total
./hapi-core-cli address count

# Make sure that publisher can't update authority's address
./hapi-core-cli address update $ADDRESS_1 $CASE_2_UUID gambling 2 --private-key $PUBLISHER_PK

# ...but can update theirs
./hapi-core-cli address update $ADDRESS_3 $CASE_2_UUID gambling 2 --private-key $PUBLISHER_PK

# See the change
./hapi-core-cli address get $ADDRESS_3
```
