import { IDL } from "../target/types/hapi_core_solana";
import { BN } from "@coral-xyz/anchor";

// HapiCore Instructions
export const HapiCoreInstructionVariants = IDL.instructions.map(
  (ix) => ix.name
);

export type HapiCoreInstruction = (typeof HapiCoreInstructionVariants)[number];

// HapiCore Accounts
export const HapiCoreAccountVariants = IDL.accounts.map((acc) => acc.name);

export type HapiCoreAccount = (typeof HapiCoreAccountVariants)[number];

export type stakeConfiguration = {
  unlockDuration: BN;
  validatorStake: BN;
  tracerStake: BN;
  publisherStake: BN;
  authorityStake: BN;
  appraiserStake: BN;
};

export type rewardConfiguration = {
  addressTracerReward: BN;
  addressConfirmationReward: BN;
  assetTracerReward: BN;
  assetConfirmationReward: BN;
};

// ReporterRole
export const ReporterRole = {
  Validator: { validator: {} },
  Tracer: { tracer: {} },
  Publisher: { publisher: {} },
  Authority: { authority: {} },
  Appraiser: { appraiser: {} },
} as const;

export type ReporterRoleKeys = keyof typeof ReporterRole;

export const ReporterRoleVariants = Object.keys(ReporterRole) as Readonly<
  ReporterRoleKeys[]
>;

export function getReporterRoleIndex(role: typeof ReporterRole) {
  const key = Object.keys(role)[0];

  switch (key) {
    case "validator":
      return 0;
    case "tracer":
      return 1;
    case "publisher":
      return 2;
    case "authority":
      return 3;
    case "appraiser":
      return 4;
    default:
      throw new Error(`Unsupported reporter role: ${role}`);
  }
}

// ReporterStatus
export const ReporterStatus = {
  Inactive: { inactive: {} },
  Active: { active: {} },
  Unstaking: { unstaking: {} },
} as const;

export type ReporterStatusKeys = keyof typeof ReporterStatus;

export const ReporterStatusVariants = Object.keys(ReporterStatus) as Readonly<
  ReporterStatusKeys[]
>;

export function getReporterStatusIndex(status: typeof ReporterStatus) {
  const key = Object.keys(status)[0];

  switch (key) {
    case "inactive":
      return 0;
    case "active":
      return 1;
    case "unstaking":
      return 2;
    default:
      throw new Error(`Unsupported reporter status: ${status}`);
  }
}

// CaseStatus
export const CaseStatus = {
  Closed: { closed: {} },
  Open: { open: {} },
} as const;

export type CaseStatusKeys = keyof typeof CaseStatus;

export const CaseStatusVariants = Object.keys(CaseStatus) as Readonly<
  CaseStatusKeys[]
>;

export function getCaseStatusIndex(status: typeof CaseStatus) {
  const key = Object.keys(status)[0];

  switch (key) {
    case "closed":
      return 0;
    case "open":
      return 1;
    default:
      throw new Error(`Unsupported case status: ${status}`);
  }
}

// Category
export const Category = {
  None: { none: {} },
  WalletService: { walletService: {} },
  MerchantService: { merchantService: {} },
  MiningPool: { miningPool: {} },
  Exchange: { exchange: {} },
  DeFi: { deFi: {} },
  OTCBroker: { oTCBroker: {} },
  ATM: { aTM: {} },
  Gambling: { gambling: {} },
  IllicitOrganization: { illicitOrganization: {} },
  Mixer: { mixer: {} },
  DarknetService: { darknetService: {} },
  Scam: { scam: {} },
  Ransomware: { ransomware: {} },
  Theft: { theft: {} },
  Counterfeit: { counterfeit: {} },
  TerroristFinancing: { terroristFinancing: {} },
  Sanctions: { sanctions: {} },
  ChildAbuse: { childAbuse: {} },
  Hacker: { hacker: {} },
  HighRiskJurisdiction: { highRiskJurisdiction: {} },
};

export type CategoryKeys = keyof typeof Category;

export const CategoryVariants = Object.keys(Category) as Readonly<
  CategoryKeys[]
>;

export function getCategoryIndex(category: typeof Category) {
  const key = Object.keys(category)[0];

  switch (key) {
    case "none":
      return 0;
    case "walletService":
      return 1;
    case "merchantService":
      return 2;
    case "miningPool":
      return 3;
    case "exchange":
      return 4;
    case "deFi":
      return 5;
    case "oTCBroker":
      return 6;
    case "aTM":
      return 7;
    case "gambling":
      return 8;
    case "illicitOrganization":
      return 9;
    case "mixer":
      return 10;
    case "darknetService":
      return 11;
    case "scam":
      return 12;
    case "ransomware":
      return 13;
    case "theft":
      return 14;
    case "counterfeit":
      return 15;
    case "terroristFinancing":
      return 16;
    case "sanctions":
      return 17;
    case "childAbuse":
      return 18;
    case "hacker":
      return 19;
    case "highRiskJurisdiction":
      return 20;
    default:
      throw new Error(`Unsupported category: ${category}`);
  }
}

export const ACCOUNT_SIZE: Readonly<Record<HapiCoreAccount, number>> = {
  network: 251,
  reporter: 397,
  case: 380,
  address: 174,
  asset: 238,
  confirmation: 123,
};
