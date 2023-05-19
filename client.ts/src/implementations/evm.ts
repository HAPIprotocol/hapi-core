import { JsonRpcProvider, Provider } from "@ethersproject/providers";
import { Wallet } from "@ethersproject/wallet";
import type { Signer } from "@ethersproject/abstract-signer";

import * as typechain from "hapi-core-evm/typechain-types";

import {
  Addr,
  Address,
  Amount,
  Asset,
  Case,
  CaseStatus,
  Category,
  HapiCore,
  HapiCoreAddresses,
  HapiCoreNetwork,
  Reporter,
  ReporterRole,
  Result,
  RewardConfiguration,
  StakeConfiguration,
  Uuid,
} from "../interface";
import { bigIntToUuid, uuidToBigNumberish } from "../util";
import { BigNumber } from "@ethersproject/bignumber";

export interface EvmConnectionOptions {
  network: HapiCoreNetwork.Ethereum | HapiCoreNetwork.BSC;
  provider: Provider | EvmProviderOptions;
  address?: Addr;
  signer?: Signer;
  signerPrivateKey?: string;
}

export interface EvmProviderOptions {
  providerUrl: string;
}

export class HapiCoreEvm implements HapiCore {
  private contract: typechain.HapiCore;

  constructor(options: EvmConnectionOptions) {
    if (options.provider.constructor.name !== "Provider") {
      options.provider = new JsonRpcProvider(
        (options.provider as EvmProviderOptions).providerUrl
      );
    }

    if (options.signerPrivateKey && !options.signer) {
      options.signer = new Wallet(
        options.signerPrivateKey,
        options.provider as Provider
      );
    }

    this.contract = typechain.HapiCore__factory.connect(
      options.address || HapiCoreAddresses[options.network],
      options.signer || (options.provider as Provider)
    );
  }

  async setAuthority(address: Addr): Promise<Result> {
    const tx = await this.contract.setAuthority(address);
    return { transactionHash: tx.hash };
  }

  async getAuthority(): Promise<Addr> {
    return await this.contract.authority();
  }

  async updateStakeConfiguration(
    token: Addr,
    unlockDuration: number,
    validatorStake: Amount,
    tracerStake: Amount,
    publisherStake: Amount,
    authorityStake: Amount
  ): Promise<Result> {
    const tx = await this.contract.updateStakeConfiguration(
      token,
      unlockDuration,
      validatorStake,
      tracerStake,
      publisherStake,
      authorityStake
    );
    return { transactionHash: tx.hash };
  }

  async getStakeConfiguration(): Promise<StakeConfiguration> {
    const stakeConfiguration = await this.contract.stakeConfiguration();
    return {
      token: stakeConfiguration.token,
      unlockDuration: stakeConfiguration.unlock_duration.toNumber(),
      validatorStake: stakeConfiguration.validator_stake.toString(),
      tracerStake: stakeConfiguration.tracer_stake.toString(),
      publisherStake: stakeConfiguration.publisher_stake.toString(),
      authorityStake: stakeConfiguration.authority_stake.toString(),
    };
  }

  async updateRewardConfiguration(
    token: Addr,
    addressConfirmationReward: Amount,
    traceReward: Amount
  ): Promise<Result> {
    const tx = await this.contract.updateRewardConfiguration(
      token,
      addressConfirmationReward,
      traceReward
    );
    return { transactionHash: tx.hash };
  }

  async getRewardConfiguration(): Promise<RewardConfiguration> {
    const rewardConfiguration = await this.contract.rewardConfiguration();
    return {
      token: rewardConfiguration.token,
      addressConfirmationReward:
        rewardConfiguration.address_confirmation_reward.toString(),
      tracerReward: rewardConfiguration.tracer_reward.toString(),
    };
  }

