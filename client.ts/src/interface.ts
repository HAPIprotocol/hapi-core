import {
  EvmConnectionOptions,
  NearConnectionOptions,
  SolanaConnectionOptions,
} from "./implementations";

export enum HapiCoreNetwork {
  Ethereum = "ethereum",
  BSC = "bsc",
  Solana = "solana",
  Bitcoin = "bitcoin",
  NEAR = "near",
}

export const HapiCoreNetworks = [
  HapiCoreNetwork.Ethereum,
  HapiCoreNetwork.BSC,
  HapiCoreNetwork.Solana,
  HapiCoreNetwork.Bitcoin,
  HapiCoreNetwork.NEAR,
];

export const HapiCoreAddresses: {
  [key in HapiCoreNetwork]: string;
} = {
  [HapiCoreNetwork.Ethereum]: "0x0DCd1Bf9A1b36cE34237eEaFef220932846BCD82",
  [HapiCoreNetwork.BSC]: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
  [HapiCoreNetwork.Solana]: "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6",
  [HapiCoreNetwork.Bitcoin]: "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6",
  [HapiCoreNetwork.NEAR]: "core.hapiprotocol.near",
};

export type HapiCoreConnectionOptions =
  | EvmConnectionOptions
  | SolanaConnectionOptions
  | NearConnectionOptions;

export enum ReporterRole {
  Validator = 0,
  Tracer = 1,
  Publisher = 2,
  Authority = 3,
}

export const ReporterRoleNames = [
  "Validator",
  "Tracer",
  "Publisher",
  "Authority",
];

export enum ReporterStatus {
  Inactive = 0,
  Active = 1,
  Unstaking = 2,
}

export const ReporterStatusNames = ["Inactive", "Active", "Unstaking"];

export enum CaseStatus {
  Closed = 0,
  Open = 1,
}

export const CaseStatusNames = ["Closed", "Open"];

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

export const CategoryNames = [
  "None",
  "WalletService",
  "MerchantService",
  "MiningPool",
  "Exchange",
  "DeFi",
  "OTCBroker",
  "ATM",
  "Gambling",
  "IllicitOrganization",
  "Mixer",
  "DarknetService",
  "Scam",
  "Ransomware",
  "Theft",
  "Counterfeit",
  "TerroristFinancing",
  "Sanctions",
  "ChildAbuse",
  "Hacker",
  "HighRiskJurisdiction",
];

export interface Result {
  transactionHash: string;
}

export type Addr = string;
export type Amount = string;
export type Uuid = string;

export interface StakeConfiguration {
  token: Addr;
  unlockDuration: number;
  validatorStake: Amount;
  tracerStake: Amount;
  publisherStake: Amount;
  authorityStake: Amount;
}

export interface RewardConfiguration {
  token: Addr;
  addressConfirmationReward: Amount;
  tracerReward: Amount;
}

export interface Reporter {
  id: Uuid;
  account: Addr;
  role: ReporterRole;
  status: ReporterStatus;
  name: string;
  url: string;
  stake: Amount;
  unlockTimestamp: number;
}

export interface Case {
  id: Uuid;
  name: string;
  url: string;
  status: CaseStatus;
}

export interface Address {
  address: Addr;
  caseId: Uuid;
  reporterId: Uuid;
  risk: number;
  category: Category;
}

export interface Asset {
  address: Addr;
  assetId: string;
  caseId: Uuid;
  reporterId: Uuid;
  risk: number;
  category: Category;
}

export interface HapiCore {
  /// Sets the authority address
  setAuthority(address: Addr): Promise<Result>;

  /// Returns the authority address
  getAuthority(): Promise<Addr>;

  /// Update stake configuration
  updateStakeConfiguration(
    token: Addr,
    unlockDuration: number,
    validatorStake: Amount,
    tracerStake: Amount,
    publisherStake: Amount,
    authorityStake: Amount
  ): Promise<Result>;

  /// Returns the stake configuration
  getStakeConfiguration(): Promise<StakeConfiguration>;

  /// Update reward configuration
  updateRewardConfiguration(
    token: Addr,
    addressConfirmationReward: Amount,
    traceReward: Amount
  ): Promise<Result>;

  /// Returns the reward configuration
  getRewardConfiguration(): Promise<RewardConfiguration>;

  /// Create a new reporter
  createReporter(
    id: Uuid,
    role: ReporterRole,
    account: Addr,
    name: string,
    url: string
  ): Promise<Result>;

  /// Returns the reporter
  getReporter(id: Uuid): Promise<Reporter>;

  /// Returns the reporter count
  getReporterCount(): Promise<number>;

  /// Returns a paged reporter list
  getReporters(skip: number, take: number): Promise<Reporter[]>;

  /// Update an existing reporter
  updateReporter(
    id: Uuid,
    role: ReporterRole,
    account: Addr,
    name: string,
    url: string
  ): Promise<Result>;

  /// Stake tokens to activate the reporter
  activateReporter(): Promise<Result>;

  /// Deactivate the reporter
  deactivateReporter(): Promise<Result>;

  /// Unstake tokens from the reporter after the deactivation period
  unstakeReporter(): Promise<Result>;

  /// Create a new case
  createCase(id: string, name: string, url: string): Promise<Result>;

  /// Returns the case
  getCase(id: Uuid): Promise<Case>;

  /// Returns the case count
  getCaseCount(): Promise<number>;

  /// Returns a paged case list
  getCases(skip: number, take: number): Promise<Case[]>;

  /// Update an existing case
  updateCase(
    id: Uuid,
    name: string,
    url: string,
    status: CaseStatus
  ): Promise<Result>;

  /// Create a new address
  createAddress(
    address: Addr,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result>;

  /// Returns the address
  getAddress(address: Addr): Promise<Address>;

  /// Returns the address count
  getAddressCount(): Promise<number>;

  /// Returns a paged address list
  getAddresses(skip: number, take: number): Promise<Address[]>;

  /// Update an existing address
  updateAddress(
    address: Addr,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result>;

  /// Create a new asset
  createAsset(
    address: Addr,
    assetId: string,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result>;

  /// Returns the asset
  getAsset(address: Addr, assetId: string): Promise<Asset>;

  /// Returns the asset count
  getAssetCount(): Promise<number>;

  /// Returns a paged asset list
  getAssets(skip: number, take: number): Promise<Asset[]>;

  /// Update an existing asset
  updateAsset(
    address: Addr,
    assetId: string,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result>;
}
