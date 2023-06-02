import { Program, web3, BN, Provider } from "@coral-xyz/anchor";
import { IDL, HapiCoreSolana } from "../target/types/hapi_core_solana";
import { bufferFromString, addrToSeeds, padBuffer, stakeConfiguration, rewardConfiguration, ReporterRole, ReporterRoleVariants } from ".";
import { ReporterRoleKeys } from "@hapi.one/core-cli";
import { Keypair } from "@solana/web3.js";
import * as Token from "@solana/spl-token";

export function encodeAddress(
  address: string
): Buffer {
  return padBuffer(Buffer.from(address), 64);
}

export function decodeAddress(
  address: Buffer | Uint8Array | number[],
): string {
  if (!(address instanceof Buffer)) {
    address = Buffer.from(address);
  }

  return address.filter((b) => b).toString();
}

export class HapiCoreProgram {
  program: Program<HapiCoreSolana>;
  programId: web3.PublicKey;

  constructor (
    hapiCoreProgramId: string | web3.PublicKey,
    provider?: Provider) {
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
      [
        bufferFromString("network"),
        bufferFromString(name, 32),
      ],
      this.programId
    );
  }

  public findReporterAddress(
    network: web3.PublicKey,
    reporterId: BN
  ) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("reporter"), network.toBytes(), new Uint8Array(reporterId.toArray("le", 8))],
      this.programId
    );
  }

  public findCaseAddress(caseId: BN) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("case"),
        new Uint8Array(caseId.toArray("le", 8)),
      ],
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

  public async InitializeNetwotk(
    name: string,
    stakeConfiguration: stakeConfiguration,
    rewardConfiguration: rewardConfiguration,
    rewardToken: string,
    stakeToken: string,
  ) {
    const [network, bump] = this.findNetworkAddress(name);
    const programData = this.findProgramDataAddress()[0];
    const stakeMint = new web3.PublicKey(stakeToken);
    const rewardMint = new web3.PublicKey(rewardToken);

    const transactionHash = this.program.methods.createNetwork(
      bufferFromString(name, 32).toJSON().data,
      stakeConfiguration,
      rewardConfiguration,
      bump,
    ).accounts({
      authority: this.program.provider.publicKey,
      network,
      rewardMint,
      stakeMint,
      programAccount: this.program.programId,
      programData,
      systemProgram: web3.SystemProgram.programId,
    },).rpc();

    return transactionHash;
  }

  public async getNetwotkData(
    name: string,
  ) {
    const network = this.findNetworkAddress(name)[0];
    let data = await this.program.account.network.fetch(network);

    return data;
  }

  public async getReporterData(
    network_name: string,
    id: string,
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const reporter = this.findReporterAddress(network, new BN(id))[0];
    let data = await this.program.account.reporter.fetch(reporter);

    return data;
  }

  public async getAllReporters(
    network_name: string,
  ) {
    let res = [];
    const network = this.findNetworkAddress(network_name)[0];
    let data = await this.program.account.reporter.all();

    data.map((acc) => {
      if (acc.account.network == network)
        res.push(acc)
    })

    return data;
  }

  public async setAuthority(
    network_name: string,
    address: string
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    let newAuthority = new web3.PublicKey(address);
    const programData = this.findProgramDataAddress()[0];

    const transactionHash = await this.program.methods.setAuthority().accounts({
      authority: this.program.provider.publicKey,
      newAuthority,
      network,
      programAccount: this.programId,
      programData
    }).rpc();

    return transactionHash;
  }

  public async updateStakeConfiguration(
    network_name: string,
    token?: string,
    unlockDuration?: number,
    validatorStake?: string,
    tracerStake?: string,
    publisherStake?: string,
    authorityStake?: string,
    appraiserStake?: string,
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    let network_data = (await this.program.account.network.fetch(network));
    let stakeMint = token ? new web3.PublicKey(token) : network_data.stakeMint;

    const stakeConfiguration = {
      unlockDuration: unlockDuration ? new BN(unlockDuration) : network_data.stakeConfiguration.unlockDuration,
      validatorStake: validatorStake ? new BN(validatorStake) : network_data.stakeConfiguration.validatorStake,
      tracerStake: tracerStake ? new BN(tracerStake) : network_data.stakeConfiguration.tracerStake,
      publisherStake: publisherStake ? new BN(publisherStake) : network_data.stakeConfiguration.publisherStake,
      authorityStake: authorityStake ? new BN(authorityStake) : network_data.stakeConfiguration.authorityStake,
      appraiserStake: appraiserStake ? new BN(appraiserStake) : network_data.stakeConfiguration.appraiserStake,
    };

    const transactionHash = await this.program.methods.updateStakeConfiguration(stakeConfiguration).accounts({
      authority: this.program.provider.publicKey,
      network: network,
      stakeMint
    }).rpc();

    return transactionHash;

  }

  public async updateRewardConfiguration(
    network_name: string,
    token?: string,
    addressTracerReward?: string,
    addressConfirmationReward?: string,
    assetTracerReward?: string,
    assetConfirmationReward?: string

  ) {
    const network = this.findNetworkAddress(network_name)[0];
    let network_data = (await this.program.account.network.fetch(network));
    let rewardMint = token ? new web3.PublicKey(token) : network_data.rewardMint;

    const rewardConfiguration = {
      addressTracerReward: addressTracerReward ? new BN(addressTracerReward) : network_data.rewardConfiguration.addressTracerReward,
      addressConfirmationReward: addressConfirmationReward ? new BN(addressConfirmationReward) : network_data.rewardConfiguration.addressConfirmationReward,
      assetTracerReward: assetTracerReward ? new BN(assetTracerReward) : network_data.rewardConfiguration.assetTracerReward,
      assetConfirmationReward: assetConfirmationReward ? new BN(assetConfirmationReward) : network_data.rewardConfiguration.assetConfirmationReward,
    };

    const transactionHash = await this.program.methods.updateRewardConfiguration(rewardConfiguration).accounts({
      authority: this.program.provider.publicKey,
      network,
      rewardMint
    }).rpc();

    return transactionHash;
  }

  async createReporter(
    network_name: string,
    id: string,
    role: string,
    account: string,
    name: string,
    url: string
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const [reporterAccount, bump] = this.findReporterAddress(
      network, new BN(id)
    );
    // TODO: will it work?
    if (!ReporterRoleVariants.includes(role as ReporterRoleKeys)) {
      throw new Error("Invalid reporter role");
    }

    const transactionHash = await this.program.methods.createReporter(
      new BN(id),
      new web3.PublicKey(account),
      bufferFromString(name, 32).toJSON().data,
      ReporterRole[role],
      url,
      bump,).accounts({
        authority: this.program.provider.publicKey,
        reporter: reporterAccount,
        network,
        systemProgram: web3.SystemProgram.programId,
      }).rpc();

    return transactionHash;
  }

  async updateReporter(
    network_name: string,
    id: string,
    role?: string,
    account?: string,
    name?: string,
    url?: string
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const reporter = this.findReporterAddress(
      network, new BN(id))[0];
    const reporterData = await this.program.account.reporter.fetch(reporter);


    // TODO: will it work?
    if (role && !ReporterRoleVariants.includes(role as ReporterRoleKeys)) {
      throw new Error("Invalid reporter role");
    }

    const reporter_role = role ? ReporterRole[role] : reporterData.role;
    const reporter_url = url ? url : reporterData.url;
    const reporter_account = account ? new web3.PublicKey(account) : reporterData.account;
    const reporter_name = name ? bufferFromString(name, 32).toJSON().data : reporterData.name;

    const transactionHash = await this.program.methods.updateReporter(
      reporter_account,
      reporter_name,
      reporter_role,
      reporter_url
    ).accounts({
      authority: this.program.provider.publicKey,
      reporter,
      network,
    }).rpc();

    return transactionHash;
  }

  async activateReporter(
    network_name: string,
    id: string,
    account?: Keypair,
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const reporter = this.findReporterAddress(
      network, new BN(id))[0];
    const networkData = await this.program.account.network.fetch(network);

    const networkStakeTokenAccount = (await Token.getOrCreateAssociatedTokenAccount(
      this.program.provider.connection,
      this.program.provider.publicKey,
      networkData.stakeMint,
      network,
      true
    )).address;

    const reporterStakeTokenAccount = (await Token.getOrCreateAssociatedTokenAccount(
      this.program.provider.connection,
      this.program.provider.publicKey,
      networkData.stakeMint,
      account.publicKey,
      false
    )).address;

    const tx = this.program.methods.activateReporter(
    ).accounts({
      signer: account ? account.publicKey : this.program.provider.publicKey,
      network,
      reporter,
      networkStakeTokenAccount,
      reporterStakeTokenAccount,
      tokenProgram: Token.TOKEN_PROGRAM_ID
    });

    if (account) {
      tx.signers([account]);
    }

    return await tx.rpc()
  }

  async deactivateReporter(
    network_name: string,
    id: string,
    account?: Keypair,
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const reporter = this.findReporterAddress(
      network, new BN(id))[0];

    const tx = await this.program.methods.deactivateReporter(
    ).accounts({
      signer: account ? account.publicKey : this.program.provider.publicKey,
      network,
      reporter,
    });

    if (account) {
      tx.signers([account]);
    }

    return await tx.rpc()
  }


  async unstake(
    network_name: string,
    id: string,
    account?: Keypair,
  ) {
    const network = this.findNetworkAddress(network_name)[0];
    const reporter = this.findReporterAddress(
      network, new BN(id))[0];
    const networkData = await this.program.account.network.fetch(network);

    const networkStakeTokenAccount = (await Token.getOrCreateAssociatedTokenAccount(
      this.program.provider.connection,
      this.program.provider.publicKey,
      networkData.stakeMint,
      network,
      true
    )).address;

    const reporterStakeTokenAccount = (await Token.getOrCreateAssociatedTokenAccount(
      this.program.provider.connection,
      this.program.provider.publicKey,
      networkData.stakeMint,
      account.publicKey,
      false
    )).address;

    const tx = this.program.methods.unstake(
    ).accounts({
      signer: account ? account.publicKey : this.program.provider.publicKey,
      network,
      reporter,
      networkStakeTokenAccount,
      reporterStakeTokenAccount,
      tokenProgram: Token.TOKEN_PROGRAM_ID
    });

    if (account) {
      tx.signers([account]);
    }

    return await tx.rpc()
  }
}
