import { setup, killValidator } from "./setup";
import {
  cli_cmd,
  checkCommandResult,
  NETWORK,
  KEYS,
  REPORTERS,
  CASES,
} from "./helpers";

import {
  StakeConfiguration,
  RewardConfiguration,
  ReporterRole as CliReporterRole,
  ReporterStatus as CliReporterStatus,
  CaseStatus as CliCaseStatus,
} from "../src/interface";

import {
  ReporterRoleToString,
  ReporterStatusToString,
  CaseStatusToString,
} from "../src/util";

import {
  HapiCoreProgram,
  bnToUuid,
  ReporterRole,
  ReporterStatus,
  getReporterRoleIndex,
  getReporterStatusIndex,
  getCaseStatusIndex,
  ReporterRoleKeys,
  CaseStatus,
} from "../../solana/lib";
import { log } from "console";

const chai = require("chai");
chai.config.truncateThreshold = 0;
chai.config.showDiff = true;
var expect = chai.expect;

describe("Solana Cli test", function () {
  let program: HapiCoreProgram;

  before(async function () {
    process.env.ANCHOR_WALLET = KEYS.admin.path;
    program = new HapiCoreProgram(KEYS.program.pk);
    await setup(program.program.provider);
  });

  after(async function () {
    killValidator();
  });

  describe("Network authority", function () {
    xit("Get authority check", async function () {
      const res = await cli_cmd("get-authority");
      const networkData = await program.getNetwotkData(NETWORK);

      checkCommandResult(res, networkData.authority.toString());
    });

    xit("Set new authority by the program upgrade authority", async function () {
      let wallet = KEYS.authority;

      await cli_cmd("set-authority", `--address ${wallet.pk}`);
      process.env.ANCHOR_WALLET = wallet.path;

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(wallet.pk);
    });

    xit("Set new authority by the current authority", async function () {
      let wallet = KEYS.admin;

      await cli_cmd("set-authority", `--address ${wallet.pk}`);
      process.env.ANCHOR_WALLET = wallet.path;

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(wallet.pk);
    });
  });

  describe("Stake configuration", function () {
    xit("Get stake configuration", async function () {
      const res = await cli_cmd("get-stake-configuration");
      const networkData = await program.getNetwotkData(NETWORK);

      const val: StakeConfiguration = {
        token: networkData.stakeMint.toString(),
        unlockDuration:
          networkData.stakeConfiguration.unlockDuration.toNumber(),
        validatorStake:
          networkData.stakeConfiguration.validatorStake.toString(),
        tracerStake: networkData.stakeConfiguration.tracerStake.toString(),
        publisherStake:
          networkData.stakeConfiguration.publisherStake.toString(),
        authorityStake:
          networkData.stakeConfiguration.authorityStake.toString(),
      };

      checkCommandResult(res, val);
    });

    it("Update stake configuration", async function () {
      const token = KEYS.token.pk;
      const unlockDuration = 123;
      const validatorStake = 1001;
      const tracerStake = 2002;
      const publisherStake = 3003;
      const authorityStake = 4004;

      await cli_cmd(
        "update-stake-configuration",
        ` --token ${token} \
          --unlock-duration ${unlockDuration} \
          --validator-stake ${validatorStake}\
          --tracer-stake ${tracerStake}\
          --publisher-stake ${publisherStake} \
          --authority-stake ${authorityStake}`
      );

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.stakeMint.toString()).to.eq(token);
      expect(networkData.stakeConfiguration.unlockDuration.toNumber()).to.eq(
        unlockDuration
      );
      expect(networkData.stakeConfiguration.validatorStake.toNumber()).to.eq(
        validatorStake
      );
      expect(networkData.stakeConfiguration.tracerStake.toNumber()).to.eq(
        tracerStake
      );
      expect(networkData.stakeConfiguration.publisherStake.toNumber()).to.eq(
        publisherStake
      );
      expect(networkData.stakeConfiguration.authorityStake.toNumber()).to.eq(
        authorityStake
      );
    });
  });

  describe("Reward configuration", function () {
    xit("Get reward configuration", async function () {
      const res = await cli_cmd("get-reward-configuration");
      const networkData = await program.getNetwotkData(NETWORK);

      // TODO: add asset configuration
      const val: RewardConfiguration = {
        token: networkData.rewardMint.toString(),
        addressConfirmationReward:
          networkData.rewardConfiguration.addressConfirmationReward.toString(),
        tracerReward:
          networkData.rewardConfiguration.addressTracerReward.toString(),
      };

      checkCommandResult(res, val);
    });

    xit("Update reward configuration", async function () {
      const token = KEYS.token.pk;
      const addressConfirmationReward = 1001;
      const tracerReward = 2002;

      await cli_cmd(
        "update-reward-configuration",
        ` --token ${token} \
          --address-confirmation-reward ${addressConfirmationReward} \
          --trace-reward ${tracerReward}`
      );
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.rewardMint.toString()).to.eq(token);
      expect(
        networkData.rewardConfiguration.addressConfirmationReward.toNumber()
      ).to.eq(addressConfirmationReward);
      expect(
        networkData.rewardConfiguration.addressTracerReward.toNumber()
      ).to.eq(tracerReward);
    });
  });

  describe("Reporter activation", function () {
    xit("Verify that contract has no reporters", async function () {
      const count = await cli_cmd("get-reporter-count");
      checkCommandResult(count, 0);

      const reporters = await cli_cmd("get-reporters");
      checkCommandResult(reporters, []);
    });

    it("Create reporters", async function () {
      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];

        await cli_cmd(
          "create-reporter",
          `--id ${reporter.id} \
           --role ${reporter.role} \
           --account ${reporter.wallet.pk} \
           --name ${reporter.name} \
           --url ${reporter.url}`
        );

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        expect(bnToUuid(reporterData.id)).to.eq(reporter.id);
        expect(reporterData.account.toString()).to.eq(reporter.wallet.pk);
        expect(reporterData.role).to.deep.equal(
          ReporterRole[reporter.role as ReporterRoleKeys]
        );
        expect(reporterData.status).to.deep.equal(ReporterStatus.Inactive);
        expect(reporterData.name).to.eq(reporter.name);
        expect(reporterData.url).to.eq(reporter.url);
        expect(reporterData.stake.toString()).to.eq("0");
        expect(reporterData.unlockTimestamp.toNumber()).to.eq(0);
      }
    });

    xit("Get reporters", async function () {
      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];

        const res = await cli_cmd("get-reporter", `--id ${reporter.id}`);
        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        const val = {
          id: bnToUuid(reporterData.id),
          account: reporterData.account.toString(),
          role: ReporterRoleToString(
            getReporterRoleIndex(
              reporterData.role as typeof ReporterRole
            ) as CliReporterRole
          ),
          status: ReporterStatusToString(
            getReporterStatusIndex(
              reporterData.status as typeof ReporterStatus
            ) as CliReporterStatus
          ),
          name: reporterData.name.toString(),
          url: reporterData.url,
          stake: reporterData.stake.toString(),
          unlockTimestamp: reporterData.unlockTimestamp.toNumber(),
        };

        checkCommandResult(res, val);
      }
    });

    it("Verify reporter count", async function () {
      const count = await cli_cmd("get-reporter-count");

      checkCommandResult(count, Object.keys(REPORTERS).length);
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
              reporterData.role as typeof ReporterRole
            ) as CliReporterRole
          ),
          status: ReporterStatusToString(
            getReporterStatusIndex(
              reporterData.status as typeof ReporterStatus
            ) as CliReporterStatus
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

    it("Activate reporters", async function () {
      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];
        process.env.ANCHOR_WALLET = reporter.wallet.path;

        await cli_cmd("activate-reporter");

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        expect(reporterData.status).to.deep.equal(ReporterStatus.Active);
      }
    });
  });

  describe("Case", function () {
    it("Verify that contract has no cases", async function () {
      const count = await cli_cmd("get-case-count");
      checkCommandResult(count, 0);

      const cases = await cli_cmd("get-cases");
      checkCommandResult(cases, []);
    });

    it("Create cases", async function () {
      const reporter = REPORTERS.authority;

      for (const key in CASES) {
        const cs = CASES[key];

        await cli_cmd(
          "create-case",
          `--id ${cs.id} \
           --name ${cs.name} \
           --url ${cs.url}`
        );

        const caseData = await program.getCaseData(NETWORK, cs.id);

        expect(bnToUuid(caseData.id)).to.eq(cs.id);
        expect(caseData.name).to.eq(cs.name);
        expect(caseData.url).to.eq(cs.url);
        expect(caseData.status).to.deep.equal(CaseStatus.Open);
      }
    });

    it("Get cases", async function () {
      for (const key in CASES) {
        const cs = CASES[key];

        const res = await cli_cmd("get-case", `--id ${cs.id}`);
        const caseData = await program.getCaseData(NETWORK, cs.id);

        const val = {
          id: bnToUuid(caseData.id),
          name: caseData.name.toString(),
          url: caseData.url,
          status: CaseStatusToString(
            getCaseStatusIndex(
              caseData.status as typeof CaseStatus
            ) as CliCaseStatus
          ),
        };

        checkCommandResult(res, val);
      }
    });

    it("Verify case count", async function () {
      const count = await cli_cmd("get-case-count");

      checkCommandResult(count, Object.keys(CASES).length);
    });

    it("Get all cases", async function () {
      const res = await cli_cmd("get-cases");
      const val = [];

      for (const key in CASES) {
        const cs = CASES[key];

        const caseData = await program.getCaseData(NETWORK, cs.id);

        val.push({
          id: bnToUuid(caseData.id),
          name: caseData.name.toString(),
          url: caseData.url,
          status: CaseStatusToString(
            getCaseStatusIndex(
              caseData.status as typeof CaseStatus
            ) as CliCaseStatus
          ),
        });
      }

      checkCommandResult(res, val);
    });

    it("Update case", async function () {
      const cs = CASES.secondCase;

      const newName = "newName";
      const newUrl = "https://new.case.blockchain";
      const newStatus = "Closed";

      await cli_cmd(
        "update-case",
        `--id ${cs.id} \
         --name ${newName} \
         --url ${newUrl} \
         --status ${newStatus}`
      );

      const caseData = await program.getCaseData(NETWORK, cs.id);

      expect(caseData.name).to.eq(newName);
      expect(caseData.url).to.eq(newUrl);
      expect(caseData.status).to.deep.equal(CaseStatus.Closed);
    });
  });
});
