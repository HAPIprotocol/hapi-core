import exp from "constants";

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

  /// Update an existing asset
  updateAsset(
    address: Addr,
    assetId: string,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result>;
}
