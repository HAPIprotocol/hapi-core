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
  .option("private-key", {
    global: true,
    demandOption: false,
    description: "Signer's private key",
    type: "string",
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
        console.error(error);
      } else {
        const abstractError = error as any;
        console.error(
          `Error: ${
            abstractError.reason || abstractError.message || abstractError
          }`
        );
      }
    }
  };
}

async function getAuthority(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getAuthority());
}

async function setAuthority(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.setAuthority(argv.address));
}

async function getStakeConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getStakeConfiguration());
}

async function updateStakeConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateStakeConfiguration(
      argv.token,
      argv.unlockDuration,
      argv.validatorStake,
      argv.tracerStake,
      argv.publisherStake,
      argv.authorityStake
    )
  );
}

async function getRewardConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getRewardConfiguration());
}

async function updateRewardConfiguration(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateRewardConfiguration(
      argv.token,
      argv.addressConfirmationReward,
      argv.traceReward
    )
  );
}

async function getReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const reporter = await hapiCore.getReporter(argv.id.toString());

  console.log({
    ...reporter,
    role: ReporterRoleToString(reporter.role),
    status: ReporterStatusToString(reporter.status),
  });
}

async function createReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.createReporter(
      argv.id,
      ReporterRoleFromString(argv.role),
      argv.account,
      argv.name,
      argv.url
    )
  );
}

async function updateReporter(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateReporter(
      argv.id.toString(),
      ReporterRoleFromString(argv.role),
      argv.account,
      argv.name,
      argv.url
    )
  );
}

async function getCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  const case_ = await hapiCore.getCase(argv.id.toString());

  console.log({
    ...case_,
    status: CaseStatusToString(case_.status),
  });
}

async function createCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.createCase(argv.id, argv.name, argv.url));
}

async function updateCase(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateCase(
      argv.id,
      argv.name,
      argv.url,
      CaseStatusFromString(argv.status)
    )
  );
}

async function getAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getAddress(argv.address));
}

async function createAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.createAddress(
      argv.address,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    )
  );
}

async function updateAddress(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateAddress(
      argv.address,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    )
  );
}

async function getAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(await hapiCore.getAsset(argv.address, argv.id));
}

async function createAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.createAsset(
      argv.address,
      argv.assetId,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    )
  );
}

async function updateAsset(setup: Setup, argv: any) {
  const { hapiCore } = setup;

  console.log(
    await hapiCore.updateAsset(
      argv.address,
      argv.assetId,
      argv.caseId,
      argv.risk,
      CategoryFromString(argv.category)
    )
  );
}
