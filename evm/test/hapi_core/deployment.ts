import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";

import { setupContract } from "../setup";

describe("HapiCore: Deployment", function () {
  async function basicFixture() {
    let setup = await setupContract();

    const [owner, authority, nobody] = await ethers.getSigners();

    return { ...setup, owner, authority, nobody };
  }

  it("Should set the right owner and authority", async function () {
    const { hapiCore, owner } = await loadFixture(basicFixture);

    const authorityRole = await hapiCore.AUTHORITY_ROLE();

    expect(await hapiCore.owner()).to.equal(owner.address);

    expect(await hapiCore.hasRole(authorityRole, owner.address)).to.equal(true);
  });

  it("Should correctly set authority from owner", async function () {
    const { hapiCore, owner, authority } = await loadFixture(basicFixture);

    const authorityRole = await hapiCore.AUTHORITY_ROLE();

    await expect(await hapiCore.grantRole(authorityRole, authority.address))
      .to.emit(hapiCore, "RoleGranted")
      .withArgs(authorityRole, authority.address, owner.address);

    expect(await hapiCore.hasRole(authorityRole, authority.address)).to.equal(
      true
    );
  });

  it("Should not allow setting authority from non-owner/non-authority", async function () {
    const { hapiCore, authority, nobody } = await loadFixture(basicFixture);

    const authorityRole = await hapiCore.AUTHORITY_ROLE();

    await expect(
      hapiCore.connect(nobody).grantRole(authorityRole, authority.address)
    ).to.be.revertedWithCustomError(hapiCore, "AccessControlUnauthorizedAccount");
  });
});
