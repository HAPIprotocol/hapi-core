# HAPI Core - Near

This is a HAPI Core contract for Near blockchain.

## Build contract

```sh
cd near && build_docker.sh
```

## Local deployment

You must not build the contract before proceeding.

You must have [docker](https://docs.docker.com/engine/install/ubuntu/) installed.

You must have [near_cli](https://docs.near.org/tools/near-cli#installation) installed.

### Quick start

Using the following command you will receive:

- a running local near node;
- hapi contract deployed on account `hapi.test.near`, this account is also contract authority;
- token contract is deployed on account `token.test.near`, with 1000 tokens minted;
- account `authority.test.near` is created and has 20 tokens minted;
- account `reporter.test.near` is created and has 20 tokens minted;

Private keys for all accounts are stored in `.near-credentials/local/account_name.json` files. Private keys will be regenerated on each run of command.

```sh
cd ../client.rs
cargo test setup_local_near_node
```

### Manual start

1. Run local node in a separate terminal

```sh
docker run --name node_master -d -e INIT=1 -p3030:3030 nearprotocol/nearcore:1.35.0
```

2. Copy credentials from docker container to local folder

```sh
docker cp node_master:/srv/near/validator_key.json ~/.near-credentials/local/
docker cp node_master:/srv/near/validator_key.json ~/.near/
```

1. Ensure that node is running

This command should return table with validators, must be only one validator with name `test.near`.

```sh
export NEAR_ENV=local
near validators current
```

After this step you receive only test.near account which will be used as master account.

### Create env variables

```sh
export NEAR_ENV=local
export MASTER_ID=test.near
export AUTHORITY_ID=authority.$MASTER_ID
export CONTRACT_ID=hapi.$MASTER_ID
export PUBLISHER_ID=reporter.$MASTER_ID
export TOKEN_ID=token.$MASTER_ID
```

Now you can use near_cli to work with localnet.

IF you need create account use:

```sh
near create-account $CONTRACT_ID --masterAccount test.near --initialBalance 10
```

To deploy contract use:

```sh
near deploy $CONTRACT_ID --wasmFile=res/hapi_core_near.wasm
```

Command to work with contract via near_cli you can find [here](README_contract.md).

### Prepare client

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
export NETWORK=near
export PROVIDER_URL=http://127.0.0.1:3030/
```

### Contract authority

Get and set contract authority (aka owner).

**Take generated PK from `~/.near-credentials/local/account_id.json`.**

It will look like `2CjCjjvHyK35z7xF2o9vHMoVDqALoYAis8BshQJ9FBE6yMQRop8XSMQWt87XUR7PT4c8HjGFVc9LNw18XWHZmBg`

```sh
export AUTHORITY_PK=authority_pk

export PUBLISHER_PK=publisher_pk
```

By setting this variable we set the default account and private-key for all commands, if they are not specified explicitly. You can use here contract authority account id and private key.

```sh
export PRIVATE_KEY=publisher_pk

export ACCOUNT_ID=$CONTRACT_ID
export CONTRACT_ADDRESS=$CONTRACT_ID
```

```sh
# Check that it's our initial public key
./hapi-core-cli authority get

# Set it to another address

./hapi-core-cli authority set $PUBLISHER_ID

# Make sure it has been changed

./hapi-core-cli authority get

# Set it back

./hapi-core-cli authority set $CONTRACT_ID --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

```

### Contract configuration

Manipulate stake and reward configurations.

```sh
export ADDRESS_CONFIRMATION_REWARD=1000000000 # 1e9
export ADDRESS_TRACER_REWARD=1000000000 # 1e9
export ASSET_CONFIRMATION_REWARD=1000000000 # 1e9
export ASSET_TRACER_REWARD=1000000000 # 1e9

# Should return an error, as we haven't configured it yet
./hapi-core-cli configuration get-reward

# Update settings
./hapi-core-cli configuration update-reward $TOKEN_ID $ADDRESS_CONFIRMATION_REWARD \
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
./hapi-core-cli configuration update-stake $TOKEN_ID $UNLOCK_DURATION $VALIDATOR_STAKE \
$TRACER_STAKE $PUBLISHER_STAKE $AUTHORITY_STAKE

# Make sure that stake configuration is now set
./hapi-core-cli configuration get-stake
```

### Reporters

Create and activate a new reporter with authority role.

```sh

# We'll need a UUID for our new reporter
export AUTHORITY_UUID="2163ddbf-cc88-409a-b7cf-7bc6a2ec4cd1"

# Create the reporter
./hapi-core-cli reporter create $AUTHORITY_UUID $AUTHORITY_ID authority "Authority reporter" "https://hapi.one/authority"

# Check on our new reporter, it's inactive
./hapi-core-cli reporter get $AUTHORITY_UUID

# Let's check that we have enough tokens for the stake
./hapi-core-cli token balance $TOKEN_ID $AUTHORITY_ID

# Activate the reporter
./hapi-core-cli reporter activate --account-id $AUTHORITY_ID --private-key $AUTHORITY_PK

# Check the list of the reporters, now our reporter is active
./hapi-core-cli reporter list
```

Let's add another reporter with publisher role.

```sh
# Generate a UUID for the publisher
export PUBLISHER_UUID="2896e3be-c40d-4864-b035-4278ba19d4bd"

# Create the reporter
./hapi-core-cli reporter create $PUBLISHER_UUID $PUBLISHER_ID publisher "Publisher reporter" "https://hapi.one/publisher"

# Check that it has been added as inactive
./hapi-core-cli reporter get $PUBLISHER_UUID

# Make sure that we now have enough tokens
./hapi-core-cli token balance $TOKEN_ID $PUBLISHER_ID

# Activate the reporter, signed by the reporter
./hapi-core-cli reporter activate --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

# Check the list of the reporters, both reporters should be active
./hapi-core-cli reporter list

# We can see that we now have 2 reporters
./hapi-core-cli reporter count
```

### Cases

We'll create a few cases.

```sh
# Set authority account as default
export ACCOUNT_ID=$AUTHORITY_ID
export PRIVATE_KEY=$AUTHORITY_PK

export CASE_1_UUID="b414275b-1f1e-4083-b4dd-04dc41a9c301"
export CASE_2_UUID="4a044ed9-9e5e-4f8a-bbb2-0bf3c4fd7fb8"
export CASE_3_UUID="d4f41512-db3e-4306-9303-203ace5fe7e2"

# Create a general-purpose case
./hapi-core-cli case create $CASE_1_UUID "First case" "https://hapi.one/authority/1"

# Create a case by the publisher reporter
./hapi-core-cli case create $CASE_2_UUID "Second case" "https://hapi.one/publisher/2" --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

# Make sure that publisher can't change authority's case
./hapi-core-cli case update $CASE_1_UUID "Modified first case" "https://hapi.one/publisher/1" open --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

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
export ADDRESS_1="address1.near"
export ADDRESS_2="address2.near"
export ADDRESS_3="address3.near"
export ADDRESS_4="address4.near"

# Create a few addresses by the authority
./hapi-core-cli address create $ADDRESS_1 $CASE_1_UUID theft 8
./hapi-core-cli address create $ADDRESS_2 $CASE_1_UUID theft 3

# ...and a few by the publisher
./hapi-core-cli address create $ADDRESS_3 $CASE_1_UUID theft 3 --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK
./hapi-core-cli address create $ADDRESS_4 $CASE_2_UUID Scam 3 --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

# See the list of created addresses
./hapi-core-cli address list

# Check that now we have 4 addresses in total
./hapi-core-cli address count

# Make sure that publisher can't update authority's address
./hapi-core-cli address update $ADDRESS_1 $CASE_2_UUID gambling 2 --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

# ...but can update theirs
./hapi-core-cli address update $ADDRESS_3 $CASE_2_UUID gambling 2 --account-id $PUBLISHER_ID --private-key $PUBLISHER_PK

# See the change
./hapi-core-cli address get $ADDRESS_3
```
