import {
  run_cmd,
  setup,
  shutDownExisting,
  checkCommandResult,
  WALLET1,
  WALLET2,
  WALLET_PATH,
  PROGRAM_ADDRESS,
  NETWORK,
} from "./setup";

import { StakeConfiguration, RewardConfiguration } from "../src/interface";
import { HapiCoreProgram } from "../../solana/lib";

var expect = require("chai").expect;

describe("Solana Cli test", function () {
  process.env.ANCHOR_WALLET = `${WALLET_PATH}/wallet_1.json`;
  const program = new HapiCoreProgram(PROGRAM_ADDRESS);

  before(async function () {
    await setup();
  });

  after(async function () {
    await shutDownExisting(false);
  });

  describe("Network authority", function () {
    it("Get authority check", async function () {
      const res = await run_cmd("get-authority");
      const networkData = await program.getNetwotkData(NETWORK);

      checkCommandResult(res, networkData.authority.toString());
    });

    it("Set new authority by the program upgrade authority", async function () {
      await run_cmd("set-authority", `--address ${WALLET2}`);
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(WALLET2);
    });

    it("Set new authority by the current authority", async function () {
      process.env.ANCHOR_WALLET = `${WALLET_PATH}/wallet_2.json`;
      await run_cmd("set-authority", `--address ${WALLET1}`);
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(WALLET1);
    });
  });

  describe("Stake configuration", function () {
    it("Get stake configuration", async function () {
      const res = await run_cmd("get-stake-configuration");
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
      const token = "3edemy16mYEwURaQrRsFcy8vznwSs1nZR7FGCqqbpdeq";
      const unlockDuration = 123;
      const validatorStake = 1001;
      const tracerStake = 2002;
      const publisherStake = 3003;
      const authorityStake = 4004;

      await run_cmd(
        `update-stake-configuration \
      --token ${token} \
      --unlock-duration ${unlockDuration} \
      --validator-stake ${validatorStake}\
      --tracer-stake ${tracerStake}\
      --publisher-stake ${publisherStake} \
      --authority-stake ${authorityStake}`
      );
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.stakeMint).to.eq(token);
      expect(networkData.stakeConfiguration.unlockDuration.toNumber()).to.eq(
        unlockDuration
      );
      expect(networkData.stakeConfiguration.validatorStake.toString()).to.eq(
        validatorStake
      );
      expect(networkData.stakeConfiguration.tracerStake.toString()).to.eq(
        tracerStake
      );
      expect(networkData.stakeConfiguration.publisherStake.toString()).to.eq(
        publisherStake
      );
      expect(networkData.stakeConfiguration.authorityStake.toString()).to.eq(
        authorityStake
      );
    });
  });

  describe("Reward configuration", function () {
    it("Get reward configuration", async function () {
      const res = await run_cmd("get-reward-configuration");
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

    it("Update reward configuration", async function () {
      const token = "3edemy16mYEwURaQrRsFcy8vznwSs1nZR7FGCqqbpdeq";
      const addressConfirmationReward = 1001;
      const tracerReward = 2002;

      await run_cmd(
        `update-reward-configuration \
      --token ${token} \
      --address-confirmation-reward ${addressConfirmationReward} \
      --trace-reward ${tracerReward}`
      );
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.rewardMint).to.eq(token);
      expect(
        networkData.rewardConfiguration.addressConfirmationReward.toString()
      ).to.eq(addressConfirmationReward);
      expect(
        networkData.rewardConfiguration.addressTracerReward.toString()
      ).to.eq(tracerReward);
    });
  });
});
