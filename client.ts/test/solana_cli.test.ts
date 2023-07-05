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

import {
  ACCOUNT_SIZE,
  HapiCoreProgram,
  Category,
  uuidToBn,
  CaseStatus,
  decodeAddress,
} from "../../solana/lib";

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

      checkCommandResult(res, WALLET1);
      checkCommandResult(res, networkData.authority.toString());
    });

    it("Set new authority by the program upgrade authority", async function () {
      await run_cmd("set-authority", `--address ${WALLET2}`);
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(WALLET2);
    });

    it("Set new authority by the current authority", async function () {
      process.env.ANCHOR_WALLET = `${WALLET_PATH}/wallet_2.json`;
      checkCommandResult(
        await run_cmd("set-authority", `--address ${WALLET1}`),
        "transactionHash"
      );
      const networkData = await program.getNetwotkData(NETWORK);

      expect(networkData.authority.toString()).to.eq(WALLET1);
    });
  });

  describe("Stake configuration", function () {
    it("Get stake configuration", async function () {
      const res = await run_cmd("get-stake-configuration");
      console.log(res);

      const networkData = await program.getNetwotkData(NETWORK);

      const val = {
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

      checkCommandResult(res, `${val}`);
    });
  });
});
