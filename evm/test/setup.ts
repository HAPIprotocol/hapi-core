import { ethers, upgrades } from "hardhat";

import { HapiCore } from "../typechain-types";
import { IERC20 } from "../typechain-types";
import { ReporterRole, randomId } from "./util";
import { BaseContract, ContractFactory } from "ethers";

export async function setupContract(): Promise<{ hapiCore: HapiCore, contractAddress: string }> {
  const HapiCoreFactory = await ethers.getContractFactory("HapiCore");

  const contract = await upgrades.deployProxy(HapiCoreFactory as any, [], {
    initializer: "initialize",
  });

  await contract.waitForDeployment();

  const contractAddress = contract.target.toString();

  return { hapiCore: contract as unknown as BaseContract & HapiCore, contractAddress };
}

export async function basicFixture() {
  let setup = await setupContract();

  const [owner, authority, nobody] = await ethers.getSigners();

  return { ...setup, owner, authority, nobody };
}

export async function fixtureWithToken() {
  let setup = await setupContract();

  const [owner, authority, publisher, validator, tracer, nobody] =
    await ethers.getSigners();

  const wallets = { owner, authority, publisher, validator, tracer, nobody };

  const cfg = {
    UNLOCK_DURATION: 3600,
    VALIDATOR_STAKE: 101,
    TRACER_STAKE: 102,
    PUBLISHER_STAKE: 103,
    AUTHORITY_STAKE: 104,
  };

  const token = (await ethers.deployContract("Token")) as IERC20;
  await token.waitForDeployment();

  await Promise.all([
    token.transfer(authority.address, cfg.AUTHORITY_STAKE * 2),
    token.transfer(publisher.address, cfg.PUBLISHER_STAKE * 2),
    token.transfer(validator.address, cfg.VALIDATOR_STAKE * 2),
    token.transfer(tracer.address, cfg.TRACER_STAKE * 2),
    token.transfer(nobody.address, 10000),
    setup.hapiCore.updateStakeConfiguration(
      token.target.toString(),
      cfg.UNLOCK_DURATION,
      cfg.VALIDATOR_STAKE,
      cfg.TRACER_STAKE,
      cfg.PUBLISHER_STAKE,
      cfg.AUTHORITY_STAKE
    ),
  ]);

  return {
    ...setup,
    token,
    wallets,
    cfg,
  };
}

export async function fixtureWithReporters() {
  let setup = await fixtureWithToken();

  let { wallets, hapiCore, token, cfg } = setup;

  const reporters = {
    authority: {
      account: wallets.authority.address,
      id: randomId(),
      role: ReporterRole.Authority,
      name: "authority",
      url: "https://authority.blockchain",
    },
    publisher: {
      account: wallets.publisher.address,
      id: randomId(),
      role: ReporterRole.Publisher,
      name: "publisher",
      url: "https://publisher.blockchain",
    },
    validator: {
      account: wallets.validator.address,
      id: randomId(),
      role: ReporterRole.Validator,
      name: "validator",
      url: "https://validator.blockchain",
    },
    tracer: {
      account: wallets.tracer.address,
      id: randomId(),
      role: ReporterRole.Tracer,
      name: "tracer",
      url: "https://tracer.blockchain",
    },
  };

  let contractAddress = await hapiCore.waitForDeployment();

  await Promise.all([
    hapiCore.createReporter(
      reporters.authority.id,
      reporters.authority.account,
      reporters.authority.role,
      reporters.authority.name,
      reporters.authority.url
    ),
    hapiCore.createReporter(
      reporters.publisher.id,
      reporters.publisher.account,
      reporters.publisher.role,
      reporters.publisher.name,
      reporters.publisher.url
    ),
    hapiCore.createReporter(
      reporters.validator.id,
      reporters.validator.account,
      reporters.validator.role,
      reporters.validator.name,
      reporters.validator.url
    ),
    hapiCore.createReporter(
      reporters.tracer.id,
      reporters.tracer.account,
      reporters.tracer.role,
      reporters.tracer.name,
      reporters.tracer.url
    ),
    token
      .connect(wallets.authority)
      .approve(contractAddress, cfg.AUTHORITY_STAKE),
    token
      .connect(wallets.publisher)
      .approve(contractAddress, cfg.PUBLISHER_STAKE),
    token
      .connect(wallets.validator)
      .approve(contractAddress, cfg.VALIDATOR_STAKE),
      token
      .connect(wallets.tracer)
      .approve(contractAddress, cfg.TRACER_STAKE),
  ]);

  await Promise.all([
    setup.hapiCore.connect(wallets.authority).activateReporter(),
    setup.hapiCore.connect(wallets.publisher).activateReporter(),
    setup.hapiCore.connect(wallets.validator).activateReporter(),
    setup.hapiCore.connect(wallets.tracer).activateReporter(),
  ]);

  return { ...setup, reporters };
}
