import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { HapiCoreProgram, stringFromArray } from "../lib";
import { errorHandler, successHandler } from "./utils";
import * as dotenv from "dotenv";
import chalk from "chalk";

dotenv.config();

let provider;
if (process.env.ANCHOR_PROVIDER_URL) {
  provider = anchor.AnchorProvider.env();
} else {
  provider = anchor.AnchorProvider.local();
}
const hapiCore = new HapiCoreProgram(process.env.CONTRACT_ADDRESS, provider);

const stakeConfiguration = {
  unlockDuration: new BN(600),
  validatorStake: new BN(10),
  tracerStake: new BN(25),
  publisherStake: new BN(50),
  authorityStake: new BN(100),
  appraiserStake: new BN(10),
};

const rewardConfiguration = {
  addressTracerReward: new BN(1),
  addressConfirmationReward: new BN(1),
  assetTracerReward: new BN(1),
  assetConfirmationReward: new BN(1),
};

let stakeMint = PublicKey.default;
let rewardMint = PublicKey.default;

async function main() {
  const [NETWORK_NAME, STAKE_MINT, REWARD_MINT] = process.argv.slice(2);

  if (!NETWORK_NAME) {
    throw new Error(`Argument <NETWORK_NAME> is required`);
  }

  if (!process.env.CONTRACT_ADDRESS) {
    throw new Error("CONTRACT_ADDRESS is not set");
  }

  if (STAKE_MINT) {
    stakeMint = new PublicKey(STAKE_MINT);
  }

  if (REWARD_MINT) {
    rewardMint = new PublicKey(REWARD_MINT);
  }

  const tx = await hapiCore.InitializeNetwork(
    NETWORK_NAME,
    stakeConfiguration,
    rewardConfiguration,
    rewardMint,
    stakeMint
  );

  console.log(chalk.green(`Network created. Signature: ${tx}`));

  const [network] = hapiCore.findNetworkAddress(NETWORK_NAME);
  const data = await hapiCore.program.account.network.fetch(network);

  return {
    account: network,
    version: data.version,
    authority: data.authority.toString(),
    name: stringFromArray(data.name as number[]),
    stakeConfiguration: {
      stakeMint: data.stakeMint.toString(),
      unlockDuration: data.stakeConfiguration.unlockDuration.toNumber(),
      validatorStake: data.stakeConfiguration.validatorStake.toNumber(),
      tracerStake: data.stakeConfiguration.tracerStake.toNumber(),
      publisherStake: data.stakeConfiguration.publisherStake.toNumber(),
      authorityStake: data.stakeConfiguration.authorityStake.toNumber(),
      appraiserStake: data.stakeConfiguration.appraiserStake.toNumber(),
    },
    rewardConfiguration: {
      rewardMint: data.rewardMint.toString(),
      addressTracerReward:
        data.rewardConfiguration.addressTracerReward.toNumber(),
      addressConfirmationReward:
        data.rewardConfiguration.addressConfirmationReward.toNumber(),
      assetTracerReward: data.rewardConfiguration.assetTracerReward.toNumber(),
      assetConfirmationReward:
        data.rewardConfiguration.assetConfirmationReward.toNumber(),
    },
  };
}

main().then(successHandler).catch(errorHandler);
