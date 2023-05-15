import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { basicFixture } from "../setup";

describe("HapiCore: Configuration", function () {
  const UNLOCK_DURATION = 3600;
  const VALIDATOR_STAKE = 101;
  const TRACER_STAKE = 102;
  const PUBLISHER_STAKE = 103;
  const AUTHORITY_STAKE = 104;
  const ADDRESS_CONFIRMATION_REWARD = 201;
  const REPORT_REWARD = 202;

  it("Should update stake configuration", async function () {
    const { hapiCore } = await loadFixture(basicFixture);

    expect(await hapiCore.stakeConfiguration()).to.deep.equal([
      ethers.constants.AddressZero,
      0,
      0,
      0,
      0,
      0,
    ]);

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

    expect(await hapiCore.rewardConfiguration()).to.deep.equal([
      ethers.constants.AddressZero,
      0,
      0,
    ]);

    const rewardTokenAddress = "0xdEADBEeF00000000000000000000000000000000";

    await expect(
      await hapiCore.updateRewardConfiguration(
        rewardTokenAddress,
        ADDRESS_CONFIRMATION_REWARD,
        REPORT_REWARD
      )
    )
      .to.emit(hapiCore, "RewardConfigurationChanged")
      .withArgs(rewardTokenAddress, ADDRESS_CONFIRMATION_REWARD, REPORT_REWARD);

    expect(await hapiCore.rewardConfiguration()).to.deep.equal([
      rewardTokenAddress,
      ADDRESS_CONFIRMATION_REWARD,
      REPORT_REWARD,
    ]);
  });
});
