import { Provider, AnchorProvider, web3 } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import {
  HapiCoreProgram,
  ReporterRoleKeys,
  CaseStatusKeys,
  CategoryKeys,
  bnToUuid,
  stringFromArray,
  getReporterRoleIndex,
  getReporterStatusIndex,
  ReporterStatus as SolReporterStatus,
  ReporterRole as SolReporterRole,
} from "../../../solana/lib";

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
  ReporterRoleNames,
  Result,
  RewardConfiguration,
  StakeConfiguration,
  HapiCoreAddresses,
  ReporterStatus,
} from "../interface";

import {
  ReporterRoleFromString,
  ReporterRoleToString,
  ReporterStatusFromString,
  CaseStatusFromString,
  CategoryFromString,
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

  async getReporterId(): Promise<string> {
    const data = await this.contract.getAllReporters(this.network);
    let reporterAccount = data.find((acc) =>
      acc.account.account.equals(this.provider.publicKey)
    );

    if (!reporterAccount) {
      throw new Error("Reporter does not exist");
    } else {
      return bnToUuid(reporterAccount.account.id);
    }
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
      unlockDuration: data.stakeConfiguration.unlockDuration.toNumber(),
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
      addresstraceReward,
      addressConfirmationReward
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
      id,
      ReporterRoleNames[role] as ReporterRoleKeys,
      new web3.PublicKey(account),
      name,
      url
    );

    return { transactionHash };
  }

  async getReporter(id: string): Promise<Reporter> {
    const data = await this.contract.getReporterData(this.network, id);

    const reporterRole = getReporterRoleIndex(
      data.role as typeof SolReporterRole
    );

    // TODO: should i add appraiser to cli lib?
    if (reporterRole > 3) {
      throw new Error("Invalid reporter role");
    }

    const ReporterStatus = getReporterStatusIndex(
      data.status as typeof SolReporterStatus
    );

    return {
      id: bnToUuid(data.id),
      account: data.account.toString(),
      role: reporterRole as ReporterRole,
      status: ReporterStatus,
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

    let res = data.map((acc) => {
      const data = acc.account;

      const reporterRole = getReporterRoleIndex(
        data.role as typeof SolReporterRole
      );

      if (reporterRole > 3) {
        throw new Error("Invalid reporter role");
      }

      const ReporterStatus = getReporterStatusIndex(
        data.status as typeof SolReporterStatus
      );

      return {
        id: bnToUuid(data.id),
        account: data.account.toString(),
        role: reporterRole,
        status: ReporterStatus,
        name: data.name.toString(),
        url: data.url,
        stake: data.stake.toString(),
        unlockTimestamp: data.unlockTimestamp.toNumber(),
      };
    });

    return res.slice(skip, skip + take);
  }

  async updateReporter(
    id: string,
    role?: ReporterRole,
    account?: string,
    name?: string,
    url?: string
  ): Promise<Result> {
    let reporterRole = role
      ? (ReporterRoleNames[role] as ReporterRoleKeys)
      : undefined;
    let reporterAccount = account ? new web3.PublicKey(account) : undefined;

    const transactionHash = await this.contract.updateReporter(
      this.network,
      id,
      reporterRole,
      reporterAccount,
      name,
      url
    );

    return { transactionHash };
  }

  async activateReporter(): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.activateReporter(
      this.network,
      // this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async deactivateReporter(): Promise<Result> {
    const reporterId = await this.getReporterId();
    const transactionHash = await this.contract.deactivateReporter(
      this.network,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async unstakeReporter(): Promise<Result> {
    const reporterId = await this.getReporterId();
    const transactionHash = await this.contract.unstake(
      this.network,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async createCase(id: string, name: string, url: string): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.createCase(
      this.network,
      id,
      name,
      url,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async getCase(id: string): Promise<Case> {
    const data = await this.contract.getCaseData(this.network, id);

    return {
      id: bnToUuid(data.id),
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
      id: bnToUuid(acc.account.id),
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
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.updateCase(
      this.network,
      reporterId,
      id,
      this.provider.wallet as NodeWallet,
      name,
      url,
      status.toString() as CaseStatusKeys
    );

    return { transactionHash };
  }

  async createAddress(
    address: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.createAddress(
      this.network,
      address,
      category.toString() as CategoryKeys,
      risk,
      caseId,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async getAddress(address: string): Promise<Address> {
    const data = await this.contract.getAddressData(this.network, address);

    return {
      address: stringFromArray(data.address),
      caseId: bnToUuid(data.caseId),
      reporterId: bnToUuid(data.reporterId),
      risk: data.riskScore,
      category: CategoryFromString(data.category.toString()),
    };
  }

  async getAddressCount(): Promise<number> {
    const count = (await this.contract.getAllAddresses(this.network)).length;

    return count;
  }

  async getAddresses(skip: number, take: number): Promise<Address[]> {
    const data = await this.contract.getAllAddresses(this.network);

    let res = data.map((acc) => ({
      address: stringFromArray(acc.account.address),
      caseId: bnToUuid(acc.account.caseId),
      reporterId: bnToUuid(acc.account.reporterId),
      risk: acc.account.riskScore,
      category: CategoryFromString(acc.account.category.toString()),
    }));

    return res.slice(skip, skip + take);
  }

  async updateAddress(
    address: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.updateAddress(
      this.network,
      address,
      this.provider.wallet as NodeWallet,
      reporterId,
      category.toString() as CategoryKeys,
      risk,
      caseId
    );

    return { transactionHash };
  }

  // TODO: this method is absent in interface
  async confirmAddress(address: string): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.confirmAddress(
      this.network,
      address,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async createAsset(
    address: string,
    assetId: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.createAsset(
      this.network,
      address,
      assetId,
      category.toString() as CategoryKeys,
      risk,
      caseId,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }

  async getAsset(address: string, assetId: string): Promise<Asset> {
    const data = await this.contract.getAddressData(this.network, address);

    return {
      address: stringFromArray(data.address),
      assetId: bnToUuid(data.id),
      caseId: bnToUuid(data.caseId),
      reporterId: bnToUuid(data.reporterId),
      risk: data.riskScore,
      category: CategoryFromString(data.category.toString()),
    };
  }

  async getAssetCount(): Promise<number> {
    const count = (await this.contract.getAllAssets(this.network)).length;

    return count;
  }

  async getAssets(skip: number, take: number): Promise<Asset[]> {
    const data = await this.contract.getAllAddresses(this.network);

    let res = data.map((acc) => ({
      address: stringFromArray(acc.account.address),
      assetId: bnToUuid(acc.account.id),
      caseId: bnToUuid(acc.account.caseId),
      reporterId: bnToUuid(acc.account.reporterId),
      risk: acc.account.riskScore,
      category: CategoryFromString(acc.account.category.toString()),
    }));

    return res.slice(skip, skip + take);
  }

  async updateAsset(
    address: string,
    assetId: string,
    caseId: string,
    risk: number,
    category: Category
  ): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.updateAsset(
      this.network,
      address,
      assetId,
      this.provider.wallet as NodeWallet,
      reporterId,
      category.toString() as CategoryKeys,
      risk,
      caseId
    );

    return { transactionHash };
  }

  // TODO: this method is absent in interface
  async confirmAsset(address: string, assetId: string): Promise<Result> {
    const reporterId = await this.getReporterId();

    const transactionHash = await this.contract.confirmAsset(
      this.network,
      address,
      assetId,
      this.provider.wallet as NodeWallet,
      reporterId
    );

    return { transactionHash };
  }
}
