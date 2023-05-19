import { JsonRpcProvider } from "@ethersproject/providers";
import * as uuid from "uuid";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import { HapiCoreNetwork, connectHapiCore } from ".";

yargs(hideBin(process.argv))
  .command("get-authority", "Get current authority address", {}, getAuthority)
  .command(
    "get-stake-configuration",
    "Get current stake configuration",
    {},
    getStakeConfiguration
  )
  .command(
    "get-reward-configuration",
    "Get current reward configuration",
    {},
    getRewardConfiguration
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
    getReporter
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
    getCase
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
    getAddress
  )
  .option("network", {
    global: true,
    demandOption: true,
    description: "Network",
    choices: [
      HapiCoreNetwork.Ethereum,
      HapiCoreNetwork.Solana,
      HapiCoreNetwork.Bitcoin,
      HapiCoreNetwork.BSC,
      HapiCoreNetwork.NEAR,
    ],
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

async function getAuthority(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getAuthority());
  } catch (e) {
    console.error(`${e}`);
  }
}

async function getStakeConfiguration(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getStakeConfiguration());
  } catch (e) {
    console.error(`${e}`);
  }
}

async function getRewardConfiguration(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getRewardConfiguration());
  } catch (e) {
    console.error(`${e}`);
  }
}

async function getReporter(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getReporter(argv.id.toString()));
  } catch (e) {
    console.error(`${e}`);
  }
}

async function getCase(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getCase(argv.id.toString()));
  } catch (e) {
    console.error(`${e}`);
  }
}

async function getAddress(argv: any) {
  const provider = new JsonRpcProvider(argv.providerUrl);

  const hapiCore = connectHapiCore(argv.network, provider);

  try {
    console.log(await hapiCore.getAddress(argv.address));
  } catch (e) {
    console.error(`${e}`);
  }
}

function uuidToBigInt(input: string): BigInt {
  uuid.parse(input);
  return BigInt("0x" + input.replace(/-/g, ""));
}
