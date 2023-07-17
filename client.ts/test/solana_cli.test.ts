import { setup, killValidator } from "./setup";

import {
  cli_cmd,
  checkCommandResult,
  NETWORK,
  KEYS,
  REPORTERS,
  CASES,
  ADDRESSES,
  ASSETS,
  setupChai,
  expect,
  CommandCheck,
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
  Category,
  CategoryKeys,
  decodeAddress,
  getCategoryIndex,
} from "../../solana/lib";

describe("Solana Cli test", function () {
  setupChai();
  let program: HapiCoreProgram;

  before(async function () {
    process.env.ANCHOR_WALLET = KEYS.admin.path;
    program = new HapiCoreProgram(KEYS.program.pubkey);
    await setup(program.program.provider);
  });

  after(async function () {
    killValidator();
  });

  describe("Network authority", function () {
    it("Get authority check", async function () {
      const res = await cli_cmd("get-authority");
      const networkData = await program.getNetwotkData(NETWORK);

      checkCommandResult(res, networkData.authority.toString());
    });

    it("Set new authority by the program upgrade authority", async function () {
      let wallet = KEYS.authority;

      await cli_cmd("set-authority", `--address ${wallet.pubkey}`);
      process.env.ANCHOR_WALLET = wallet.path;

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(wallet.pubkey);
    });

    it("Set new authority by the current authority", async function () {
      let wallet = KEYS.admin;

      await cli_cmd("set-authority", `--address ${wallet.pubkey}`);
      process.env.ANCHOR_WALLET = wallet.path;

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(wallet.pubkey);
    });
  });

  describe("Stake configuration", function () {
    it("Get stake configuration", async function () {
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
      const token = KEYS.token.pubkey;
      const unlockDuration = 1;
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
    it("Get reward configuration", async function () {
      const res = await cli_cmd("get-reward-configuration");
      const networkData = await program.getNetwotkData(NETWORK);

      const val: RewardConfiguration = {
        token: networkData.rewardMint.toString(),
        addressConfirmationReward:
          networkData.rewardConfiguration.addressConfirmationReward.toString(),
        tracerReward:
          networkData.rewardConfiguration.addressTracerReward.toString(),
      };

      checkCommandResult(res, val);
    });

    it("Update reward configuration", async function () {
      const token = KEYS.token.pubkey;
      const addressConfirmationReward = 1001;
      const addressTracerReward = 2002;
      const assetConfirmationReward = 1001;
      const assetTracerReward = 2002;

      await cli_cmd(
        "update-reward-configuration",
        ` --token ${token} \
          --address-confirmation-reward ${addressConfirmationReward} \
          --address-tracer-reward ${addressTracerReward} \
          --asset-confirmation-reward ${assetConfirmationReward} \
          --asset-tracer-reward ${assetTracerReward}`
      );

      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.rewardMint.toString()).to.eq(token);
      expect(
        networkData.rewardConfiguration.addressConfirmationReward.toNumber()
      ).to.eq(addressConfirmationReward);
      expect(
        networkData.rewardConfiguration.addressTracerReward.toNumber()
      ).to.eq(addressTracerReward);
      expect(
        networkData.rewardConfiguration.assetConfirmationReward.toNumber()
      ).to.eq(assetConfirmationReward);
      expect(
        networkData.rewardConfiguration.assetTracerReward.toNumber()
      ).to.eq(assetTracerReward);
    });
  });

  describe("Reporter activation", function () {
    it("Verify that contract has no reporters", async function () {
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
           --account ${reporter.wallet.pubkey} \
           --name ${reporter.name} \
           --url ${reporter.url}`
        );

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        expect(bnToUuid(reporterData.id)).to.eq(reporter.id);
        expect(reporterData.account.toString()).to.eq(reporter.wallet.pubkey);
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

    it("Get reporters", async function () {
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

    it("Get all reporters", async function () {
      const res = await cli_cmd("get-reporters");

      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];

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

        checkCommandResult(res, val, CommandCheck.ToContain);
      }
    });

    it("Update reporter", async function () {
      const reporter = REPORTERS.authority;
      const newName = "newName";
      const newUrl = "https://new.authority.blockchain";

      await cli_cmd(
        "update-reporter",
        `--id ${reporter.id} \
         --role ${reporter.role} \
         --account ${reporter.wallet.pubkey} \
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
        expect(reporterData.stake).to.not.eq(0);
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
      process.env.ANCHOR_WALLET = reporter.wallet.path;

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
        expect(bnToUuid(caseData.reporterId)).to.eq(reporter.id);
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

      for (const key in CASES) {
        const cs = CASES[key];

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

        checkCommandResult(res, val, CommandCheck.ToContain);
      }
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

  describe("Address", function () {
    it("Verify that contract has no addresses", async function () {
      const count = await cli_cmd("get-address-count");
      checkCommandResult(count, 0);

      const addresses = await cli_cmd("get-addresses");
      checkCommandResult(addresses, []);
    });

    it("Create addresses", async function () {
      const reporter = REPORTERS.authority;
      process.env.ANCHOR_WALLET = reporter.wallet.path;

      for (const key in ADDRESSES) {
        const address = ADDRESSES[key];
        await cli_cmd(
          "create-address",
          `--address ${address.address} \
             --case-id ${address.caseId} \
             --risk ${address.riskScore}\
             --category ${address.category}`
        );
        const addressData = await program.getAddressData(
          NETWORK,
          address.address
        );
        expect(decodeAddress(addressData.address)).to.eq(address.address);
        expect(bnToUuid(addressData.caseId)).to.eq(address.caseId);
        expect(bnToUuid(addressData.reporterId)).to.eq(reporter.id);
        expect(addressData.riskScore).to.eq(address.riskScore);
        expect(addressData.category).to.deep.equal(
          Category[address.category as CategoryKeys]
        );
      }
    });

    it("Get addresses", async function () {
      for (const key in ADDRESSES) {
        const address = ADDRESSES[key];
        const res = await cli_cmd(
          "get-address",
          `--address ${address.address}`
        );
        const addressData = await program.getAddressData(
          NETWORK,
          address.address
        );
        const val = {
          address: decodeAddress(addressData.address),
          caseId: bnToUuid(addressData.caseId),
          reporterId: bnToUuid(addressData.reporterId),
          risk: addressData.riskScore,
          category: getCategoryIndex(addressData.category as typeof Category),
        };
        checkCommandResult(res, val);
      }
    });

    it("Verify address count", async function () {
      const count = await cli_cmd("get-address-count");
      checkCommandResult(count, Object.keys(ADDRESSES).length);
    });

    it("Get all addresses", async function () {
      const res = await cli_cmd("get-addresses");

      for (const key in ADDRESSES) {
        const address = ADDRESSES[key];
        const addressData = await program.getAddressData(
          NETWORK,
          address.address
        );

        const val = {
          address: decodeAddress(addressData.address),
          caseId: bnToUuid(addressData.caseId),
          reporterId: bnToUuid(addressData.reporterId),
          risk: addressData.riskScore,
          category: getCategoryIndex(addressData.category as typeof Category),
        };

        checkCommandResult(res, val, CommandCheck.ToContain);
      }
    });

    it("Update address", async function () {
      const address = ADDRESSES.firstAddr;
      process.env.ANCHOR_WALLET = REPORTERS.authority.wallet.path;

      const newRisk = 6;
      const newCategory = "DeFi";

      await cli_cmd(
        "update-address",
        `--address ${address.address} \
         --case-id ${address.caseId} \
         --risk ${newRisk} \
         --category ${newCategory}`
      );

      const addressData = await program.getAddressData(
        NETWORK,
        address.address
      );

      expect(addressData.riskScore).to.eq(newRisk);
      expect(addressData.category).to.deep.equal(Category.DeFi);
    });

    it("Confirm address", async function () {
      const address = ADDRESSES.firstAddr;
      process.env.ANCHOR_WALLET = REPORTERS.publisher.wallet.path;

      await cli_cmd("confirm-address", `--address ${address.address}`);

      const addressData = await program.getAddressData(
        NETWORK,
        address.address
      );

      expect(addressData.confirmations).to.eq(1);
    });
  });

  describe("Asset", function () {
    it("Verify that contract has no assets", async function () {
      const count = await cli_cmd("get-asset-count");
      checkCommandResult(count, 0);

      const assets = await cli_cmd("get-assets");
      checkCommandResult(assets, []);
    });

    it("Create assets", async function () {
      const reporter = REPORTERS.authority;
      process.env.ANCHOR_WALLET = reporter.wallet.path;

      for (const key in ASSETS) {
        const asset = ASSETS[key];

        await cli_cmd(
          "create-asset",
          ` --address ${asset.address} \
            --asset-id ${asset.assetId} \
            --case-id ${asset.caseId} \
            --risk ${asset.riskScore} \
            --category ${asset.category}`
        );
        const assetData = await program.getAssetData(
          NETWORK,
          asset.address,
          asset.assetId
        );

        expect(decodeAddress(assetData.address)).to.eq(asset.address);
        expect(decodeAddress(assetData.id)).to.eq(asset.assetId);
        expect(bnToUuid(assetData.caseId)).to.eq(asset.caseId);
        expect(bnToUuid(assetData.reporterId)).to.eq(reporter.id);
        expect(assetData.riskScore).to.eq(asset.riskScore);
        expect(assetData.category).to.deep.equal(
          Category[asset.category as CategoryKeys]
        );
      }
    });

    it("Get assets", async function () {
      for (const key in ASSETS) {
        const asset = ASSETS[key];

        const res = await cli_cmd(
          "get-asset",
          `--address ${asset.address} --asset-id ${asset.assetId}`
        );

        const assetData = await program.getAssetData(
          NETWORK,
          asset.address,
          asset.assetId
        );

        const val = {
          address: decodeAddress(assetData.address),
          assetId: decodeAddress(assetData.id),
          caseId: bnToUuid(assetData.caseId),
          reporterId: bnToUuid(assetData.reporterId),
          risk: assetData.riskScore,
          category: getCategoryIndex(assetData.category as typeof Category),
        };

        checkCommandResult(res, val);
      }
    });

    it("Verify asset count", async function () {
      const count = await cli_cmd("get-asset-count");
      checkCommandResult(count, Object.keys(ASSETS).length);
    });

    it("Get all assets", async function () {
      const res = await cli_cmd("get-assets");

      for (const key in ASSETS) {
        const asset = ASSETS[key];

        const assetData = await program.getAssetData(
          NETWORK,
          asset.address,
          asset.assetId
        );

        const val = {
          address: decodeAddress(assetData.address),
          assetId: decodeAddress(assetData.id),
          caseId: bnToUuid(assetData.caseId),
          reporterId: bnToUuid(assetData.reporterId),
          risk: assetData.riskScore,
          category: getCategoryIndex(assetData.category as typeof Category),
        };

        checkCommandResult(res, val, CommandCheck.ToContain);
      }
    });

    it("Update asset", async function () {
      const asset = ASSETS.firstAsset;
      process.env.ANCHOR_WALLET = REPORTERS.authority.wallet.path;

      const newRisk = 6;
      const newCategory = "DeFi";

      await cli_cmd(
        "update-asset",
        `--address ${asset.address} \
         --asset-id ${asset.assetId} \
         --case-id ${asset.caseId} \
         --risk ${newRisk} \
         --category ${newCategory}`
      );

      const assetData = await program.getAssetData(
        NETWORK,
        asset.address,
        asset.assetId
      );

      expect(assetData.riskScore).to.eq(newRisk);
      expect(assetData.category).to.deep.equal(Category.DeFi);
    });

    it("Confirm asset", async function () {
      const asset = ASSETS.firstAsset;
      process.env.ANCHOR_WALLET = REPORTERS.publisher.wallet.path;

      await cli_cmd(
        "confirm-asset",
        `--address ${asset.address} \
         --asset-id ${asset.assetId}`
      );

      const assetData = await program.getAssetData(
        NETWORK,
        asset.address,
        asset.assetId
      );

      expect(assetData.confirmations).to.eq(1);
    });
  });

  describe("Reporter deactivation", function () {
    it("Activate reporters", async function () {
      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];
        process.env.ANCHOR_WALLET = reporter.wallet.path;

        await cli_cmd("deactivate-reporter");

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        expect(reporterData.status).to.deep.equal(ReporterStatus.Unstaking);
      }
    });

    it("Unstake", async function () {
      for (const key in REPORTERS) {
        const reporter = REPORTERS[key];
        process.env.ANCHOR_WALLET = reporter.wallet.path;

        await cli_cmd("unstake-reporter");

        const reporterData = await program.getReporterData(
          NETWORK,
          reporter.id
        );

        expect(reporterData.status).to.deep.equal(ReporterStatus.Inactive);
        expect(reporterData.stake.toNumber()).to.eq(0);
      }
    });
  });
});
