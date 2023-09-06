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
# Run local node
npx hardhat node

# Get the contract owner private key from the node output
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# Deploy the contract
npx hardhat deploy --network localhost

# Get the deployed contract address from the output of deploy command
export CONTRACT_ADDRESS=0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0

# Deploy a test token contract
npx hardhat deploy-test-token --network localhost

# Get the deployed contract address from the token deploy command output
export TOKEN_ADDRESS=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9

# Run Javascript console with correct environment
npx hardhat console --network localhost
```