  async createReporter(
    id: Uuid,
    role: ReporterRole,
    account: Addr,
    name: string,
    url: string
  ): Promise<Result> {
    const tx = await this.contract.createReporter(
      uuidToBigNumberish(id),
      account,
      role.toString(),
      name,
      url
    );

    return { transactionHash: tx.hash };
  }

  async getReporter(id: Uuid): Promise<Reporter> {
    const reporter = await this.contract.getReporter(uuidToBigNumberish(id));
    return {
      id: bigIntToUuid(reporter.id),
      account: reporter.account,
      role: reporter.role,
      status: reporter.status,
      name: reporter.name,
      url: reporter.url,
      stake: reporter.stake.toString(),
      unlockTimestamp: reporter.unlock_timestamp.toNumber(),
    };
  }

  async updateReporter(
    id: Uuid,
    role: ReporterRole,
    account: Addr,
    name: string,
    url: string
  ): Promise<Result> {
    const tx = await this.contract.updateReporter(
      uuidToBigNumberish(id),
      role.toString(),
      account,
      name,
      url
    );
    return { transactionHash: tx.hash };
  }

  async activateReporter(): Promise<Result> {
    const tx = await this.contract.activateReporter();
    return { transactionHash: tx.hash };
  }

  async deactivateReporter(): Promise<Result> {
    const tx = await this.contract.deactivateReporter();
    return { transactionHash: tx.hash };
  }

  async unstakeReporter(): Promise<Result> {
    const tx = await this.contract.unstake();
    return { transactionHash: tx.hash };
  }

  async createCase(id: string, name: string, url: string): Promise<Result> {
    const tx = await this.contract.createCase(
      uuidToBigNumberish(id),
      name,
      url
    );
    return { transactionHash: tx.hash };
  }

  async getCase(id: Uuid): Promise<Case> {
    const c = await this.contract.getCase(uuidToBigNumberish(id));
    return {
      id: bigIntToUuid(c.id),
      name: c.name,
      url: c.url,
      status: c.status,
    };
  }

  async updateCase(
    id: Uuid,
    name: Addr,
    url: string,
    status: CaseStatus
  ): Promise<Result> {
    const tx = await this.contract.updateCase(
      uuidToBigNumberish(id),
      name,
      url,
      status.toString()
    );
    return { transactionHash: tx.hash };
  }

  async createAddress(
    address: Addr,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result> {
    const tx = await this.contract.createAddress(
      address,
      uuidToBigNumberish(caseId),
      risk,
      category.toString()
    );
    return { transactionHash: tx.hash };
  }

  async getAddress(address: Addr): Promise<Address> {
    const a = await this.contract.getAddress(address);
    return {
      address: a.addr.toString(),
      caseId: bigIntToUuid(a.case_id),
      reporterId: a.reporter_id.toString(),
      risk: a.risk,
      category: a.category,
    };
  }

  async updateAddress(
    address: Addr,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result> {
    const tx = await this.contract.updateAddress(
      address,
      uuidToBigNumberish(caseId),
      risk,
      category.toString()
    );
    return { transactionHash: tx.hash };
  }

  async createAsset(
    address: Addr,
    assetId: string,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result> {
    const tx = await this.contract.createAsset(
      address,
      assetId,
      uuidToBigNumberish(caseId),
      risk,
      category.toString()
    );
    return { transactionHash: tx.hash };
  }

  async getAsset(address: Addr, assetId: string): Promise<Asset> {
    const a = await this.contract.getAsset(address, assetId);
    return {
      address: a.addr.toString(),
      assetId: a.asset_id.toString(),
      caseId: bigIntToUuid(a.case_id),
      reporterId: a.reporter_id.toString(),
      risk: a.risk,
      category: a.category,
    };
  }

  async updateAsset(
    address: Addr,
    assetId: string,
    caseId: Uuid,
    risk: number,
    category: Category
  ): Promise<Result> {
    const tx = await this.contract.updateAsset(
      address,
      assetId,
      caseId,
      risk,
      category.toString()
    );
    return { transactionHash: tx.hash };
  }
}
