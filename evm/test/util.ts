import { BigNumber } from "ethers";
import { ethers } from "hardhat";

export function randomId() {
  return BigNumber.from(ethers.utils.randomBytes(16));
}

export enum ReporterRole {
  Validator = 0,
  Tracer = 1,
  Publisher = 2,
  Authority = 3,
}

export enum ReporterStatus {
  Inactive = 0,
  Active = 1,
  Unstaking = 2,
}

export enum CaseStatus {
  Closed = 0,
  Open = 1,
}
