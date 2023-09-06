# HAPI Core - Solidity

This is a HAPI Core contract written in Solidity for EVM-based blockchains.

## Build contract

```sh
# Make sure that dependencies are installed
npm install

# Use hardhat to compile the contract and create ABI
npm run build

# Use `jq` to extract ABI from built artifacts
jq .abi ./artifacts/contracts/HapiCore.sol/HapiCore.json

# Observe the generated Typescript interface
cat ./typechain-types/contracts/HapiCore.ts
```

## Local deployment

You should build the contract before proceeding

```sh
# 1. Run local node in a separate terminal
npx hardhat node

# 2. Get the contract owner private key from the node output
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# 3. Compile the contract
npx hardhat compile

# 4. Deploy the contract
npx hardhat deploy --network localhost

# 5. Get the deployed contract address from the output of deploy command
export CONTRACT_ADDRESS=0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0

# 6. Deploy a test token contract
npx hardhat deploy-test-token --network localhost

# 7. Get the deployed contract address from the token deploy command output
export TOKEN_ADDRESS=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

# 8. Run Javascript console with correct environment
npx hardhat console --network localhost
```

## Testing with the Rust client

Repeat points 1 through 7 from "Local deployment" section to deploy the contract on a local node.

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
export NETWORK=ethereum
export PROVIDER_URL=http://127.0.0.1:8545/
export CHAIN_ID=31337
```

### Contract authority

Get and set contract authority (aka owner).

```sh
export AUTHORITY_ADDR=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
export AUTHORITY_PK=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

export PUBLISHER_ADDR=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
export PUBLISHER_PK=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

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
export ADDRESS_CONFIRMATION_REWARD=1000000000000000000 # 1e18
export TRACER_REWARD=1000000000000000000 # 1e18

# Should return an error, as we haven't configured it yet
./hapi-core-cli configuration get-reward

# Update settings
./hapi-core-cli configuration update-reward $TOKEN_ADDRESS $ADDRESS_CONFIRMATION_REWARD $TRACER_REWARD

# Make sure that reward configuration is now set
./hapi-core-cli configuration get-reward

export UNLOCK_DURATION=60 # 60 seconds
export VALIDATOR_STAKE=100000000000000000000 # 100e18
export TRACER_STAKE=101000000000000000000 # 101e18
export PUBLISHER_STAKE=102000000000000000000 # 102e18
export AUTHORITY_STAKE=103000000000000000000 # 103e18

# Should return an error, as we haven't configured it yet
./hapi-core-cli configuration get-stake

# Update settings
./hapi-core-cli configuration update-stake $TOKEN_ADDRESS $UNLOCK_DURATION $VALIDATOR_STAKE $TRACER_STAKE $PUBLISHER_STAKE $AUTHORITY_STAKE

# Make sure that stake configuration is now set
./hapi-core-cli configuration get-stake 
```

### Reporters

Create and activate a new reporter with authority role.

```sh
# We'll need a UUID for our new reporter
export AUTHORITY_UUID="2163ddbf-cc88-409a-b7cf-7bc6a2ec4cd1"

# Create the reporter
./hapi-core-cli reporter create $AUTHORITY_UUID $AUTHORITY_ADDR authority "Authority reporter" "https://hapi.one/authority"

# Check on our new reporter, it's inactive
./hapi-core-cli reporter get $AUTHORITY_UUID

# Let's check that we have enough tokens for the stake
./hapi-core-cli token balance $TOKEN_ADDRESS $AUTHORITY_ADDR

# Let's approve token allowance for our stake
./hapi-core-cli token approve $TOKEN_ADDRESS $CONTRACT_ADDRESS $AUTHORITY_STAKE

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
./hapi-core-cli token transfer $TOKEN_ADDRESS $PUBLISHER_ADDR $PUBLISHER_STAKE

# Make sure that we now have enought tokens
./hapi-core-cli token balance $TOKEN_ADDRESS $PUBLISHER_ADDR

# Approve token allowance for the stake, signed by the reporter
./hapi-core-cli token approve $TOKEN_ADDRESS $CONTRACT_ADDRESS $PUBLISHER_STAKE --private-key $PUBLISHER_PK

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
export ADDRESS_1="0xfcf0a2631c2cf023c8c3955f348ea10ae2b43b21"
export ADDRESS_2="0x5cea62b209091c1e9be11e3e0e74e57fab2a18bf"
export ADDRESS_3="0xa554150aa42540740e85931c8dbd855bac103eb9"
export ADDRESS_4="0x81273458f2b0d78d457edcc0ff6fb9e486f69ea0"

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
./hapi-core-cli address update $ADDRESS_1 $CASE_2_UUID gambling 2

# ...but can update theirs
./hapi-core-cli address update $ADDRESS_3 $CASE_2_UUID gambling 2

# See the change
./hapi-core-cli address get $ADDRESS_3
```