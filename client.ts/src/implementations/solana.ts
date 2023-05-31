import { Provider, web3 } from "@coral-xyz/anchor";
import { HapiCoreProgram } from "../../../solana/lib";

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
  HapiCoreAddresses,
  ReporterRoleNames
} from "../interface";

export interface SolanaConnectionOptions {
  network: HapiCoreNetwork.Solana | HapiCoreNetwork.Bitcoin;
  address?: Addr;
  provider: Provider;
  signer?: web3.Keypair;
}

export class HapiCoreSolana implements HapiCore {
  private contract: HapiCoreProgram;
  private network: string;
  private signer?: web3.Keypair;

  constructor (options: SolanaConnectionOptions) {
    this.contract = new HapiCoreProgram(
      options.address || HapiCoreAddresses[options.network],
      options.provider);

    if (options.signer) {
      this.signer = options.signer;
    }

    this.network = options.network;
  }

  async setAuthority(address: string): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.setAuthority(this.network, address);
    return { transactionHash };
  }

  async getAuthority(): Promise<string> {
    throw new Error("Method is not tested.");

    let data = await this.contract.getNetwotkData(this.network);

    return data.authority.toString();

  }

  async updateStakeConfiguration(
    token?: string,
    unlockDuration?: number,
    validatorStake?: string,
    tracerStake?: string,
    publisherStake?: string,
    authorityStake?: string,
  ): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.updateStakeConfiguration(
      this.network,
      token,
      unlockDuration,
      validatorStake,
      tracerStake,
      publisherStake,
      authorityStake,
    );

    return { transactionHash };

  }

  async getStakeConfiguration(): Promise<StakeConfiguration> {
    throw new Error("Method is not tested.");

    let data = await this.contract.getNetwotkData(this.network);

    return {
      token: data.stakeMint.toString(),
      unlockDuration: data.unlockTimestamp.toNumber(),
      validatorStake: data.stakeConfiguration.validatorStake.toString(),
      tracerStake: data.stakeConfiguration.tracerStake.toString(),
      publisherStake: data.stakeConfiguration.publisherStake.toString(),
      authorityStake: data.stakeConfiguration.authorityStake.toString(),
    }
  }

  async updateRewardConfiguration(
    token?: string,
    addressConfirmationReward?: string,
    addresstraceReward?: string
  ): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.updateRewardConfiguration(this.network, token, addressConfirmationReward, addresstraceReward);

    return { transactionHash };
  }

  async getRewardConfiguration(): Promise<RewardConfiguration> {
    throw new Error("Method is not tested.");

    let data = await this.contract.getNetwotkData(this.network);

    return {
      token: data.rewardMint.toString(),
      addressConfirmationReward: data.rewardConfiguration.addressConfirmationReward.toString(),
      tracerReward: data.rewardConfiguration.addressTracerReward.toString()
    };
  }

  async createReporter(
    id: string,
    role: ReporterRole,
    account: string,
    name: string,
    url: string
  ): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.createReporter(this.network, id, ReporterRoleNames[role], account, name, url);

    return { transactionHash };
  }

  async getReporter(id: string): Promise<Reporter> {
    // throw new Error("Method is not tested.");
    let data = await this.contract.getNetwotkData(this.network);

    // TODO: fix it
    return {
      id: data.id.toString(),
      account: data.account.toString(),
      role: ReporterRole[data.role as ReporterRoleKeys],
      status: data.status,
      name: data.name,
      url: data.url,
      stake: data.stake.toString(),
      unlockTimestamp: data.unlockTimestamp.toNumber()
    };


  }

  async getReporterCount(): Promise<number> {
    throw new Error("Method not implemented.");
  }

  async getReporters(skip: number, take: number): Promise<Reporter[]> {
    throw new Error("Method not implemented.");
  }

  async updateReporter(
    id: string,
    role: ReporterRole,
    account?: string,
    name?: string,
    url?: string
  ): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.updateReporter(this.network, id, ReporterRoleNames[role], account, name, url);

    return { transactionHash };

  }

  async activateReporter(): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.activateReporter(this.network, id, this.signer);

    return { transactionHash };

  }

  async deactivateReporter(): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.deactivateReporter(this.network, id, this.signer);

    return { transactionHash };
  }

  async unstakeReporter(): Promise<Result> {
    throw new Error("Method is not tested.");

    const transactionHash = await this.contract.unstake(this.network, id, this.signer);

    return { transactionHash };
  }

  async createCase(id: string, name: string, url: string): Promise<Result> {
    throw new Error("Method not implemented.");
  }

  async getCase(id: string): Promise<Case> {
    throw new Error("Method not implemented.");
  }

  async getCaseCount(): Promise<number> {
    throw new Error("Method not implemented.");
  }

  async getCases(skip: number, take: number): Promise<Case[]> {
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

  async getAddressCount(): Promise<number> {
    throw new Error("Method not implemented.");
  }

  async getAddresses(skip: number, take: number): Promise<Address[]> {
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

  async getAssetCount(): Promise<number> {
    throw new Error("Method not implemented.");
  }

  async getAssets(skip: number, take: number): Promise<Asset[]> {
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
