# HAPI Core

HAPI Core contract built on Anchor for Solana. If you want to know more about HAPI Protocol, please visit the [official site](https://hapi.one/) and our [gitbook](https://hapi-one.gitbook.io/hapi-protocol). If you want to propose any changes to this smart contract, please visit our [governance forum](https://gov.hapi.one/). Suggestions for the client library enhancements are welcome.

## Dependencies

To install everything you need to work with this project, you'll need to install dependencies as described in [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) documentation.

## Program

The source code of **hapi-core** program is in `./programs/hapi-core`.

### Build

To build the **hapi-core** program, you need to execute this command:

```sh
anchor build
```

You'll get the following output:

- program binaries at `./target/deploy/hapi_core.so`
- IDL file at `./target/idl/hapi_core.json`
- Typescript typings at `./target/types/hapi_core.ts`

### Test

To test the program, you'll have to run this command:

```sh
anchor test
```

This command starts a local validator, sets up the program on chain and runs a suite of Jest tests against it.

### Deploy

To deploy the program, run this command:

```sh
anchor deploy \
    --provider.cluster https://api.mainnet-beta.solana.com \
    --provider.wallet ~/.config/solana/id.json
```

Where `provider.cluster` is the target node API and `provider.wallet` is the path to keypair you want to use to deploy the program.

## Javascript client

The Javascript/Typescript client for this program is an NPM package that can be found here: [@hapi.one/core-cli](https://www.npmjs.com/package/@hapi.one/core-cli).

It's published by this command:

```sh
npm publish
```

Please view the test suite (`./tests/hapi-core/**.spec.ts`) to see how can this client be used in NodeJS context.

### Basic usage example in browser

```typescript
import { Connection, PublicKey } from "@solana/web3.js";
import { Provider } from "@coral-xyz/anchor";
import { initHapiCore } from "@hapi.one/core-cli";

// Setup web3 Connection
const connection = new Connection("https://api.mainnet-beta.solana.com");

// Use Phantom wallet provider
const wallet = window.solana;

// Setup Anchor provider
const provider = new Provider(connection, wallet as any);

// hapi-core program ID is a well-known public key
const HAPI_PROGRAM_ID = new PublicKey(
  "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6"
);

// Setup the client
const hapiCore = initHapiCore(HAPI_PROGRAM_ID, provider);

// HAPI community account is a well-known public key
const communityAccount = new PublicKey(
  "31gQ11Qsd7dPcnkdCJ2ZGnY2ijRXsvFCPWagpcFxYwem"
);

// Use client to fetch community account data
const communityData = await hapiCore.account.community.fetch(communityAccount);
console.log(communityData);

// Find a PDA for a particular network
const [networkAccount] = await program.pda.findNetworkAddress(
  communityAccount,
  "solana"
);

// Encode address buffer
const addressEncoded = hapiCore.util.encodeAddress(
  "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
  "Solana"
);

// Find a PDA for an address, which we want to check
const [addressAccount] = await program.pda.findAddressAddress(
  networkAccount,
  addressEncoded
);

// Fetch address risk scoring data
const addressData = await hapiCore.account.address.fetch(addressAccount);
console.log(addressData);
```
