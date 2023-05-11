import { ethers, upgrades } from "hardhat";

import { HapiCore } from "../typechain-types";

export async function setupContract(): Promise<{ hapiCore: HapiCore }> {
  const HapiCore = await ethers.getContractFactory("HapiCore");

  const contract = await upgrades.deployProxy(HapiCore, [], {
    initializer: "initialize",
  });

  await contract.deployed();

  return { hapiCore: contract as HapiCore };
}
