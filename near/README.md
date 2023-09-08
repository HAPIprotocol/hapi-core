# HAPI Core - NEAR

This is a HAPI Core contract for NEAR blockchain.

## Usage

## Building

For building locally use `build_local.sh` script located in **near/contract** folder.

For building in docker use `build_docker.sh` script located in **near** folder.

## Accounts

```bash
export NEAR_ENV=testnet
export AUTHORITY_ID=authority.near
export CONTRACT_ID=contract.near
export REPORTER_ID=reporter.near
export STAKE_TOKEN=stake.near
export REWARD_TOKEN=reward.near
```

### For creating the new account for deploying contract use next command

```bash
near create-account $CONTRACT_ID --masterAccount $AUTHORITY_ID --initialBalance 10
```

### Deploy contract

```bash
near deploy $CONTRACT_ID --wasmFile=res/hapi_core_near.wasm
```

### Initialize contract

```bash
near call $CONTRACT_ID initialize '{}' --accountId $AUTHORITY_ID
```

## View methods

### Get authority

```bash
near view $CONTRACT_ID get_authority '{}'
```

## Get stake configuration

Returns a structure of Stake configuration.

```bash
near view $CONTRACT_ID get_stake_configuration '{}'
```

## Get reward configuration

Returns a structure of Reward configuration.

```bash
near view $CONTRACT_ID get_reward_configuration '{}'
```

### Get reporter

```bash
near view $CONTRACT_ID get_reporter '{"id": "42"}'
```

### Get reporters

```bash
near view $CONTRACT_ID get_reporters '{"take": 10, "skip": 0}'
```

### Get reporters count

```bash
near view $CONTRACT_ID get_reporter_count '{}'
```

### Get reporter by account

```bash
near view $CONTRACT_ID get_reporter_by_account '{"account_id": "'$REPORTER_ID'"}'
```

### Get case

Returns a Case structure.

```bash
near view $CONTRACT_ID get_case '{"id": "UUID"}'
```

### Get cases

Returns a vector of Case structures.

```bash
near view $CONTRACT_ID get_cases '{"take": 10, "skip": 0}'
```

### Get case count

```bash
near view $CONTRACT_ID get_case_count '{}'
```

### Get address

Returns an AddressView structure.

```bash
near view $CONTRACT_ID get_address '{"address": "address.near"}'
```

### Get addresses

Returns a vector of AddressView structures.

```bash
near view $CONTRACT_ID get_addresses '{"take": 10, "skip": 0}'
```

### Get address count

```bash
near view $CONTRACT_ID get_address_count '{}'
```

## Contract configuration

### Set authority

Callable from authority only.

```bash
near call $CONTRACT_ID set_authority '{"authority": "'$AUTHORITY_ID'"}' --accountId $AUTHORITY_ID
```

### Update stake configuration

Callable from authority only.

```bash
near call $CONTRACT_ID update_stake_configuration '{"stake_configuration": {"token": "'$STAKE_TOKEN'", "unlock_duration": 420, "validator_stake": "5", "tracer_stake": "10", "publisher_stake": "15", "authority_stake": "20"}}' --accountId $AUTHORITY_ID
```

### Update reward configuration

Callable from authority only.

```bash
near call $CONTRACT_ID update_reward_configuration '{"reward_configuration": {"token": "$REWARD_TOKEN", "address_confirmation_reward": "4", "address_tracer_reward": "20", "asset_confirmation_reward": "5", "asset_tracer_reward": "15"}}' --accountId $AUTHORITY_ID
```

## Reporter management

### Create reporter

Callable from authority only.

```bash
near call $CONTRACT_ID create_reporter '{"id": "42", "account_id": "'$REPORTER_ID'", "name": "reporter", "role": "Publisher", "url": "reporter.com"}' --accountId $AUTHORITY_ID
```

### Update reporter

Callable from authority only.

```bash
near call $CONTRACT_ID update_reporter '{"id": "42", "account_id": "'$REPORTER_ID'", "name": "reporter", "role": "Publisher", "url": "reporter.com"}' --accountId $AUTHORITY_ID
```

### Activate reporter

```bash
near call $STAKE_TOKEN ft_transfer_call '{"receiver_id": "'$CONTRACT_ID'", "amount": "1000000", "msg": "", "memo": ""}' --account_id $REPORTER_ID --depositYocto 1 --gas=100000000000000
```

### Deactivate reporter

Callable from reporter for itself only.

```bash
near call $CONTRACT_ID deactivate_reporter '{}' --accountId $REPORTER_ID
```

### Unstake

Callable from reporter for itself only.

```bash
near call $CONTRACT_ID unstake '{}' --accountId $REPORTER_ID --gas=60000000000000
```

## Case management

### Create case

```bash
near call $CONTRACT_ID create_case '{"id": "42", "name": "Case", "url": "case.com"}' --accountId $REPORTER_ID
```

### Update case

```bash
near call $CONTRACT_ID update_case '{"id": "42", "name": "Case", "status":"Closed", "url": "case.com"}' --accountId $REPORTER_ID
```

## Address management

### Create address

```bash
near call $CONTRACT_ID create_address '{"address": "address.near", "category": "Scam", "risk_score": 5, "case_id": "UUID"}' --accountId $REPORTER_ID
```

### Update address

```bash
near call $CONTRACT_ID update_address '{"address": "address.near", "category": "Scam", "risk_score": 5, "case_id": "UUID"}' --accountId $REPORTER_ID
```

### Confirm address

```bash
near call $CONTRACT_ID confirm_address '{"address": "address.near"}' --accountId $REPORTER_ID
```
