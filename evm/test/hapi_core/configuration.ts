import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";

import { basicFixture } from "../setup";

describe("HapiCore: Configuration", function () {
  const UNLOCK_DURATION = 3600;
  const VALIDATOR_STAKE = 101;
  const TRACER_STAKE = 102;
  const PUBLISHER_STAKE = 103;
  const AUTHORITY_STAKE = 104;
  const ADDRESS_CONFIRMATION_REWARD = 201;
  const ADDRESS_TRACER_REWARD = 202;
  const ASSET_CONFIRMATION_REWARD = 203;
  const ASSET_TRACER_REWARD = 204;

  it("Should update stake configuration", async function () {
    const { hapiCore } = await loadFixture(basicFixture);

    await expect(hapiCore.stakeConfiguration())
      .to.be.revertedWithCustomError(hapiCore, "ContractNotConfigured")
      .withArgs();

    const stakeTokenAddress = "0xdEADBEeF00000000000000000000000000000000";

    await expect(
      await hapiCore.updateStakeConfiguration(
        stakeTokenAddress,
        UNLOCK_DURATION,
        VALIDATOR_STAKE,
        TRACER_STAKE,
        PUBLISHER_STAKE,
        AUTHORITY_STAKE
      )
    )
      .to.emit(hapiCore, "StakeConfigurationChanged")
      .withArgs(
        stakeTokenAddress,
        UNLOCK_DURATION,
        VALIDATOR_STAKE,
        TRACER_STAKE,
        PUBLISHER_STAKE,
        AUTHORITY_STAKE
      );

    expect(await hapiCore.stakeConfiguration()).to.deep.equal([
      stakeTokenAddress,
      UNLOCK_DURATION,
      VALIDATOR_STAKE,
      TRACER_STAKE,
      PUBLISHER_STAKE,
      AUTHORITY_STAKE,
    ]);
  });

  it("Should update reward configuration", async function () {
    const { hapiCore } = await loadFixture(basicFixture);

    await expect(hapiCore.rewardConfiguration())
      .to.be.revertedWithCustomError(hapiCore, "ContractNotConfigured")
      .withArgs();

    const rewardTokenAddress = "0xdEADBEeF00000000000000000000000000000000";

    await expect(
      await hapiCore.updateRewardConfiguration(
        rewardTokenAddress,
        ADDRESS_CONFIRMATION_REWARD,
        ADDRESS_TRACER_REWARD,
        ASSET_CONFIRMATION_REWARD,
        ASSET_TRACER_REWARD
      )
    )
      .to.emit(hapiCore, "RewardConfigurationChanged")
      .withArgs(
        rewardTokenAddress,
        ADDRESS_CONFIRMATION_REWARD,
        ADDRESS_TRACER_REWARD,
        ASSET_CONFIRMATION_REWARD,
        ASSET_TRACER_REWARD
      );

    expect(await hapiCore.rewardConfiguration()).to.deep.equal([
      rewardTokenAddress,
      ADDRESS_CONFIRMATION_REWARD,
      ADDRESS_TRACER_REWARD,
      ASSET_CONFIRMATION_REWARD,
      ASSET_TRACER_REWARD,
    ]);
  });
});
