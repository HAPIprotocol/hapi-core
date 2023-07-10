import { setup, killValidator } from "./setup";
import {
  cli_cmd,
  checkCommandResult,
  NETWORK,
  KEYS,
  REPORTERS,
} from "./helpers";

import {
  StakeConfiguration,
  RewardConfiguration,
  ReporterRole,
  ReporterStatus,
} from "../src/interface";

import { ReporterRoleToString, ReporterStatusToString } from "../src/util";

import {
  HapiCoreProgram,
  bnToUuid,
  ReporterRole as SolReporterRole,
  ReporterStatus as SolReporterStatus,
  getReporterRoleIndex,
  getReporterStatusIndex,
} from "../../solana/lib";

const chai = require("chai");
chai.config.truncateThreshold = 0;
chai.config.showDiff = true;
var expect = chai.expect;

describe("Solana Cli test", function () {
  process.env.ANCHOR_WALLET = KEYS.wallet1.path;
  const program = new HapiCoreProgram(KEYS.program.pk);

  before(async function () {
    await setup();
  });

  after(async function () {
    killValidator();
  });

  // describe("Network authority", function () {
  //   it("Get authority check", async function () {
  //     const res = await cli_cmd("get-authority");
  //     const networkData = await program.getNetwotkData(NETWORK);

  //     checkCommandResult(res, networkData.authority.toString());
  //   });

  //   it("Set new authority by the program upgrade authority", async function () {
  //     let wallet = KEYS.wallet2;

  //     await cli_cmd("set-authority", `--address ${wallet.pk}`);
  //     process.env.ANCHOR_WALLET = wallet.path;

  //     const networkData = await program.getNetwotkData(NETWORK);

  //     expect(networkData.authority.toString()).to.eq(wallet.pk);
  //   });

  //   it("Set new authority by the current authority", async function () {
  //     let wallet = KEYS.wallet1;

  //     await cli_cmd("set-authority", `--address ${wallet.pk}`);
  //     process.env.ANCHOR_WALLET = wallet.path;

  //     const networkData = await program.getNetwotkData(NETWORK);

  //     expect(networkData.authority.toString()).to.eq(wallet.pk);
  //   });
  // });

  // describe("Stake configuration", function () {
  //   it("Get stake configuration", async function () {
  //     const res = await cli_cmd("get-stake-configuration");
  //     const networkData = await program.getNetwotkData(NETWORK);

  //     const val: StakeConfiguration = {
  //       token: networkData.stakeMint.toString(),
  //       unlockDuration:
  //         networkData.stakeConfiguration.unlockDuration.toNumber(),
  //       validatorStake:
  //         networkData.stakeConfiguration.validatorStake.toString(),
  //       tracerStake: networkData.stakeConfiguration.tracerStake.toString(),
  //       publisherStake:
  //         networkData.stakeConfiguration.publisherStake.toString(),
  //       authorityStake:
  //         networkData.stakeConfiguration.authorityStake.toString(),
  //     };

  //     checkCommandResult(res, val);
  //   });

  //   it("Update stake configuration", async function () {
  //     const token = KEYS.token.pk;
  //     const unlockDuration = 123;
  //     const validatorStake = 1001;
  //     const tracerStake = 2002;
  //     const publisherStake = 3003;
  //     const authorityStake = 4004;

  //     await cli_cmd(
  //       "update-stake-configuration",
  //       ` --token ${token} \
  //         --unlock-duration ${unlockDuration} \
  //         --validator-stake ${validatorStake}\
  //         --tracer-stake ${tracerStake}\
  //         --publisher-stake ${publisherStake} \
  //         --authority-stake ${authorityStake}`
  //     );
  //     const networkData = await program.getNetwotkData(NETWORK);

  //     expect(networkData.stakeMint.toString()).to.eq(token);
  //     expect(networkData.stakeConfiguration.unlockDuration.toNumber()).to.eq(
  //       unlockDuration
  //     );
  //     expect(networkData.stakeConfiguration.validatorStake.toNumber()).to.eq(
  //       validatorStake
  //     );
  //     expect(networkData.stakeConfiguration.tracerStake.toNumber()).to.eq(
  //       tracerStake
  //     );
  //     expect(networkData.stakeConfiguration.publisherStake.toNumber()).to.eq(
  //       publisherStake
  //     );
  //     expect(networkData.stakeConfiguration.authorityStake.toNumber()).to.eq(
  //       authorityStake
  //     );
  //   });
  // });

  // describe("Reward configuration", function () {
  //   it("Get reward configuration", async function () {
  //     const res = await cli_cmd("get-reward-configuration");
  //     const networkData = await program.getNetwotkData(NETWORK);

  //     // TODO: add asset configuration
  //     const val: RewardConfiguration = {
  //       token: networkData.rewardMint.toString(),
  //       addressConfirmationReward:
  //         networkData.rewardConfiguration.addressConfirmationReward.toString(),
  //       tracerReward:
  //         networkData.rewardConfiguration.addressTracerReward.toString(),
  //     };

  //     checkCommandResult(res, val);
  //   });

  //   it("Update reward configuration", async function () {
  //     const token = KEYS.token.pk;
  //     const addressConfirmationReward = 1001;
  //     const tracerReward = 2002;

  //     await cli_cmd(
  //       "update-reward-configuration",
  //       ` --token ${token} \
  //         --address-confirmation-reward ${addressConfirmationReward} \
  //         --trace-reward ${tracerReward}`
  //     );
  //     const networkData = await program.getNetwotkData(NETWORK);

  //     expect(networkData.rewardMint.toString()).to.eq(token);
  //     expect(
  //       networkData.rewardConfiguration.addressConfirmationReward.toNumber()
  //     ).to.eq(addressConfirmationReward);
  //     expect(
  //       networkData.rewardConfiguration.addressTracerReward.toNumber()
  //     ).to.eq(tracerReward);
  //   });
  // });

  describe("Reporter", function () {
    xit("Verify that contract has no reporters", async function () {
      const count = await cli_cmd("get-reporter-count");
      checkCommandResult(count, 0);

      const reporters = await cli_cmd("get-reporters");
      checkCommandResult(reporters, []);
    });

    it("Create authority reporter", async function () {
      const reporter = REPORTERS.authority;

      await cli_cmd(
        "create-reporter",
        `--id ${reporter.id} \
         --role ${reporter.role} \
         --account ${reporter.wallet.pk} \
         --name ${reporter.name} \
         --url ${reporter.url}`
      );

      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      expect(bnToUuid(reporterData.id)).to.eq(reporter.id);
      expect(reporterData.account.toString()).to.eq(reporter.wallet.pk);
      expect(reporterData.role).to.deep.equal(SolReporterRole.Authority);
      expect(reporterData.status).to.deep.equal(SolReporterStatus.Inactive);
      expect(reporterData.name).to.eq(reporter.name);
      expect(reporterData.url).to.eq(reporter.url);
      expect(reporterData.stake.toString()).to.eq("0");
      expect(reporterData.unlockTimestamp.toNumber()).to.eq(0);
    });

    xit("Get authority reporter", async function () {
      const reporter = REPORTERS.authority;

      const res = await cli_cmd("get-reporter", `--id ${reporter.id}`);
      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      const val = {
        id: bnToUuid(reporterData.id),
        account: reporterData.account.toString(),
        role: ReporterRoleToString(
          getReporterRoleIndex(
            reporterData.role as typeof SolReporterRole
          ) as ReporterRole
        ),
        status: ReporterStatusToString(
          getReporterStatusIndex(
            reporterData.status as typeof SolReporterStatus
          ) as ReporterStatus
        ),
        name: reporterData.name.toString(),
        url: reporterData.url,
        stake: reporterData.stake.toString(),
        unlockTimestamp: reporterData.unlockTimestamp.toNumber(),
      };

      checkCommandResult(res, val);
    });

    xit("Create publisher reporter", async function () {
      const reporter = REPORTERS.publisher;

      await cli_cmd(
        "create-reporter",
        `--id ${reporter.id} \
         --role ${reporter.role} \
         --account ${reporter.wallet.pk} \
         --name ${reporter.name} \
         --url ${reporter.url}`
      );

      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      expect(bnToUuid(reporterData.id)).to.eq(reporter.id);
      expect(reporterData.account.toString()).to.eq(reporter.wallet.pk);
      expect(reporterData.role).to.deep.equal(SolReporterRole.Publisher);
      expect(reporterData.status).to.deep.equal(SolReporterStatus.Inactive);
      expect(reporterData.name).to.eq(reporter.name);
      expect(reporterData.url).to.eq(reporter.url);
      expect(reporterData.stake.toString()).to.eq("0");
      expect(reporterData.unlockTimestamp.toNumber()).to.eq(0);
    });

    xit("Get authority reporter", async function () {
      const reporter = REPORTERS.publisher;

      const res = await cli_cmd("get-reporter", `--id ${reporter.id}`);
      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      const val = {
        id: bnToUuid(reporterData.id),
        account: reporterData.account.toString(),
        role: ReporterRoleToString(
          getReporterRoleIndex(
            reporterData.role as typeof SolReporterRole
          ) as ReporterRole
        ),
        status: ReporterStatusToString(
          getReporterStatusIndex(
            reporterData.status as typeof SolReporterStatus
          ) as ReporterStatus
        ),
        name: reporterData.name.toString(),
        url: reporterData.url,
        stake: reporterData.stake.toString(),
        unlockTimestamp: reporterData.unlockTimestamp.toNumber(),
      };

      checkCommandResult(res, val);
    });

    xit("Verify reporter count", async function () {
      const count = await cli_cmd("get-reporter-count");

      checkCommandResult(count, 2);
    });

    xit("Get all reporters", async function () {
      const res = await cli_cmd("get-reporters");
      const val = [];

      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        val.push({
          id: bnToUuid(reporterData.id),
          account: reporterData.account.toString(),
          role: ReporterRoleToString(
            getReporterRoleIndex(
              reporterData.role as typeof SolReporterRole
            ) as ReporterRole
          ),
          status: ReporterStatusToString(
            getReporterStatusIndex(
              reporterData.status as typeof SolReporterStatus
            ) as ReporterStatus
          ),
          name: reporterData.name.toString(),
          url: reporterData.url,
          stake: reporterData.stake.toString(),
          unlockTimestamp: reporterData.unlockTimestamp.toNumber(),
        });
      }

      checkCommandResult(res, val);
    });

    xit("Update reporter", async function () {
      const reporter = REPORTERS.authority;
      const newName = "newName";
      const newUrl = "https://new.authority.blockchain";

      await cli_cmd(
        "update-reporter",
        `--id ${reporter.id} \
         --role ${reporter.role} \
         --account ${reporter.wallet.pk} \
         --name ${newName} \
         --url ${newUrl}`
      );

      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      expect(reporterData.name).to.eq(newName);
      expect(reporterData.url).to.eq(newUrl);
    });

    it("Activate reporter", async function () {
      const reporter = REPORTERS.authority;
      process.env.ANCHOR_WALLET = reporter.wallet.path;

      console.log("Activate reporter");
      console.log(await cli_cmd("activate-reporter"));

      const reporterData = await program.getReporterData(NETWORK, reporter.id);

      expect(reporterData.status).to.deep.equal(SolReporterStatus.Active);
    });
  });
});
