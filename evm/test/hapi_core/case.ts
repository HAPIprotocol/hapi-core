import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";

import { fixtureWithReporters } from "../setup";
import { CaseStatus, randomId } from "../util";

describe("HapiCore: Case", function () {
  it("Reporter should be able to create a new case", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    const case2 = {
      id: randomId(),
      name: "small scam 2022",
      url: "https://small.scam",
    };

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url)
    )
      .to.emit(hapiCore, "CaseCreated")
      .withArgs(case1.id);

    expect(await hapiCore.getCase(case1.id)).to.deep.equal([
      case1.id,
      case1.name,
      reporters.publisher.id,
      CaseStatus.Open,
      case1.url,
    ]);

    await expect(
      await hapiCore
        .connect(wallets.authority)
        .createCase(case2.id, case2.name, case2.url)
    )
      .to.emit(hapiCore, "CaseCreated")
      .withArgs(case2.id);

    expect(await hapiCore.getCaseCount()).to.equal(2);

    expect(await hapiCore.getCases(0, 10)).to.deep.equal([
      [
        case1.id,
        case1.name,
        reporters.publisher.id,
        CaseStatus.Open,
        case1.url,
      ],
      [
        case2.id,
        case2.name,
        reporters.authority.id,
        CaseStatus.Open,
        case2.url,
      ],
    ]);
  });

  it("Should not allow to create case if the reporter is not publisher or authority", async function () {
    const { hapiCore, wallets } = await loadFixture(fixtureWithReporters);

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    await expect(
      hapiCore
        .connect(wallets.validator)
        .createCase(case1.id, case1.name, case1.url)
    )
      .to.be.revertedWithCustomError(hapiCore, "InvalidReporter")
      .withArgs(wallets.validator.address);

    await expect(
      hapiCore
        .connect(wallets.tracer)
        .createCase(case1.id, case1.name, case1.url)
    )
      .to.be.revertedWithCustomError(hapiCore, "InvalidReporter")
      .withArgs(wallets.tracer.address);

    await expect(
      hapiCore
        .connect(wallets.nobody)
        .createCase(case1.id, case1.name, case1.url)
    )
      .to.be.revertedWithCustomError(hapiCore, "InvalidReporter")
      .withArgs(wallets.nobody.address);
  });

  it("Reporter should be able to update own case", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    await hapiCore
      .connect(wallets.publisher)
      .createCase(case1.id, case1.name, case1.url);

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .updateCase(
          case1.id,
          "_big hack 2023",
          "https://big.hack/2",
          CaseStatus.Closed
        )
    )
      .to.emit(hapiCore, "CaseUpdated")
      .withArgs(case1.id);

    expect(await hapiCore.getCase(case1.id)).to.deep.equal([
      case1.id,
      "_big hack 2023",
      reporters.publisher.id,
      CaseStatus.Closed,
      "https://big.hack/2",
    ]);
  });

  it("Should not be able to update other's case", async function () {
    const { hapiCore, wallets } = await loadFixture(fixtureWithReporters);

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    await hapiCore
      .connect(wallets.authority)
      .createCase(case1.id, case1.name, case1.url);

    await expect(
      hapiCore
        .connect(wallets.publisher)
        .updateCase(
          case1.id,
          "_big hack 2023",
          "https://big.hack/2",
          CaseStatus.Closed
        )
    )
      .to.be.revertedWithCustomError(hapiCore, "MustBeCaseReporterOrAuthority")
      .withArgs();
  });

  it("Should be able to update other's case if authority", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    await hapiCore
      .connect(wallets.publisher)
      .createCase(case1.id, case1.name, case1.url);

    await expect(
      await hapiCore
        .connect(wallets.authority)
        .updateCase(
          case1.id,
          "_big hack 2023",
          "https://big.hack/2",
          CaseStatus.Closed
        )
    )
      .to.emit(hapiCore, "CaseUpdated")
      .withArgs(case1.id);

    expect(await hapiCore.getCase(case1.id)).to.deep.equal([
      case1.id,
      "_big hack 2023",
      reporters.publisher.id,
      CaseStatus.Closed,
      "https://big.hack/2",
    ]);
  });

  it("Should panic if case not found", async function () {
    const { hapiCore } = await loadFixture(fixtureWithReporters);

    const id = randomId();

    await expect(hapiCore.getCase(id))
      .to.be.revertedWithCustomError(hapiCore, "CaseNotFound")
      .withArgs(id);
  });

  it("Should not allow to create a case with a duplicate ID", async function () {
    const { hapiCore, wallets } = await loadFixture(fixtureWithReporters);

    const id = randomId();

    const case1 = {
      id,
      name: "big hack 2024",
      url: "https://big.hack",
    };

    await hapiCore
      .connect(wallets.publisher)
      .createCase(case1.id, case1.name, case1.url);

    await expect(
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url)
    )
      .to.be.revertedWithCustomError(hapiCore, "DuplicateId")
      .withArgs(id);
  });
});
