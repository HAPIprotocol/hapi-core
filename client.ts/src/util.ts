import * as uuid from "uuid";
import {
  CaseStatus,
  Category,
  ReporterRole,
  ReporterStatus,
} from "./interface";
import { BigNumber } from "@ethersproject/bignumber";

export function uuidToBigInt(input: string): BigInt {
  uuid.parse(input);
  return BigInt("0x" + input.replace(/-/g, ""));
}

export function uuidToBigNumberish(input: string): string {
  return uuidToBigInt(input).toString();
}

export function bigIntToUuid(bigInt: BigInt | string | any) {
  // Convert input to BigInt if necessary
  if (typeof bigInt === "string") {
    bigInt = BigInt(bigInt);
  } else if (typeof bigInt === "object") {
    bigInt = BigInt(bigInt.toString());
  }

  // Convert BigInt to a hex string, padded to 32 characters
  const hex = bigInt.toString(16).padStart(32, "0");

  // Insert hyphens at the correct places to form a UUID
  const uuid = [
    hex.slice(0, 8),
    hex.slice(8, 12),
    hex.slice(12, 16),
    hex.slice(16, 20),
    hex.slice(20, 32),
  ].join("-");

  return uuid;
}

export function validateUuid(input: string): string {
  uuid.parse(input);
  return input;
}

export function validateRiskScore(value: number): number {
  if (value < 0 || value > 10) {
    throw new Error(`Invalid risk value: ${value}`);
  }
  return value;
}

export function CaseStatusFromString(value: string): CaseStatus {
  switch (value) {
    case "Closed":
      return CaseStatus.Closed;
    case "Open":
      return CaseStatus.Open;
    default:
      throw new Error(`Unsupported case status: ${value}`);
  }
}

export function CaseStatusToString(value: CaseStatus): string {
  switch (value) {
    case CaseStatus.Closed:
      return "Closed";
    case CaseStatus.Open:
      return "Open";
    default:
      throw new Error(`Unsupported case status: ${value}`);
  }
}

export function CategoryFromString(value: string): Category {
  switch (value) {
    case "None":
      return Category.None;
    case "WalletService":
      return Category.WalletService;
    case "MerchantService":
      return Category.MerchantService;
    case "MiningPool":
      return Category.MiningPool;
    case "Exchange":
      return Category.Exchange;
    case "DeFi":
      return Category.DeFi;
    case "OTCBroker":
      return Category.OTCBroker;
    case "ATM":
      return Category.ATM;
    case "Gambling":
      return Category.Gambling;
    case "IllicitOrganization":
      return Category.IllicitOrganization;
    case "Mixer":
      return Category.Mixer;
    case "DarknetService":
      return Category.DarknetService;
    case "Scam":
      return Category.Scam;
    case "Ransomware":
      return Category.Ransomware;
    case "Theft":
      return Category.Theft;
    case "Counterfeit":
      return Category.Counterfeit;
    case "TerroristFinancing":
      return Category.TerroristFinancing;
    case "Sanctions":
      return Category.Sanctions;
    case "ChildAbuse":
      return Category.ChildAbuse;
    case "Hacker":
      return Category.Hacker;
    case "HighRiskJurisdiction":
      return Category.HighRiskJurisdiction;
    default:
      throw new Error(`Unsupported category: ${value}`);
  }
}

export function CategoryToString(value: Category): string {
  switch (value) {
    case Category.None:
      return "None";
    case Category.WalletService:
      return "WalletService";
    case Category.MerchantService:
      return "MerchantService";
    case Category.MiningPool:
      return "MiningPool";
    case Category.Exchange:
      return "Exchange";
    case Category.DeFi:
      return "DeFi";
    case Category.OTCBroker:
      return "OTCBroker";
    case Category.ATM:
      return "ATM";
    case Category.Gambling:
      return "Gambling";
    case Category.IllicitOrganization:
      return "IllicitOrganization";
    case Category.Mixer:
      return "Mixer";
    case Category.DarknetService:
      return "DarknetService";
    case Category.Scam:
      return "Scam";
    case Category.Ransomware:
      return "Ransomware";
    case Category.Theft:
      return "Theft";
    case Category.Counterfeit:
      return "Counterfeit";
    case Category.TerroristFinancing:
      return "TerroristFinancing";
    case Category.Sanctions:
      return "Sanctions";
    case Category.ChildAbuse:
      return "ChildAbuse";
    case Category.Hacker:
      return "Hacker";
    case Category.HighRiskJurisdiction:
      return "HighRiskJurisdiction";
    default:
      throw new Error(`Unsupported category: ${value}`);
  }
}

export function ReporterRoleFromString(value: string): ReporterRole {
  switch (value) {
    case "Authority":
      return ReporterRole.Authority;
    case "Publisher":
      return ReporterRole.Publisher;
    case "Validator":
      return ReporterRole.Validator;
    case "Tracer":
      return ReporterRole.Tracer;
    default:
      throw new Error(`Unsupported reporter role: ${value}`);
  }
}

export function ReporterRoleToString(value: ReporterRole): string {
  switch (value) {
    case ReporterRole.Authority:
      return "Authority";
    case ReporterRole.Publisher:
      return "Publisher";
    case ReporterRole.Validator:
      return "Validator";
    case ReporterRole.Tracer:
      return "Tracer";
    default:
      throw new Error(`Unsupported reporter role: ${value}`);
  }
}

export function ReporterStatusFromString(value: string): ReporterStatus {
  switch (value) {
    case "Inactive":
      return ReporterStatus.Inactive;
    case "Active":
      return ReporterStatus.Active;
    case "Unstaking":
      return ReporterStatus.Unstaking;
    default:
      throw new Error(`Unsupported reporter status: ${value}`);
  }
}

export function ReporterStatusToString(value: ReporterStatus): string {
  switch (value) {
    case ReporterStatus.Inactive:
      return "Inactive";
    case ReporterStatus.Active:
      return "Active";
    case ReporterStatus.Unstaking:
      return "Unstaking";
    default:
      throw new Error(`Unsupported reporter status: ${value}`);
  }
}
