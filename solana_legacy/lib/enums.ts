import { IDL } from "./idl/hapi_core";

// HapiCore Instructions
export const HapiCoreInstructionVariants = IDL.instructions.map(
  (ix) => ix.name
);

export type HapiCoreInstruction = typeof HapiCoreInstructionVariants[number];

// HapiCore Accounts
export const HapiCoreAccountVariants = IDL.accounts.map((acc) => acc.name);

export type HapiCoreAccount = typeof HapiCoreAccountVariants[number];

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

// CaseStatus
export const CaseStatus = {
  Closed: { closed: {} },
  Open: { open: {} },
} as const;

export type CaseStatusKeys = keyof typeof CaseStatus;

export const CaseStatusVariants = Object.keys(CaseStatus) as Readonly<
  CaseStatusKeys[]
>;

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
  HighRiskJurisdiction: { highRiskJurisdiction: {} },
  Hacker: { hacker: {} },
};

export type CategoryKeys = keyof typeof Category;

export const CategoryVariants = Object.keys(Category) as Readonly<
  CategoryKeys[]
>;

export const NetworkSchema = {
  Plain: { plain: {} },
  Solana: { solana: {} },
  Ethereum: { ethereum: {} },
  Bitcoin: { bitcoin: {} },
  Near: { near: {} },
};

export type NetworkSchemaKeys = keyof typeof NetworkSchema;

export const NetworkSchemaVariants = Object.keys(NetworkSchema) as Readonly<
  NetworkSchemaKeys[]
>;

export const ACCOUNT_SIZE: Readonly<Record<HapiCoreAccount, number>> = {
  address: 222,
  addressV0: 184,
  asset: 254,
  assetV0: 216,
  case: 148,
  caseV0: 120,
  community: 172,
  communityV0: 192,
  network: 180,
  networkV0: 176,
  reporter: 158,
  reporterV0: 128,
  reporterReward: 139,
  reporterRewardV0: 112,
};
