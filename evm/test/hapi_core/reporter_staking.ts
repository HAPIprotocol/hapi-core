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
      hapiCore.connect(wallets.publisher).activateReporter(reporterAccount.id)
    ).to.be.revertedWith("ERC20: insufficient allowance");

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    expect(
      await hapiCore
        .connect(wallets.publisher)
        .activateReporter(reporterAccount.id)
    )
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
      hapiCore.connect(wallets.publisher).activateReporter(reporterAccount.id)
    ).to.be.revertedWith("ERC20: insufficient allowance");

    await token
      .connect(wallets.publisher)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await expect(
      hapiCore.connect(wallets.publisher).activateReporter(reporterAccount.id)
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
      hapiCore.connect(wallets.nobody).activateReporter(reporterAccount.id)
    ).to.be.revertedWith("Caller is not the target reporter");
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

    await hapiCore
      .connect(wallets.publisher)
      .activateReporter(reporterAccount.id);

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

    expect(
      await hapiCore
        .connect(wallets.publisher)
        .deactivateReporter(reporterAccount.id)
    )
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

    await hapiCore
      .connect(wallets.publisher)
      .activateReporter(reporterAccount.id);

    await token
      .connect(wallets.nobody)
      .approve(hapiCore.address, cfg.PUBLISHER_STAKE);

    await expect(
      hapiCore.connect(wallets.nobody).deactivateReporter(reporterAccount.id)
    ).to.be.revertedWith("Caller is not the target reporter");
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

    await hapiCore
      .connect(wallets.publisher)
      .activateReporter(reporterAccount.id);

    await hapiCore
      .connect(wallets.publisher)
      .deactivateReporter(reporterAccount.id);

    await time.increase(cfg.UNLOCK_DURATION + 1);

    expect(
      await hapiCore.connect(wallets.publisher).unstake(reporterAccount.id)
    )
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

    await hapiCore
      .connect(wallets.publisher)
      .activateReporter(reporterAccount.id);

    await hapiCore
      .connect(wallets.publisher)
      .deactivateReporter(reporterAccount.id);

    await time.increase(cfg.UNLOCK_DURATION + 1);

    await expect(
      hapiCore.connect(wallets.nobody).unstake(reporterAccount.id)
    ).to.be.revertedWith("Caller is not the target reporter");

    const balanceAfter = await token.balanceOf(wallets.publisher.address);

    expect(balanceBefore.sub(balanceAfter)).to.equal(cfg.PUBLISHER_STAKE);
  });
});
