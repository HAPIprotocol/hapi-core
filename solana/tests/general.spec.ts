import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import {
  getReporters,
  getNetworks,
  getCases,
  getAddresses,
  setupNetworks,
  setupReporters,
  setupCases,
  HAPI_CORE_TEST_ID,
  getAssets,
} from "./util/setup";

import {
  ACCOUNT_SIZE,
  HapiCoreProgram,
  Category,
  uuidToBn,
  CaseStatus,
  decodeAddress,
  bufferFromString,
  ReporterRole,
} from "../lib";

describe("HapiCore General", () => {
  const program = new HapiCoreProgram(new web3.PublicKey(HAPI_CORE_TEST_ID));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "GeneralNetwork";

  const REPORTERS = getReporters();
  let NETWORKS = getNetworks([mainNetwork]);
  const CASES = getCases();
  const ADDRESSES = getAddresses();
  const ASSETS = getAssets();

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      await program.program.provider.connection.requestAirdrop(
        reporter.keypair.publicKey,
        web3.LAMPORTS_PER_SOL
      );

      await stakeToken.transfer(null, reporter.keypair.publicKey, 100_000_000);
    }

    NETWORKS[mainNetwork].stakeConfiguration.unlockDuration = new BN(1);
  });

  it("Initialize network", async () => {
    const [programDataAddress] = web3.PublicKey.findProgramAddressSync(
      [program.programId.toBytes()],
      new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    );

    const network = NETWORKS[mainNetwork];

    const name = bufferFromString(network.name, 32);
    const [networkAccount, bump] = program.findNetworkAddress(network.name);

    await program.program.methods
      .createNetwork(
        name.toJSON().data,
        network.stakeConfiguration,
        network.rewardConfiguration,
        bump
      )
      .accounts({
        authority: provider.wallet.publicKey,
        network: networkAccount,
        rewardMint: rewardToken.mintAccount,
        stakeMint: stakeToken.mintAccount,
        programAccount: program.programId,
        programData: programDataAddress,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("Initialize reporters", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];
      const id = uuidToBn(reporter.id);

      await program.program.methods
        .createReporter(
          id,
          reporter.keypair.publicKey,
          mainNetwork,
          reporterRole,
          reporter.url,
          bump
        )
        .accounts({
          authority: provider.wallet.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
    }
  });

  it("Activate reporters", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await program.program.methods
        .activateReporter()
        .accounts({
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          networkStakeTokenAccount,
          reporterStakeTokenAccount,
          tokenProgram: stakeToken.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Create cases", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    const reporter = REPORTERS.authority;
    const [reporterAccount] = program.findReporterAddress(
      networkAccount,
      reporter.id
    );

    for (const key of Object.keys(CASES)) {
      const cs = CASES[key];

      const [caseAccount, bump] = program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const id = uuidToBn(cs.id);

      await program.program.methods
        .createCase(id, cs.name, cs.url, bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Create addresses", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    const reporter = REPORTERS.publisher;
    const [reporterAccount] = program.findReporterAddress(
      networkAccount,
      reporter.id
    );

    const cs = CASES.firstCase;
    const [caseAccount] = await program.findCaseAddress(networkAccount, cs.id);

    for (const key of Object.keys(ADDRESSES)) {
      const address = ADDRESSES[key];

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .createAddress(
          [...address.address],
          Category[address.category],
          address.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Create assets", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    const reporter = REPORTERS.publisher;
    const [reporterAccount] = program.findReporterAddress(
      networkAccount,
      reporter.id
    );

    const cs = CASES.firstCase;
    const [caseAccount] = await program.findCaseAddress(networkAccount, cs.id);

    for (const key of Object.keys(ASSETS)) {
      const asset = ASSETS[key];

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .createAsset(
          [...asset.address],
          [...asset.id],
          Category[asset.category],
          asset.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Confirm addresses", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    const reporter = REPORTERS.validator;
    const [reporterAccount] = program.findReporterAddress(
      networkAccount,
      reporter.id
    );

    const cs = CASES.firstCase;
    const [caseAccount] = await program.findCaseAddress(networkAccount, cs.id);

    for (const key of Object.keys(ADDRESSES)) {
      const address = ADDRESSES[key];

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await program.program.methods
        .confirmAddress(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Confirm assets", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    const reporter = REPORTERS.validator;
    const [reporterAccount] = program.findReporterAddress(
      networkAccount,
      reporter.id
    );

    const cs = CASES.firstCase;
    const [caseAccount] = await program.findCaseAddress(networkAccount, cs.id);

    for (const key of Object.keys(ASSETS)) {
      const asset = ASSETS[key];

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await program.program.methods
        .confirmAsset(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Deactivate reporters", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      await program.program.methods
        .deactivateReporter()
        .accounts({
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });

  it("Unstake reporters", async () => {
    const [networkAccount] = program.findNetworkAddress(mainNetwork);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await program.program.methods
        .unstake()
        .accounts({
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          networkStakeTokenAccount,
          reporterStakeTokenAccount,
          tokenProgram: stakeToken.programId,
        })
        .signers([reporter.keypair])
        .rpc();
    }
  });
});
