import { web3, BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { v4 as uuidv4 } from "uuid";

import {
  stakeConfiguration,
  rewardConfiguration,
  HapiCoreProgram,
  ReporterRole,
} from "../../lib";

import { TestToken } from "./token";

export type Network = {
  name: string;
  stakeConfiguration: stakeConfiguration;
  rewardConfiguration: rewardConfiguration;
};

export type Reporter = {
  name: string;
  id: string;
  keypair: web3.Keypair;
  role: keyof typeof ReporterRole;
  url: string;
};

export type Case = {
  id: string;
  name: string;
  url: string;
};

export function getNetwotks(names: Array<string>) {
  let networks: Record<string, Network> = {};

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

export function getReporters() {
  const reporters: Record<string, Reporter> = {
    publisher: {
      id: uuidv4(),
      name: "alice",
      keypair: web3.Keypair.generate(),
      role: "Publisher",
      url: "https://publisher.blockchain",
    },
    tracer: {
      id: uuidv4(),
      name: "bob",
      keypair: web3.Keypair.generate(),
      role: "Tracer",
      url: "https://tracer.blockchain",
    },
    authority: {
      id: uuidv4(),
      name: "carol",
      keypair: web3.Keypair.generate(),
      role: "Authority",
      url: "https://authority.blockchain",
    },
    validator: {
      id: uuidv4(),
      name: "dave",
      keypair: web3.Keypair.generate(),
      role: "Validator",
      url: "https://validator.blockchain",
    },
    appraiser: {
      id: uuidv4(),
      name: "erin",
      keypair: web3.Keypair.generate(),
      role: "Appraiser",
      url: "https://appraiser.blockchain",
    },
  };

  return reporters;
}

export function getCases() {
  const cases: Record<string, Case> = {
    firstCase: {
      id: uuidv4(),
      name: "safe network addresses",
      url: "https://big.hack",
    },
    secondCase: {
      id: uuidv4(),
      name: "suspicious nft txes",
      url: "https://big.hack",
    },
    thirdCase: {
      id: uuidv4(),
      name: "new case",
      url: "https://big.hack",
    },
  };

  return cases;
}

export async function setupNetworks(
  program: HapiCoreProgram,
  networks: Record<string, Network>,
  rewardToken: PublicKey,
  stakeToken: PublicKey
) {
  const wait: Promise<unknown>[] = [];

  for (const key of Object.keys(networks)) {
    const network = networks[key];

    wait.push(
      program.InitializeNetwork(
        network.name,
        network.stakeConfiguration,
        network.rewardConfiguration,
        rewardToken,
        stakeToken
      )
    );
  }

  await Promise.all(wait);
}

export async function setupReporters(
  program: HapiCoreProgram,
  reporters: Record<string, Reporter>,
  network_name: string,
  stakeToken: TestToken
) {
  for (const key of Object.keys(reporters)) {
    const reporter = reporters[key];

    await program.createReporter(
      network_name,
      reporter.id,
      reporter.role,
      reporter.keypair.publicKey,
      reporter.name,
      reporter.url
    );

    await program.program.provider.connection.requestAirdrop(
      reporter.keypair.publicKey,
      10_000_000
    );

    await stakeToken.getTokenAccount(reporter.keypair.publicKey);
    await stakeToken.transfer(null, reporter.keypair.publicKey, 1_000_000);

    await program.activateReporter(network_name, reporter.keypair, reporter.id);
  }
}

export async function createCases(
  program: HapiCoreProgram,
  cases: Record<string, Case>,
  network_name: string,
  reporter: Reporter
) {
  const wait: Promise<unknown>[] = [];

  for (const key of Object.keys(cases)) {
    const cs = cases[key];

    wait.push(
      program.createCase(
        network_name,
        cs.id,
        cs.name,
        cs.url,
        reporter.keypair,
        reporter.id
      )
    );
  }

  await Promise.all(wait);
}
