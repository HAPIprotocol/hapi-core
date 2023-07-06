import yargs from "yargs";
import { hideBin } from "yargs/helpers";

import {
  CaseStatusFromString,
  CaseStatusToString,
  CategoryFromString,
  ReporterRoleFromString,
  ReporterRoleToString,
  ReporterStatusToString,
  validateRiskScore,
  validateUuid,
} from "./util";
import {
  CaseStatusNames,
  CategoryNames,
  CommandOutput,
  CommandOutputs,
  HapiCore,
  HapiCoreNetworks,
  ReporterRoleNames,
} from "./interface";
import { connectHapiCore } from ".";

yargs(hideBin(process.argv))
  .env("HAPI_CORE")
  .command(
    "get-authority",
    "Get current authority address",
    {},
    cmdWrapper(getAuthority)
  )
  .command(
    "set-authority",
    "Set authority address",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Authority address",
      },
    },
    cmdWrapper(setAuthority)
  )
  .command(
    "get-stake-configuration",
    "Get current stake configuration",
    {},
    cmdWrapper(getStakeConfiguration)
  )
  .command(
    "update-stake-configuration",
    "Update stake configuration",
    {
      token: {
        string: true,
        demandOption: true,
        description: "Stake token address",
      },
      "unlock-duration": {
        number: true,
        demandOption: true,
        description: "Unlock duration in seconds",
      },
      "validator-stake": {
        string: true,
        demandOption: true,
        description: "Validator reporter stake amount",
      },
      "tracer-stake": {
        string: true,
        demandOption: true,
        description: "Tracer reporter stake amount",
      },
      "publisher-stake": {
        string: true,
        demandOption: true,
        description: "Publisher reporter stake amount",
      },
      "authority-stake": {
        string: true,
        demandOption: true,
        description: "Authority reporter stake amount",
      },
    },
    cmdWrapper(updateStakeConfiguration)
  )
  .command(
    "get-reward-configuration",
    "Get current reward configuration",
    {},
    cmdWrapper(getRewardConfiguration)
  )
  .command(
    "update-reward-configuration",
    "Update reward configuration",
    {
      token: {
        string: true,
        demandOption: true,
        description: "Reward token address",
      },
      "address-confirmation-reward": {
        string: true,
        demandOption: true,
        description: "Address confirmation reward amount",
      },
      "trace-reward": {
        string: true,
        demandOption: true,
        description: "Trace reward amount",
      },
    },
    cmdWrapper(updateRewardConfiguration)
  )
  .command(
    "get-reporter",
    "Get reporter",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Reporter ID",
        coerce: validateUuid,
      },
    },
    cmdWrapper(getReporter)
  )
  .command(
    "get-reporter-count",
    "Get reporter count",
    {},
    cmdWrapper(getReporterCount)
  )
  .command(
    "get-reporters",
    "Get reporters",
    {
      skip: {
        number: true,
        description: "Skip",
        default: 0,
      },
      take: {
        number: true,
        description: "Take",
        default: 10,
      },
    },
    cmdWrapper(getReporters)
  )
  .command(
    "create-reporter",
    "Create reporter",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Reporter ID",
        coerce: validateUuid,
      },
      role: {
        string: true,
        demandOption: true,
        description: "Reporter role",
        choices: ReporterRoleNames,
      },
      account: {
        string: true,
        demandOption: true,
        description: "Reporter account address",
      },
      name: {
        string: true,
        demandOption: true,
        description: "Reporter display name",
      },
      url: {
        string: true,
        demandOption: true,
        description: "Reporter URL",
      },
    },
    cmdWrapper(createReporter)
  )
  .command(
    "update-reporter",
    "Update reporter",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Reporter ID",
        coerce: validateUuid,
      },
      role: {
        string: true,
        demandOption: true,
        description: "Reporter role",
        choices: ReporterRoleNames,
      },
      account: {
        string: true,
        demandOption: true,
        description: "Reporter account address",
      },
      name: {
        string: true,
        demandOption: true,
        description: "Reporter display name",
      },
      url: {
        string: true,
        demandOption: true,
        description: "Reporter URL",
      },
    },
    cmdWrapper(updateReporter)
  )
  .command(
    "activate-reporter",
    "Activate reporter",
    {},
    cmdWrapper(activateReporter)
  )
  .command(
    "deactivate-reporter",
    "Deactivate reporter",
    {},
    cmdWrapper(deactivateReporter)
  )
  .command(
    "unstake-reporter",
    "Unstake reporter",
    {},
    cmdWrapper(unstakeReporter)
  )
  .command(
    "get-case",
    "Get case",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
    },
    cmdWrapper(getCase)
  )
  .command("get-case-count", "Get case count", {}, cmdWrapper(getCaseCount))
  .command(
    "get-cases",
    "Get cases",
    {
      skip: {
        number: true,
        description: "Skip",
        default: 0,
      },
      take: {
        number: true,
        description: "Take",
        default: 10,
      },
    },
    cmdWrapper(getCases)
  )
  .command(
    "create-case",
    "Create case",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      name: {
        string: true,
        demandOption: true,
        description: "Case name",
      },
      url: {
        string: true,
        demandOption: true,
        description: "Case URL",
      },
    },
    cmdWrapper(createCase)
  )
  .command(
    "update-case",
    "Update case",
    {
      id: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      name: {
        string: true,
        demandOption: true,
        description: "Case name",
      },
      url: {
        string: true,
        demandOption: true,
        description: "Case URL",
      },
      status: {
        string: true,
        demandOption: true,
        description: "Case status",
        choices: CaseStatusNames,
      },
    },
    cmdWrapper(updateCase)
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
  .command(
    "get-address-count",
    "Get address count",
    {},
    cmdWrapper(getAddressCount)
  )
  .command(
    "get-addresses",
    "Get addresses",
    {
      skip: {
        number: true,
        description: "Skip",
        default: 0,
      },
      take: {
        number: true,
        description: "Take",
        default: 10,
      },
    },
    cmdWrapper(getAddresses)
  )
  .command(
    "create-address",
    "Create address",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
      caseId: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      risk: {
        number: true,
        demandOption: true,
        description: "Risk score (0..10)",
        coerce: validateRiskScore,
      },
      category: {
        string: true,
        demandOption: true,
        description: "Category",
        choices: CategoryNames,
      },
    },
    cmdWrapper(createAddress)
  )
  .command(
    "update-address",
    "Update address",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
      caseId: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      risk: {
        number: true,
        demandOption: true,
        description: "Risk score (0..10)",
        coerce: validateRiskScore,
      },
      category: {
        string: true,
        demandOption: true,
        description: "Category",
        choices: CategoryNames,
      },
    },
    cmdWrapper(updateAddress)
  )
  .command(
    "get-asset",
    "Get asset",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
      assetId: {
        string: true,
        demandOption: true,
        description: "Asset ID",
      },
    },
    cmdWrapper(getAsset)
  )
  .command("get-asset-count", "Get asset count", {}, cmdWrapper(getAssetCount))
  .command(
    "get-assets",
    "Get assets",
    {
      skip: {
        number: true,
        description: "Skip",
        default: 0,
      },
      take: {
        number: true,
        description: "Take",
        default: 10,
      },
    },
    cmdWrapper(getAssets)
  )
  .command(
    "create-asset",
    "Create asset",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
      assetId: {
        string: true,
        demandOption: true,
        description: "Asset ID",
      },
      caseId: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      risk: {
        number: true,
        demandOption: true,
        description: "Risk score (0..10)",
        coerce: validateRiskScore,
      },
      category: {
        string: true,
        demandOption: true,
        description: "Category",
        choices: CategoryNames,
      },
    },
    cmdWrapper(createAsset)
  )
  .command(
    "update-asset",
    "Update asset",
    {
      address: {
        string: true,
        demandOption: true,
        description: "Address",
      },
      assetId: {
        string: true,
        demandOption: true,
        description: "Asset ID",
      },
      caseId: {
        string: true,
        demandOption: true,
        description: "Case ID",
        coerce: validateUuid,
      },
      risk: {
        number: true,
        demandOption: true,
        description: "Risk score (0..10)",
        coerce: validateRiskScore,
      },
      category: {
        string: true,
        demandOption: true,
        description: "Category",
        choices: CategoryNames,
      },
    },
    cmdWrapper(updateAsset)
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
  .option("contract-address", {
    global: true,
    demandOption: true,
    description: "Contract address",
    type: "string",
    default: undefined,
  })
  .option("output", {
    global: true,
    demandOption: false,
    description: "Command output format",
    choices: CommandOutputs,
    default: CommandOutput.Plain,
  })
  // Used only with evm part
  .option("private-key", {
    global: true,
    demandOption: false,
    description: "Signer's private key",
    type: "string",
  })
  .demandCommand()
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
    address: argv.contractAddress,
    signerPrivateKey: argv.privateKey,
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
        console.error((error as any).error);
      } else {
        const abstractError = error as any;
        console.error(
          `Error: ${
            (abstractError.error ? abstractError.error.reason : null) ||
            abstractError.reason ||
            abstractError.message ||
            abstractError
          }`
        );
      }
    }
  };
}

