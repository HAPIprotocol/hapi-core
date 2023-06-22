import {
  Program,
  web3,
  BN,
  Provider,
  Wallet,
  AnchorProvider,
} from "@coral-xyz/anchor";
import { Signer } from "@solana/web3.js";
import * as Token from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { parse as uuidParse } from "uuid";

import { IDL, HapiCoreSolana } from "../target/types/hapi_core_solana";
import {
  bufferFromString,
  addrToSeeds,
  padBuffer,
  stakeConfiguration,
  rewardConfiguration,
  ReporterRole,
  ReporterRoleKeys,
  CaseStatus,
  CaseStatusKeys,
  uuidToBn,
} from ".";

export function encodeAddress(address: string): Buffer {
  return padBuffer(Buffer.from(address), 64);
}

export function decodeAddress(address: Buffer | Uint8Array | number[]): string {
  if (!(address instanceof Buffer)) {
    address = Buffer.from(address);
  }

  return address.filter((b) => b).toString();
}

export class HapiCoreProgram {
  program: Program<HapiCoreSolana>;
  programId: web3.PublicKey;

  constructor(hapiCoreProgramId: string | web3.PublicKey, provider?: Provider) {
    this.programId =
      typeof hapiCoreProgramId === "string"
        ? new web3.PublicKey(hapiCoreProgramId)
        : hapiCoreProgramId;

    this.program = new Program(IDL, this.programId, provider);
  }

  public findProgramDataAddress() {
    return web3.PublicKey.findProgramAddressSync(
      [this.programId.toBytes()],
      new web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    );
  }

