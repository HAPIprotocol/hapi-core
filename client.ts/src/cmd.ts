import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { uuidToBigInt } from "./util";
import { HapiCore, HapiCoreNetworks } from "./interface";
import { connectHapiCore } from ".";

yargs(hideBin(process.argv))
  .command(
    "get-authority",
    "Get current authority address",
    {},
    cmdWrapper(getAuthority)
  )
  .command(
    "get-stake-configuration",
    "Get current stake configuration",
    {},
    cmdWrapper(getStakeConfiguration)
  )
  .command(
    "get-reward-configuration",
    "Get current reward configuration",
    {},
    cmdWrapper(getRewardConfiguration)
  )
  .command(
    "get-reporter",
    "Get reporter",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Reporter ID",
        coerce: uuidToBigInt,
      },
    },
    cmdWrapper(getReporter)
  )
  .command(
    "get-case",
    "Get case",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: uuidToBigInt,
      },
    },
    cmdWrapper(getCase)
  )
  .command(
    "get-address",
    "Get address",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
    },
    cmdWrapper(getAddress)
  )
  .option("network", {
    global: true,
    demandOption: true,
    description: "Network",
    choices: HapiCoreNetworks,
  })
  .option("provider-url", {
    global: true,
    demandOption: true,
    description: "Provider URL",
    type: "string",
    default: "http://localhost:8545",
  })
  .demandCommand(1)
  .help()
  .parse();

interface Setup {
  hapiCore: HapiCore;
}

async function setup(argv: any): Promise<Setup> {
  const hapiCore = connectHapiCore({
    network: argv.network,
    provider: {
      providerUrl: argv.providerUrl,
    },
  });

  return { hapiCore };
}

function cmdWrapper(
  fn: (setup: Setup, argv: any) => Promise<void>
): (argv: any) => Promise<void> {
  return async (argv: any): Promise<void> => {
    try {
      await fn(await setup(argv), argv);
    } catch (error) {
      if (argv.verbose) {
        console.error(error);
      } else {
        const abstractError = error as any;
        console.error(`Error: ${abstractError.reason || abstractError.message || abstractError}`);
      }
    }
  };
}

async function getAuthority(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getAuthority());
}

async function getStakeConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getStakeConfiguration());
}

async function getRewardConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getRewardConfiguration());
}

async function getReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getReporter(argv.id.toString()));
}

async function getCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getCase(argv.id.toString()));
}

async function getAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getAddress(argv.address));
}
