import { Program, web3, BN, Provider, AnchorProvider, utils } from "@coral-xyz/anchor";
import { HapiCoreProgram, bufferFromString } from "../../../solana/lib";


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
  // signer?: unknown;
}

export class HapiCoreSolana implements HapiCore {
  private contract: HapiCoreProgram;
  private provider: AnchorProvider;
  private network: web3.PublicKey

  constructor (options: SolanaConnectionOptions) {
    this.provider = options.provider as AnchorProvider;
    this.contract = new HapiCoreProgram(
      options.address || HapiCoreAddresses[options.network],
      options.provider);

    this.network = this.contract.findNetworkAddress(options.network)[0];
  }

  async setAuthority(address: string): Promise<Result> {
    // throw new Error("Method is not tested.");

    let newAuthority = new web3.PublicKey(address);
    const programData = this.contract.findProgramDataAddress()[0];

    const transactionHash = await this.contract.program.methods.setAuthority().accounts({
      authority: this.provider.wallet.publicKey,
      newAuthority,
      network: this.network,
      programAccount: this.contract.programId,
      programData
    }).rpc();

    return { transactionHash };
  }

  async getAuthority(): Promise<string> {
    // throw new Error("Method is not tested.");
    let data = await this.contract.program.account.network.fetch(this.network);

    return data.authority.toString();

  }

  async updateStakeConfiguration(
    token: string,
    unlockDuration: number,
    validatorStake: string,
    tracerStake: string,
    publisherStake: string,
    authorityStake: string,
  ): Promise<Result> {
    // throw new Error("Method is not tested.");
    let data = await this.contract.program.account.network.fetch(this.network)
    let stakeMint = new web3.PublicKey(token);

    // TODO: need to add appraiserStake field
    const stakeConfiguration = {
      unlockDuration: new BN(unlockDuration),
      validatorStake: new BN(validatorStake),
      tracerStake: new BN(tracerStake),
      publisherStake: new BN(publisherStake),
      authorityStake: new BN(authorityStake),
      appraiserStake: data.stakeConfiguration.appraiserStake,
    };


    const transactionHash = await this.contract.program.methods.updateStakeConfiguration(stakeConfiguration).accounts({
      authority: this.provider.wallet.publicKey,
      network: this.network,
      stakeMint
    }).rpc();

    return { transactionHash };

  }

  async getStakeConfiguration(): Promise<StakeConfiguration> {
    // throw new Error("Method is not tested.");
    let data = await this.contract.program.account.network.fetch(this.network)

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
    token: string,
    addressConfirmationReward: string,
    traceReward: string
  ): Promise<Result> {
    // throw new Error("Method is not tested.");
    let data = await this.contract.program.account.network.fetch(this.network)
    let rewardMint = new web3.PublicKey(token);

    // TODO: need to add assetTracerReward and assetConfirmationReward field
    const rewardConfiguration = {
      addressTracerReward: new BN(traceReward),
      addressConfirmationReward: new BN(addressConfirmationReward),
      assetTracerReward: data.rewardConfiguration.assetTracerReward,
      assetConfirmationReward: data.rewardConfiguration.assetConfirmationReward,
    };


    const transactionHash = await this.contract.program.methods.updateRewardConfiguration(rewardConfiguration).accounts({
      authority: this.provider.wallet.publicKey,
      network: this.network,
      rewardMint
    }).rpc();

    return { transactionHash };
  }

  async getRewardConfiguration(): Promise<RewardConfiguration> {
    // throw new Error("Method is not tested.");

    let data = await this.contract.program.account.network.fetch(this.network);

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
    // throw new Error("Method is not tested.");

    const [reporterAccount, bump] = this.contract.findReporterAddress(
      this.network, new BN(id)
    );

    const args = [
      new BN(id),
      new web3.PublicKey(account),
      bufferFromString(name, 32).toJSON().data,
      ReporterRoleNames[role],
      url,
      bump,
    ];

    const transactionHash = await this.contract.program.methods.createReporter(..args).accounts({
      authority: this.provider.wallet.publicKey,
      reporter: reporterAccount,
      network: this.network,
      systemProgram: web3.SystemProgram.programId,
    }).rpc();

    return { transactionHash };
  }

  async getReporter(id: string): Promise<Reporter> {
    throw new Error("Method not implemented.");
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
