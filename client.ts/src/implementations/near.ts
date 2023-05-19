import {
  Addr,
  Address,
  Asset,
  Case,
  CaseStatus,
  Category,
  HapiCore,
  HapiCoreNetwork,
  Reporter,
  ReporterRole,
  Result,
  RewardConfiguration,
  StakeConfiguration,
} from "../interface";

export interface NearConnectionOptions {
  network: HapiCoreNetwork.NEAR;
  address: Addr;
  provider: unknown;
  signer?: unknown;
}

export class HapiCoreNear implements HapiCore {
  private contract: any;

  constructor(options: NearConnectionOptions) {
    this.contract = options.address;
  }

  async setAuthority(address: string): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getAuthority(): Promise<string> {
    throw new Error("Method not implemented.");
  }

  async updateStakeConfiguration(
    token: string,
    unlockDuration: number,
    validatorStake: string,
    tracerStake: string,
    publisherStake: string,
    authorityStake: string
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getStakeConfiguration(): Promise<StakeConfiguration> {
    throw new Error("Method not implemented.");
  }

  async updateRewardConfiguration(
    token: string,
    addressConfirmationReward: string,
    traceReward: string
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getRewardConfiguration(): Promise<RewardConfiguration> {
    throw new Error("Method not implemented.");
  }

  async createReporter(
    id: string,
    role: ReporterRole,
    account: string,
    name: string,
    url: string
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getReporter(id: string): Promise<Reporter> {
    throw new Error("Method not implemented.");
  }

  async updateReporter(
    id: string,
    role: ReporterRole,
    account: string,
    name: string,
    url: string
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async activateReporter(): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async deactivateReporter(): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async unstakeReporter(): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async createCase(id: string, name: string, url: string): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getCase(id: string): Promise<Case> {
    throw new Error("Method not implemented.");
  }

  async updateCase(
    id: string,
    name: string,
    url: string,
    status: CaseStatus
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async createAddress(
    address: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getAddress(address: string): Promise<Address> {
    throw new Error("Method not implemented.");
  }

  async updateAddress(
    address: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async createAsset(
    address: string,
    assetId: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getAsset(address: string, assetId: string): Promise<Asset> {
    throw new Error("Method not implemented.");
  }

  async updateAsset(
    address: string,
    assetId: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    throw new Error("Method not implemented.");
  }
}
