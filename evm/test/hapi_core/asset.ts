import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";

import { fixtureWithReporters } from "../setup";
import { Category, randomId } from "../util";

describe("HapiCore: Asset", function () {
  it("Should be able to create an asset", async function () {
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

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        )
    )
      .to.emit(hapiCore, "AssetCreated")
      .withArgs(asset.addr, asset.assetId, asset.risk, asset.category);

    expect(await hapiCore.getAsset(asset.addr, asset.assetId)).to.deep.equal([
      asset.addr,
      asset.assetId,
      asset.caseId,
      asset.reporterId,
      0,
      asset.risk,
      asset.category,
    ]);

    expect(await hapiCore.getAssetCount()).to.equal(1);

    expect(await hapiCore.getAssets(0, 10)).to.deep.equal([
      [
        asset.addr,
        asset.assetId,
        asset.caseId,
        asset.reporterId,
        0,
        asset.risk,
        asset.category,
      ],
    ]);
  });

  it("Should return empty asset if not found", async function () {
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

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    expect(await hapiCore.getAsset(asset.addr, asset.assetId)).to.deep.equal([
      "0x0000000000000000000000000000000000000000",
      BigInt(0),
      "0000000000000000000000000000000000000000000000000000000000000000",
      "0000000000000000000000000000000000000000000000000000000000000000",
      0,
      0,
      0,
    ]);
  });

  it("Should be able to update an asset", async function () {
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

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.publisher)
        .createCase(case2.id, case2.name, case2.url),
    ]);

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await hapiCore
      .connect(wallets.publisher)
      .createAsset(
        asset.addr,
        asset.assetId,
        asset.caseId,
        asset.risk,
        asset.category
      );

    const newRisk = 6;
    const newCategory = Category.Scam;

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .updateAsset(asset.addr, asset.assetId, newRisk, newCategory, case2.id)
    )
      .to.emit(hapiCore, "AssetUpdated")
      .withArgs(asset.addr, asset.assetId, newRisk, newCategory);

    expect(await hapiCore.getAsset(asset.addr, asset.assetId)).to.deep.equal([
      asset.addr,
      asset.assetId,
      case2.id,
      asset.reporterId,
      0,
      newRisk,
      newCategory,
    ]);
  });

  it("Tracer shouldn't be able to change asset case", async function () {
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
      name: "child abuse case #1488",
      url: "https://child.abuse",
    };
    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.publisher)
        .createCase(case2.id, case2.name, case2.url),
      hapiCore
        .connect(wallets.tracer)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        ),
    ]);

    await expect(
      hapiCore
        .connect(wallets.tracer)
        .updateAsset(
          asset.addr,
          asset.assetId,
          10,
          Category.ChildAbuse,
          case2.id
        )
    )
      .to.be.revertedWithCustomError(hapiCore, "InvalidReporter")
      .withArgs(wallets.tracer.address);
  });

  it("Should be able to confirm an asset", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.tracer.id,
      risk: 5,
      category: Category.Hacker,
    };

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.tracer)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        ),
    ]);

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .confirmAsset(asset.addr, asset.assetId)
    )
      .to.emit(hapiCore, "AssetConfirmed")
      .withArgs(asset.addr, asset.assetId);

    expect(await hapiCore.getAsset(asset.addr, asset.assetId)).to.deep.equal([
      asset.addr,
      asset.assetId,
      case1.id,
      asset.reporterId,
      1,
      asset.risk,
      asset.category,
    ]);
  });

  it("Should be able to confirm an asset only once", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.tracer)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        ),
    ]);

    await expect(
      await hapiCore
        .connect(wallets.publisher)
        .confirmAsset(asset.addr, asset.assetId)
    )
      .to.emit(hapiCore, "AssetConfirmed")
      .withArgs(asset.addr, asset.assetId);

    await expect(
      hapiCore
        .connect(wallets.publisher)
        .confirmAsset(asset.addr, asset.assetId)
    )
      .to.be.revertedWithCustomError(hapiCore, "AssetAlreadyConfirmed")
      .withArgs(asset.addr, asset.assetId, reporters.publisher.id);
  });

  it("Only publisher or validator should be able to confirm an asset", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.tracer)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        ),
    ]);

    await expect(
      hapiCore.connect(wallets.tracer).confirmAsset(asset.addr, asset.assetId)
    )
      .to.be.revertedWithCustomError(hapiCore, "InvalidReporter")
      .withArgs(wallets.tracer.address);
  });

  it("Cannot confirm the asset reported by himself", async function () {
    const { hapiCore, wallets, reporters } = await loadFixture(
      fixtureWithReporters
    );

    const case1 = {
      id: randomId(),
      name: "big hack 2023",
      url: "https://big.hack",
    };

    const asset = {
      addr: "0xeEE91Aa5d1AcBBe0DA7a1009BeC3fdD91e711832",
      assetId: BigInt(1),
      caseId: case1.id,
      reporterId: reporters.publisher.id,
      risk: 5,
      category: Category.Hacker,
    };

    await Promise.all([
      hapiCore
        .connect(wallets.publisher)
        .createCase(case1.id, case1.name, case1.url),
      hapiCore
        .connect(wallets.publisher)
        .createAsset(
          asset.addr,
          asset.assetId,
          asset.caseId,
          asset.risk,
          asset.category
        ),
    ]);

    await expect(
      hapiCore
        .connect(wallets.publisher)
        .confirmAsset(asset.addr, asset.assetId)
    )
      .to.be.revertedWithCustomError(hapiCore, "CannotConfirmOwnAsset")
      .withArgs(asset.addr, asset.assetId, reporters.publisher.id);
  });
});
