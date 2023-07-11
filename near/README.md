# HAPI Core - NEAR

This is a HAPI Core contract for NEAR blockchain.

## Usage

## Accounts

```bash
export NEAR_ENV=testnet
export AUTHORITY_ID=authority.near
export CONTRACT_ID=contract.near
```

### For creating the new account for deploying contract use next command

```bash
near create-account $CONTRACT_ID --masterAccount $AUTHORITY_ID --initialBalance 10
```

### Deploy contract

```bash
near deploy $CONTRACT_ID --wasmFile=res/hapi_core_near_release.wasm
```
