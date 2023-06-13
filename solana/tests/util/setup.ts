import { web3, BN } from "@coral-xyz/anchor";

import {
  stakeConfiguration,
  rewardConfiguration,
  HapiCoreProgram,
  bufferFromString,
} from "../../lib";
import { PublicKey } from "@solana/web3.js";

export type Networks = Record<
  string,
  {
    name: string;
    stakeConfiguration: stakeConfiguration;
    rewardConfiguration: rewardConfiguration;
  }
>;

export function randomId(): BN {
  return new BN(Math.floor(Math.random() * Math.pow(2, 64)).toString());
}

export function getReporters() {
  return {
    alice: {
      id: randomId(),
      name: "alice",
      keypair: web3.Keypair.generate(),
      role: "Publisher",
      url: "https://publisher.blockchain",
    },
    bob: {
      id: randomId(),
      name: "bob",
      keypair: web3.Keypair.generate(),
      role: "Tracer",
      url: "https://tracer.blockchain",
    },
    carol: {
      id: randomId(),
      name: "carol",
      keypair: web3.Keypair.generate(),
      role: "Authority",
      url: "https://authority.blockchain",
    },
    dave: {
      id: randomId(),
      name: "dave",
      keypair: web3.Keypair.generate(),
      role: "Validator",
      url: "https://validator.blockchain",
    },
    erin: {
      id: randomId(),
      name: "erin",
      keypair: web3.Keypair.generate(),
      role: "Appraiser",
      url: "https://appraiser.blockchain",
    },
  };
}

export function getNetwotks(names: Array<string>) {
  let networks: Networks = {};

  names.forEach((name) => {
    networks[name] = {
      name,
      stakeConfiguration: {
        unlockDuration: new BN(1_000),
        validatorStake: new BN(2_000),
        tracerStake: new BN(3_000),
        publisherStake: new BN(4_000),
        authorityStake: new BN(5_000),
        appraiserStake: new BN(6_000),
      },
      rewardConfiguration: {
        addressTracerReward: new BN(1_000),
        addressConfirmationReward: new BN(2_000),
        assetTracerReward: new BN(3_000),
        assetConfirmationReward: new BN(4_000),
      },
    };
  });

  return networks;
}

export async function createNetwotks(
  program: HapiCoreProgram,
  networks: Networks,
  authority: PublicKey,
  rewardToken: PublicKey,
  stakeToken: PublicKey
) {
  const wait: Promise<unknown>[] = [];
  const programDataAddress = program.findProgramDataAddress()[0];

  for (const key of Object.keys(networks)) {
    const network = networks[key];

    const [networkAccount, bump] = program.findNetworkAddress(network.name);

    const args = [
      bufferFromString(network.name, 32).toJSON().data,
      network.stakeConfiguration,
      network.rewardConfiguration,
      bump,
    ];

    wait.push(
      program.program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority,
          network: networkAccount,
          rewardMint: rewardToken,
          stakeMint: stakeToken,
          programAccount: program.programId,
          programData: programDataAddress,
          systemProgram: web3.SystemProgram.programId,
        },
      })
    );
  }

  await Promise.all(wait);
}
