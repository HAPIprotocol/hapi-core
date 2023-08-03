import { randomBytes } from "ethers";

function uint8ArrayToBigInt(array: Uint8Array) {
  let hex = [...array]
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
  return BigInt('0x' + hex);
}

export function randomId() {
  return uint8ArrayToBigInt(randomBytes(16));
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

export enum Category {
  None = 0,
  WalletService = 1,
  MerchantService = 2,
  MiningPool = 3,
  Exchange = 4,
  DeFi = 5,
  OTCBroker = 6,
  ATM = 7,
  Gambling = 8,
  IllicitOrganization = 9,
  Mixer = 10,
  DarknetService = 11,
  Scam = 12,
  Ransomware = 13,
  Theft = 14,
  Counterfeit = 15,
  TerroristFinancing = 16,
  Sanctions = 17,
  ChildAbuse = 18,
  Hacker = 19,
  HighRiskJurisdiction = 20,
}
