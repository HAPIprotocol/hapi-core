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
