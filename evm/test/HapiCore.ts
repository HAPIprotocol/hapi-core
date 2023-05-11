import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { setupContract } from "./setup";

describe("HapiCore", function () {
  async function basicFixture() {
    let setup = await setupContract();

    const [owner, authority, nobody] = await ethers.getSigners();

    return { ...setup, owner, authority, nobody };
  }

  describe("Deployment", function () {
    it("Should set the right owner and authority", async function () {
      const { hapiCore, owner } = await loadFixture(basicFixture);

      expect(await hapiCore.owner()).to.equal(owner.address);

      expect(await hapiCore.authority()).to.equal(owner.address);
    });

    it("Should correctly set authority from owner", async function () {
      const { hapiCore, authority } = await loadFixture(basicFixture);

      await expect(await hapiCore.setAuthority(authority.address))
        .to.emit(hapiCore, "AuthorityChanged")
        .withArgs(authority.address);

      expect(await hapiCore.authority()).to.equal(authority.address);
    });

    it("Should correctly set authority from previous authority", async function () {
      const { hapiCore, authority, nobody } = await loadFixture(basicFixture);

      await expect(await hapiCore.setAuthority(authority.address))
        .to.emit(hapiCore, "AuthorityChanged")
        .withArgs(authority.address);

      expect(await hapiCore.authority()).to.equal(authority.address);

      await expect(await hapiCore.connect(authority).setAuthority(nobody.address))
        .to.emit(hapiCore, "AuthorityChanged")
        .withArgs(nobody.address);

      expect(await hapiCore.authority()).to.equal(nobody.address);
    });

    it("Should not allow setting authority from non-owner/non-authority", async function () {
      const { hapiCore, authority, nobody } = await loadFixture(basicFixture);

      await expect(
        hapiCore.connect(nobody).setAuthority(authority.address)
      ).to.be.revertedWith("Caller is not the owner or authority");
    });
  });

  describe("Configuration", function () {
    it("Should update stake configuration", async function () {
      const { hapiCore } = await loadFixture(basicFixture);

      expect(await hapiCore.stakeConfiguration()).to.deep.equal([ethers.constants.AddressZero, 0, 0, 0, 0, 0]);

      const stakeTokenAddress = "0xdEADBEeF00000000000000000000000000000000";

      await expect(await hapiCore.updateStakeConfiguration(stakeTokenAddress, 3600, 101, 102, 103, 104))
        .to.emit(hapiCore, "StakeConfigurationChanged")
        .withArgs(stakeTokenAddress, 3600, 101, 102, 103, 104);

      expect(await hapiCore.stakeConfiguration()).to.deep.equal([stakeTokenAddress, 3600, 101, 102, 103, 104]);
    });

    it("Should update reward configuration", async function () {
      const { hapiCore } = await loadFixture(basicFixture);

      expect(await hapiCore.rewardConfiguration()).to.deep.equal([ethers.constants.AddressZero, 0, 0]);

      const rewardTokenAddress = "0xdEADBEeF00000000000000000000000000000000";

      await expect(await hapiCore.updateRewardConfiguration(rewardTokenAddress, 101, 102))
        .to.emit(hapiCore, "RewardConfigurationChanged")
        .withArgs(rewardTokenAddress, 101, 102);

      expect(await hapiCore.rewardConfiguration()).to.deep.equal([rewardTokenAddress, 101, 102]);
    });
  });
});
