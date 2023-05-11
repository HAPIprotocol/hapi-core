import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import '@openzeppelin/hardhat-upgrades';
import { ethers, upgrades } from "hardhat";


describe("HapiCore", function () {
  async function basicFixture() {
    const [owner, otherAccount] = await ethers.getSigners();

    const HapiCore = await ethers.getContractFactory("HapiCore");

    const hapi = await upgrades.deployProxy(HapiCore, [], {
      initializer: 'initialize',
    });

    await hapi.deployed();

    return { hapi, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      const { hapi, owner } = await loadFixture(basicFixture);

      expect(await hapi.owner()).to.equal(owner.address);
    });
  });
});