  public findNetworkAddress(name: string) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("network"), bufferFromString(name, 32)],
      this.programId
    );
  }

  public findReporterAddress(network: web3.PublicKey, reporterId: string) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("reporter"), network.toBytes(), uuidParse(reporterId)],
      this.programId
    );
  }

  public findCaseAddress(network: web3.PublicKey, caseId: string) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("case"), network.toBytes(), uuidParse(caseId)],
      this.programId
    );
  }

  public findAddressAddress(network: web3.PublicKey, address: Buffer) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("address"), network.toBytes(), ...addrToSeeds(address)],
      this.programId
    );
  }

  public findAssetAddress(
    network: web3.PublicKey,
    mint: Buffer,
    assetId: Buffer | Uint8Array
  ) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("asset"),
        network.toBytes(),
        ...addrToSeeds(mint),
        assetId,
      ],
      this.programId
    );
  }

  public async InitializeNetwork(
    name: string,
    stakeConfiguration: stakeConfiguration,
    rewardConfiguration: rewardConfiguration,
    rewardMint: web3.PublicKey,
    stakeMint: web3.PublicKey
  ) {
    const [network, bump] = this.findNetworkAddress(name);
    const [programData] = this.findProgramDataAddress();

    await Token.getOrCreateAssociatedTokenAccount(
      this.program.provider.connection,
      ((this.program.provider as AnchorProvider).wallet as NodeWallet).payer,
      stakeMint,
      network,
      true
    );

    const transactionHash = await this.program.methods
      .createNetwork(
        bufferFromString(name, 32).toJSON().data,
        stakeConfiguration,
        rewardConfiguration,
        bump
      )
      .accounts({
        authority: this.program.provider.publicKey,
        network,
        rewardMint,
        stakeMint,
        programAccount: this.program.programId,
        programData,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    return transactionHash;
  }

  public async getNetwotkData(name: string) {
    const [network] = this.findNetworkAddress(name);
    let data = await this.program.account.network.fetch(network);

    return data;
  }

  public async getReporterData(network_name: string, id: string) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, id);
    let data = await this.program.account.reporter.fetch(reporter);

    return data;
  }

  public async getCaseData(network_name: string, id: string) {
    const [network] = this.findNetworkAddress(network_name);
    const [caseAccount] = this.findCaseAddress(network, id);

    let data = await this.program.account.case.fetch(caseAccount);

    return data;
  }

  public async getAllReporters(network_name: string) {
    const [network] = this.findNetworkAddress(network_name);
    let data = await this.program.account.reporter.all();
    const res = data.filter((acc) => acc.account.network === network);

    return res;
  }

  public async getAllCases(network_name: string) {
    const [network] = this.findNetworkAddress(network_name);
    let data = await this.program.account.case.all();
    const res = data.filter((acc) => acc.account.network === network);

    return res;
  }

  public async setAuthority(network_name: string, address: web3.PublicKey) {
    const [network] = this.findNetworkAddress(network_name);
    const [programData] = this.findProgramDataAddress();

    const transactionHash = await this.program.methods
      .setAuthority()
      .accounts({
        authority: this.program.provider.publicKey,
        newAuthority: address,
        network,
        programAccount: this.programId,
        programData,
      })
      .rpc();

    return transactionHash;
  }

  public async updateStakeConfiguration(
    network_name: string,
    token?: web3.PublicKey,
    unlockDuration?: number,
    validatorStake?: string,
    tracerStake?: string,
    publisherStake?: string,
    authorityStake?: string,
    appraiserStake?: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    let networkData = await this.program.account.network.fetch(network);
    let stakeMint = token ?? networkData.stakeMint;

    const stakeConfiguration = {
      unlockDuration: unlockDuration
        ? new BN(unlockDuration)
        : networkData.stakeConfiguration.unlockDuration,
      validatorStake: validatorStake
        ? new BN(validatorStake)
        : networkData.stakeConfiguration.validatorStake,
      tracerStake: tracerStake
        ? new BN(tracerStake)
        : networkData.stakeConfiguration.tracerStake,
      publisherStake: publisherStake
        ? new BN(publisherStake)
        : networkData.stakeConfiguration.publisherStake,
      authorityStake: authorityStake
        ? new BN(authorityStake)
        : networkData.stakeConfiguration.authorityStake,
      appraiserStake: appraiserStake
        ? new BN(appraiserStake)
        : networkData.stakeConfiguration.appraiserStake,
    };

    const transactionHash = await this.program.methods
      .updateStakeConfiguration(stakeConfiguration)
      .accounts({
        authority: this.program.provider.publicKey,
        network: network,
        stakeMint,
      })
      .rpc();

    return transactionHash;
  }

  public async updateRewardConfiguration(
    network_name: string,
    token?: web3.PublicKey,
    addressTracerReward?: string,
    addressConfirmationReward?: string,
    assetTracerReward?: string,
    assetConfirmationReward?: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    let networkData = await this.program.account.network.fetch(network);
    let rewardMint = token ?? networkData.rewardMint;

    const rewardConfiguration = {
      addressTracerReward: addressTracerReward
        ? new BN(addressTracerReward)
        : networkData.rewardConfiguration.addressTracerReward,
      addressConfirmationReward: addressConfirmationReward
        ? new BN(addressConfirmationReward)
        : networkData.rewardConfiguration.addressConfirmationReward,
      assetTracerReward: assetTracerReward
        ? new BN(assetTracerReward)
        : networkData.rewardConfiguration.assetTracerReward,
      assetConfirmationReward: assetConfirmationReward
        ? new BN(assetConfirmationReward)
        : networkData.rewardConfiguration.assetConfirmationReward,
    };

    const transactionHash = await this.program.methods
      .updateRewardConfiguration(rewardConfiguration)
      .accounts({
        authority: this.program.provider.publicKey,
        network,
        rewardMint,
      })
      .rpc();

    return transactionHash;
  }

  async createReporter(
    network_name: string,
    id: string,
    role: ReporterRoleKeys,
    account: web3.PublicKey,
    name: string,
    url: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporterAccount, bump] = this.findReporterAddress(network, id);

    const transactionHash = await this.program.methods
      .createReporter(
        uuidToBn(id),
        account,
        name,
        ReporterRole[role],
        url,
        bump
      )
      .accounts({
        authority: this.program.provider.publicKey,
        reporter: reporterAccount,
        network,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    return transactionHash;
  }

  async updateReporter(
    network_name: string,
    id: string,
    role?: ReporterRoleKeys,
    account?: web3.PublicKey,
    name?: string,
    url?: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, id);
    const reporterData = await this.program.account.reporter.fetch(reporter);

    const reporterRole = role ? ReporterRole[role] : reporterData.role;
    const reporterUrl = url ?? reporterData.url;
    const reporterAccount = account ?? reporterData.account;
    const reporterName = name ?? reporterData.name.toString();

    const transactionHash = await this.program.methods
      .updateReporter(reporterAccount, reporterName, reporterRole, reporterUrl)
      .accounts({
        authority: this.program.provider.publicKey,
        reporter,
        network,
      })
      .rpc();

    return transactionHash;
  }

  async activateReporter(
    network_name: string,
    wallet: Signer | Wallet,
    id: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, id);
    const networkData = await this.program.account.network.fetch(network);

    let signer = wallet as Signer;

    const networkStakeTokenAccount = Token.getAssociatedTokenAddressSync(
      networkData.stakeMint,
      network,
      true
    );

    const reporterStakeTokenAccount = Token.getAssociatedTokenAddressSync(
      networkData.stakeMint,
      signer.publicKey
    );

    const transactionHash = await this.program.methods
      .activateReporter()
      .accounts({
        signer: signer.publicKey,
        network,
        reporter,
        networkStakeTokenAccount,
        reporterStakeTokenAccount,
        tokenProgram: Token.TOKEN_PROGRAM_ID,
      })
      .signers([signer as Signer])
      .rpc();

    return transactionHash;
  }

  async deactivateReporter(
    network_name: string,
    wallet: Signer | Wallet,
    id: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, id);
    let signer = wallet as Signer;

    const transactionHash = await this.program.methods
      .deactivateReporter()
      .accounts({
        signer: signer.publicKey,
        network,
        reporter,
      })
      .signers([signer])
      .rpc();

    return transactionHash;
  }

  async unstake(network_name: string, wallet: Signer | Wallet, id: string) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, id);
    const networkData = await this.program.account.network.fetch(network);

    let signer = wallet as Signer;

    const networkStakeTokenAccount = Token.getAssociatedTokenAddressSync(
      networkData.stakeMint,
      network,
      true
    );

    const reporterStakeTokenAccount = Token.getAssociatedTokenAddressSync(
      networkData.stakeMint,
      signer.publicKey
    );

    const transactionHash = await this.program.methods
      .unstake()
      .accounts({
        signer: signer.publicKey,
        network,
        reporter,
        networkStakeTokenAccount,
        reporterStakeTokenAccount,
        tokenProgram: Token.TOKEN_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    return transactionHash;
  }

  async createCase(
    network_name: string,
    id: string,
    name: string,
    url: string,
    wallet: Signer | Wallet,
    reporter_id: string
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, reporter_id);
    const [caseAccount, bump] = this.findCaseAddress(network, id);

    let signer = wallet as Signer;

    const transactionHash = await this.program.methods
      .createCase(uuidToBn(id), name, url, bump)
      .accounts({
        sender: signer.publicKey,
        network,
        reporter,
        case: caseAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    return transactionHash;
  }

  async updateCase(
    network_name: string,
    reporter_id: string,
    id: string,
    wallet: Signer | Wallet,
    name?: string,
    url?: string,
    status?: CaseStatusKeys
  ) {
    const [network] = this.findNetworkAddress(network_name);
    const [reporter] = this.findReporterAddress(network, reporter_id);
    const [caseAccount] = this.findCaseAddress(network, id);

    const caseData = await this.program.account.case.fetch(caseAccount);
    const caseStatus = status
      ? CaseStatus[status]
      : (caseData.status as (typeof CaseStatus)[keyof typeof CaseStatus]);
    const caseUrl = url ?? caseData.url;
    const caseName = name ?? caseData.name.toString();

    let signer = wallet as Signer;

    const transactionHash = await this.program.methods
      .updateCase(caseName, caseUrl, caseStatus)
      .accounts({
        sender: signer.publicKey,
        network,
        reporter,
        case: caseAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    return transactionHash;
  }
}