function printResult(result: any, output: CommandOutput) {
  if (output === CommandOutput.Json) {
    console.log(JSON.stringify({ data: result }));
  } else {
    console.log(result);
  }
}

async function getAuthority(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAuthority(), argv.output);
}

async function setAuthority(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.setAuthority(argv.address), argv.output);
}

async function getStakeConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getStakeConfiguration(), argv.output);
}

async function updateStakeConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateStakeConfiguration(
      argv.token,
      argv.unlockDuration,
      argv.validatorStake,
      argv.tracerStake,
      argv.publisherStake,
      argv.authorityStake
    ),
    argv.output
  );
}

async function getRewardConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getRewardConfiguration(), argv.output);
}

async function updateRewardConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateRewardConfiguration(
      argv.token,
      argv.addressConfirmationReward,
      argv.traceReward
    ),
    argv.output
  );
}

async function getReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const reporter = await hapiCore.getReporter(argv.id.toString());

  printResult(
    {
      ...reporter,
      role: ReporterRoleToString(reporter.role),
      status: ReporterStatusToString(reporter.status),
    },
    argv.output
  );
}

async function getReporterCount(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getReporterCount(), argv.output);
}

async function getReporters(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const reporters = await hapiCore.getReporters(argv.skip, argv.take);

  printResult(
    reporters.map((reporter) => ({
      ...reporter,
      role: ReporterRoleToString(reporter.role),
      status: ReporterStatusToString(reporter.status),
    })),
    argv.output
  );
}

