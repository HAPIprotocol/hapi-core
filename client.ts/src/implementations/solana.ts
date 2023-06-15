import { Provider, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import { HapiCoreProgram, ReporterRoleKeys } from "../../../solana/lib";

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
} from "../interface";

import {
  ReporterRoleFromString,
  ReporterStatusFromString,
  CaseStatusFromString,
} from "../util";

export interface SolanaConnectionOptions {
  network: HapiCoreNetwork.Solana | HapiCoreNetwork.Bitcoin;
  address?: Addr;
  provider: Provider | SolanaProviderOptions;
}

export interface SolanaProviderOptions {
  providerUrl: string;
}

export class HapiCoreSolana implements HapiCore {
  private contract: HapiCoreProgram;
  private network: string;
  private provider: AnchorProvider;

  constructor(options: SolanaConnectionOptions) {
    let programProvider =
      options.provider.constructor.name === "Provider"
        ? (options.provider as AnchorProvider)
        : (options.provider = AnchorProvider.local(
            (options.provider as SolanaProviderOptions).providerUrl
          ));

    this.contract = new HapiCoreProgram(
      options.address || HapiCoreAddresses[options.network],
      programProvider
    );

    this.network = options.network;
    this.provider = programProvider;
  }

  async setAuthority(address: string): Promise<Result> {
    const transactionHash = await this.contract.setAuthority(
      this.network,
      new web3.PublicKey(address)
    );
    return { transactionHash };
  }

  async getAuthority(): Promise<string> {
    let data = await this.contract.getNetwotkData(this.network);
    return data.authority.toString();
  }

  async updateStakeConfiguration(
    token?: string,
    unlockDuration?: number,
    validatorStake?: string,
    tracerStake?: string,
    publisherStake?: string,
    authorityStake?: string
  ): Promise<Result> {
    let stakeToken = token ? new web3.PublicKey(token) : undefined;

    const transactionHash = await this.contract.updateStakeConfiguration(
      this.network,
      stakeToken,
      unlockDuration,
      validatorStake,
      tracerStake,
      publisherStake,
      authorityStake
    );

    return { transactionHash };
  }

  async getStakeConfiguration(): Promise<StakeConfiguration> {
    let data = await this.contract.getNetwotkData(this.network);

    return {
      token: data.stakeMint.toString(),
      unlockDuration: data.unlockTimestamp.toNumber(),
      validatorStake: data.stakeConfiguration.validatorStake.toString(),
      tracerStake: data.stakeConfiguration.tracerStake.toString(),
      publisherStake: data.stakeConfiguration.publisherStake.toString(),
      authorityStake: data.stakeConfiguration.authorityStake.toString(),
    };
  }

  async updateRewardConfiguration(
    token?: string,
    addressConfirmationReward?: string,
    addresstraceReward?: string
  ): Promise<Result> {
    let rewardToken = token ? new web3.PublicKey(token) : undefined;

    const transactionHash = await this.contract.updateRewardConfiguration(
      this.network,
      rewardToken,
      addressConfirmationReward,
      addresstraceReward
    );

    return { transactionHash };
  }

  async getRewardConfiguration(): Promise<RewardConfiguration> {
    let data = await this.contract.getNetwotkData(this.network);

    return {
      token: data.rewardMint.toString(),
      addressConfirmationReward:
        data.rewardConfiguration.addressConfirmationReward.toString(),
      tracerReward: data.rewardConfiguration.addressTracerReward.toString(),
    };
  }

  async createReporter(
    id: string,
    role: ReporterRole,
    account: string,
    name: string,
    url: string
  ): Promise<Result> {
    const transactionHash = await this.contract.createReporter(
      this.network,
      new BN(id),
      role.toString() as ReporterRoleKeys,
      new web3.PublicKey(account),
      name,
      url
    );

    return { transactionHash };
  }

  async getReporter(id: string): Promise<Reporter> {
    const data = await this.contract.getReporterData(this.network, new BN(id));

    return {
      id: data.id.toString(),
      account: data.account.toString(),
      role: ReporterRoleFromString(data.role as string),
      status: ReporterStatusFromString(data.status as string),
      name: data.name.toString(),
      url: data.url,
      stake: data.stake.toString(),
      unlockTimestamp: data.unlockTimestamp.toNumber(),
    };
  }

  async getReporterCount(): Promise<number> {
    const count = (await this.contract.getAllReporters(this.network)).length;

    return count;
  }

  async getReporters(skip: number, take: number): Promise<Reporter[]> {
    const data = await this.contract.getAllReporters(this.network);

    let res = data.map((acc) => ({
      id: acc.account.id.toString(),
      account: acc.account.account.toString(),
      role: ReporterRoleFromString(acc.account.role as string),
      status: ReporterStatusFromString(acc.account.status as string),
      name: acc.account.name.toString(),
      url: acc.account.url,
      stake: acc.account.stake.toString(),
      unlockTimestamp: acc.account.unlockTimestamp.toNumber(),
    }));

    return res.slice(skip, skip + take);
  }

  async updateReporter(
    id: string,
    role?: ReporterRole,
    account?: string,
    name?: string,
    url?: string
  ): Promise<Result> {
    let reporterRole = role ? (role.toString() as ReporterRoleKeys) : undefined;
    let reporterAccount = account ? new web3.PublicKey(account) : undefined;

    const transactionHash = await this.contract.updateReporter(
      this.network,
      new BN(id),
      reporterRole,
      reporterAccount,
      name,
      url
    );

    return { transactionHash };
  }

  async getReporterAccount(): Promise<BN> {
    const data = await this.contract.getAllReporters(this.network);
    let reporterAccount = data.find((acc) =>
      acc.account.account.equals(this.provider.publicKey)
    );

    if (!reporterAccount) {
      throw new Error("Reporter does not exist");
    } else {
      return reporterAccount.account.id;
    }
  }

  async activateReporter(): Promise<Result> {
    const reporterId = await this.getReporterAccount();

    const transactionHash = await this.contract.activateReporter(
      this.network,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async deactivateReporter(): Promise<Result> {
    const reporterId = await this.getReporterAccount();
    const transactionHash = await this.contract.deactivateReporter(
      this.network,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async unstakeReporter(): Promise<Result> {
    const reporterId = await this.getReporterAccount();
    const transactionHash = await this.contract.unstake(
      this.network,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async createCase(id: string, name: string, url: string): Promise<Result> {
    const reporterId = await this.getReporterAccount();

    const transactionHash = await this.contract.createCase(
      this.network,
      new BN(id),
      name,
      url,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async getCase(id: string): Promise<Case> {
    const data = await this.contract.getCaseData(this.network, new BN(id));

    return {
      id: data.id.toString(),
      name: data.name.toString(),
      url: data.url.toString(),
      status: CaseStatusFromString(data.status.toString()),
    };
  }

  async getCaseCount(): Promise<number> {
    const count = (await this.contract.getAllCases(this.network)).length;

    return count;
  }

  async getCases(skip: number, take: number): Promise<Case[]> {
    const data = await this.contract.getAllCases(this.network);

    let res = data.map((acc) => ({
      id: acc.account.id.toString(),
      name: acc.account.name.toString(),
      url: acc.account.url.toString(),
      status: CaseStatusFromString(acc.account.status.toString()),
    }));

    return res.slice(skip, skip + take);
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
