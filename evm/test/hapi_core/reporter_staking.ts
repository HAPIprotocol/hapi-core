import { loadFixture, time } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";

import { fixtureWithToken } from "../setup";
import { ReporterRole, ReporterStatus, randomId } from "../util";

describe("HapiCore: Reporter staking", function () {
  it("Should stake for a reporter", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    const balanceBefore = await token.balanceOf(wallets.publisher.address);

    // Shouldn't be able to stake if not approved
    await expect(
      hapiCore.connect(wallets.publisher).activateReporter()
    ).to.be.revertedWith("ERC20: insufficient allowance");

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    expect(await hapiCore.connect(wallets.publisher).activateReporter())
      .to.emit(hapiCore, "ReporterActivated")
      .withArgs(reporterAccount.id);

    expect(await hapiCore.getReporter(reporterAccount.id)).to.deep.equal([
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.name,
      reporterAccount.url,
      reporterAccount.role,
      ReporterStatus.Active,
      cfg.PUBLISHER_STAKE,
      0,
    ]);

    const balanceAfter = await token.balanceOf(wallets.publisher.address);

    expect(balanceBefore.sub(balanceAfter)).to.equal(cfg.PUBLISHER_STAKE);
  });

  it("Should not stake for a reporter if not enough balance", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    // Give away tokens to someone else
    await token
      .connect(wallets.publisher)
      .transfer(
        wallets.nobody.address,
        await token.balanceOf(wallets.publisher.address)
      );

    await expect(
      hapiCore.connect(wallets.publisher).activateReporter()
    ).to.be.revertedWith("ERC20: insufficient allowance");

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await expect(
      hapiCore.connect(wallets.publisher).activateReporter()
    ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
  });

  it("Should not be able to activate someone else's reporter account", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    await token
      .connect(wallets.nobody)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await expect(
      hapiCore.connect(wallets.nobody).activateReporter()
    ).to.be.revertedWith('Caller is not a reporter');
  });

  it("Should deactivate a reporter", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await hapiCore.connect(wallets.publisher).activateReporter();

    expect(await hapiCore.getReporter(reporterAccount.id)).to.deep.equal([
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.name,
      reporterAccount.url,
      reporterAccount.role,
      ReporterStatus.Active,
      cfg.PUBLISHER_STAKE,
      0,
    ]);

    expect(await hapiCore.connect(wallets.publisher).deactivateReporter())
      .to.emit(hapiCore, "ReporterDeactivated")
      .withArgs(reporterAccount.id);

    expect(await hapiCore.getReporter(reporterAccount.id)).to.deep.equal([
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.name,
      reporterAccount.url,
      reporterAccount.role,
      ReporterStatus.Unstaking,
      cfg.PUBLISHER_STAKE,
      cfg.UNLOCK_DURATION + (await time.latest()),
    ]);
  });

  it("Should not be able to deactivate someone else's reporter account", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await hapiCore.connect(wallets.publisher).activateReporter();

    await token
      .connect(wallets.nobody)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await expect(
      hapiCore.connect(wallets.nobody).deactivateReporter()
    ).to.be.revertedWith('Caller is not a reporter');
  });

  it("Should be able to unstake tokens after unlock duration", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    const balanceBefore = await token.balanceOf(wallets.publisher.address);

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await hapiCore.connect(wallets.publisher).activateReporter();

    await hapiCore.connect(wallets.publisher).deactivateReporter();

    await time.increase(cfg.UNLOCK_DURATION + 1);

    expect(await hapiCore.connect(wallets.publisher).unstake())
      .to.emit(hapiCore, "ReporterUnstaked")
      .withArgs(reporterAccount.id);

    expect(await hapiCore.getReporter(reporterAccount.id)).to.deep.equal([
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.name,
      reporterAccount.url,
      reporterAccount.role,
      ReporterStatus.Inactive,
      0,
      0,
    ]);

    const balanceAfter = await token.balanceOf(wallets.publisher.address);

    expect(balanceAfter).to.equal(balanceBefore);
  });

  it("Should not be able to unstake other's reporter account", async function () {
    const { hapiCore, wallets, token, cfg } = await loadFixture(
      fixtureWithToken
    );

    const reporterAccount = {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporterAccount.id,
      reporterAccount.account,
      reporterAccount.role,
      reporterAccount.name,
      reporterAccount.url
    );

    const balanceBefore = await token.balanceOf(wallets.publisher.address);

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await hapiCore.connect(wallets.publisher).activateReporter();

    await hapiCore.connect(wallets.publisher).deactivateReporter();

    await time.increase(cfg.UNLOCK_DURATION + 1);

    await expect(hapiCore.connect(wallets.nobody).unstake()).to.be.revertedWith(
      "Caller is not a reporter"
    );

    const balanceAfter = await token.balanceOf(wallets.publisher.address);

    expect(balanceBefore.sub(balanceAfter)).to.equal(cfg.PUBLISHER_STAKE);
  });
});
