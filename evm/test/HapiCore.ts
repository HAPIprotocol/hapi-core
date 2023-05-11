import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { setupContract } from "./setup";

describe("HapiCore", function () {
  async function basicFixture() {
    let setup = await setupContract();

    const [owner, otherAccount] = await ethers.getSigners();

    return { ...setup, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      const { hapiCore, owner } = await loadFixture(basicFixture);

      expect(await hapiCore.owner()).to.equal(owner.address);
    });
  });
});
