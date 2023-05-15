import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";

import { basicFixture } from "../setup";
import { ReporterRole, ReporterStatus, randomId } from "../util";

describe("HapiCore: Reporter management", function () {
  it("Should create a reporter", async function () {
    const { hapiCore } = await loadFixture(basicFixture);

    const reporter = {
      account: "0xdEADBEeF00000000000000000000000000000000",
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await expect(
      await hapiCore.createReporter(
        reporter.id,
        reporter.account,
        reporter.role,
        reporter.name,
        reporter.url
      )
    )
      .to.emit(hapiCore, "ReporterCreated")
      .withArgs(reporter.id, reporter.account, reporter.role);

    expect(await hapiCore.getReporter(reporter.id)).to.deep.equal([
      reporter.id,
      reporter.account,
      reporter.name,
      reporter.url,
      reporter.role,
      ReporterStatus.Inactive,
      0,
      0,
    ]);
  });

  it("Should not create a reporter if not authority", async function () {
    const { hapiCore, nobody } = await loadFixture(basicFixture);

    const reporter = {
      account: "0xdEADBEeF00000000000000000000000000000000",
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await expect(
      hapiCore
        .connect(nobody)
        .createReporter(
          reporter.id,
          reporter.account,
          reporter.role,
          reporter.name,
          reporter.url
        )
    ).to.be.revertedWith("Caller is not the authority");
  });

  it("Should update a reporter", async function () {
    const { hapiCore } = await loadFixture(basicFixture);

    const reporterOld = {
      account: "0xdEADBEeF00000000000000000000000000000000",
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    const reporterNew = {
      id: reporterOld.id,
      account: "0xb04b26349DE3f1B4Dc2e54ecCb54458c343C2909",
      role: ReporterRole.Authority,
      name: "authority",
      url: "https://authority.blockchain",
    };

    await hapiCore.createReporter(
      reporterOld.id,
      reporterOld.account,
      reporterOld.role,
      reporterOld.name,
      reporterOld.url
    );

    await expect(
      await hapiCore.updateReporter(
        reporterOld.id,
        reporterNew.account,
        reporterNew.role,
        reporterNew.name,
        reporterNew.url
      )
    )
      .to.emit(hapiCore, "ReporterUpdated")
      .withArgs(reporterOld.id, reporterNew.account, reporterNew.role);

    expect(await hapiCore.getReporter(reporterOld.id)).to.deep.equal([
      reporterOld.id,
      reporterNew.account,
      reporterNew.name,
      reporterNew.url,
      reporterNew.role,
      ReporterStatus.Inactive,
      0,
      0,
    ]);
  });

  it("Should not update a reporter if not authority", async function () {
    const { hapiCore, nobody } = await loadFixture(basicFixture);

    const reporter = {
      account: "0xdEADBEeF00000000000000000000000000000000",
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    await hapiCore.createReporter(
      reporter.id,
      reporter.account,
      reporter.role,
      reporter.name,
      reporter.url
    );

    await expect(
      hapiCore
        .connect(nobody)
        .updateReporter(
          reporter.id,
          reporter.account,
          reporter.role,
          reporter.name,
          reporter.url
        )
    ).to.be.revertedWith("Caller is not the authority");
  });

  it("Should list reporters", async function () {
    const { hapiCore, nobody, authority } = await loadFixture(basicFixture);

    const reporter1 = {
      account: nobody.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    const reporter2 = {
      account: authority.address,
      id: randomId(),
      role: ReporterRole.Authority,
      name: "authority",
      url: "https://authority.blockchain",
    };

    await hapiCore.createReporter(
      reporter1.id,
      reporter1.account,
      reporter1.role,
      reporter1.name,
      reporter1.url
    );
    await hapiCore.createReporter(
      reporter2.id,
      reporter2.account,
      reporter2.role,
      reporter2.name,
      reporter2.url
    );

    expect(await hapiCore.getReporters(1, 0)).to.deep.equal([
      [
        reporter1.id,
        reporter1.account,
        reporter1.name,
        reporter1.url,
        reporter1.role,
        ReporterStatus.Inactive,
        0,
        0,
      ],
    ]);

    expect(await hapiCore.getReporters(1, 1)).to.deep.equal([
      [
        reporter2.id,
        reporter2.account,
        reporter2.name,
        reporter2.url,
        reporter2.role,
        ReporterStatus.Inactive,
        0,
        0,
      ],
    ]);

    expect(await hapiCore.getReporters(2, 0)).to.deep.equal([
      [
        reporter1.id,
        reporter1.account,
        reporter1.name,
        reporter1.url,
        reporter1.role,
        ReporterStatus.Inactive,
        0,
        0,
      ],
      [
        reporter2.id,
        reporter2.account,
        reporter2.name,
        reporter2.url,
        reporter2.role,
        ReporterStatus.Inactive,
        0,
        0,
      ],
    ]);

    expect(await hapiCore.getReporters(100, 5)).to.deep.equal([]);
  });

  it("Should retrieve reporter count", async function () {
    const { hapiCore, nobody, authority } = await loadFixture(basicFixture);

    const reporter1 = {
      account: nobody.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    };

    const reporter2 = {
      account: authority.address,
      id: randomId(),
      role: ReporterRole.Authority,
      name: "authority",
      url: "https://authority.blockchain",
    };

    await hapiCore.createReporter(
      reporter1.id,
      reporter1.account,
      reporter1.role,
      reporter1.name,
      reporter1.url
    );
    await hapiCore.createReporter(
      reporter2.id,
      reporter2.account,
      reporter2.role,
      reporter2.name,
      reporter2.url
    );

    expect(await hapiCore.getReporterCount()).to.equal(2);
  });
});