async function createReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.createReporter(
      argv.id,
      ReporterRoleFromString(argv.role),
      argv.account,
      argv.name,
      argv.url
    ),
    argv.output
  );
}

async function updateReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateReporter(
      argv.id.toString(),
      ReporterRoleFromString(argv.role),
      argv.account,
      argv.name,
      argv.url
    ),
    argv.output
  );
}

async function activateReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.activateReporter(), argv.output);
}

async function deactivateReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.deactivateReporter(), argv.output);
}

async function unstakeReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.unstakeReporter(), argv.output);
}

async function getCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const case_ = await hapiCore.getCase(argv.id.toString());

  printResult(
    {
      ...case_,
      status: CaseStatusToString(case_.status),
    },
    argv.output
  );
}

async function getCaseCount(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getCaseCount(), argv.output);
}

async function getCases(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const cases = await hapiCore.getCases(argv.skip, argv.take);

  console.log(
    cases.map((case_) => ({
      ...case_,
      status: CaseStatusToString(case_.status),
    })),
    argv.output
  );
}

async function createCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.createCase(argv.id, argv.name, argv.url),
    argv.output
  );
}

async function updateCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateCase(
      argv.id,
      argv.name,
      argv.url,
      CaseStatusFromString(argv.status)
    ),
    argv.output
  );
}

async function getAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAddress(argv.address), argv.output);
}

async function getAddressCount(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAddressCount(), argv.output);
}

async function getAddresses(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAddresses(argv.skip, argv.take), argv.output);
}

async function createAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.createAddress(
      argv.address,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    ),
    argv.output
  );
}

async function updateAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateAddress(
      argv.address,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    ),
    argv.output
  );
}

async function getAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAsset(argv.address, argv.id), argv.output);
}

async function getAssetCount(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAssetCount(), argv.output);
}

async function getAssets(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(await hapiCore.getAssets(argv.skip, argv.take), argv.output);
}

async function createAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.createAsset(
      argv.address,
      argv.assetId,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    ),
    argv.output
  );
}

async function updateAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  printResult(
    await hapiCore.updateAsset(
      argv.address,
      argv.assetId,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    ),
    argv.output
  );
}
