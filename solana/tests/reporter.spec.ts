import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { v1 as uuidv1 } from "uuid";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import { getReporters, getNetworks, setupNetworks } from "./util/setup";

import {
  ACCOUNT_SIZE,
  ReporterRole,
  HapiCoreProgram,
  ReporterStatus,
  uuidToBn,
  bufferFromString,
} from "../lib";
import { log } from "console";

describe("HapiCore Reporter", () => {
  const program = new HapiCoreProgram(
    new web3.PublicKey("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk")
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;
  const another_authority = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "ReporterMainNetwork";
  const secondaryNetwork = "ReporterSecondaryNetwork";

  const REPORTERS = getReporters();
  let NETWORKS = getNetworks([mainNetwork, secondaryNetwork]);

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

    await provider.connection.requestAirdrop(
      another_authority.publicKey,
      10_000_000
    );

    NETWORKS[mainNetwork].stakeConfiguration.unlockDuration = new BN(1);

    await setupNetworks(
      program,
      NETWORKS,
      rewardToken.mintAccount,
      stakeToken.mintAccount
    );
  });

  describe("create_reporter", () => {
    it("fail - authority mismatch", async () => {
      const networkName = mainNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.publisher;

      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .createReporter(
              uuidToBn(reporter.id),
              reporter.keypair.publicKey,
              networkName,
              reporterRole,
              reporter.url,
              bump
            )
            .accounts({
              authority: another_authority.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([another_authority])
            .rpc(),
        programError("AuthorityMismatch")
      );
    });

    it("fail - invalid reporter id", async () => {
      const networkName = mainNetwork;
      const reporterId = bufferFromString("invalid-id", 16);

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.publisher;

      const [reporterAccount, bump] = web3.PublicKey.findProgramAddressSync(
        [bufferFromString("reporter"), networkAccount.toBytes(), reporterId],
        program.programId
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .createReporter(
              new BN(reporterId, "be"),
              reporter.keypair.publicKey,
              networkName,
              reporterRole,
              reporter.url,
              bump
            )
            .accounts({
              authority: authority.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .rpc(),
        programError("InvalidUUID")
      );
    });

    it("fail - invalid reporter id", async () => {
      const networkName = mainNetwork;
      const reporterId = uuidv1();

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.publisher;

      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporterId
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .createReporter(
              uuidToBn(reporterId),
              reporter.keypair.publicKey,
              networkName,
              reporterRole,
              reporter.url,
              bump
            )
            .accounts({
              authority: authority.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .rpc(),
        programError("InvalidUUID")
      );
    });

    it("success - alice", async () => {
      const networkName = mainNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.publisher;
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
          networkName,
          reporterRole,
          reporter.url,
          bump
        )
        .accounts({
          authority: authority.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect((fetchedReporterAccount.id as BN).eq(id)).toBeTruthy();
      expect(fetchedReporterAccount.network).toEqual(networkAccount);
      expect(fetchedReporterAccount.account).toEqual(
        reporter.keypair.publicKey
      );
      expect(fetchedReporterAccount.name).toEqual(networkName);
      expect(fetchedReporterAccount.role).toEqual(reporterRole);
      expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.url).toEqual(reporter.url);
      expect(fetchedReporterAccount.bump).toEqual(bump);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
    });

    it("success - bob", async () => {
      const networkName = secondaryNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.tracer;
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
          networkName,
          reporterRole,
          reporter.url,
          bump
        )
        .accounts({
          authority: authority.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect((fetchedReporterAccount.id as BN).eq(id)).toBeTruthy();
      expect(fetchedReporterAccount.network).toEqual(networkAccount);
      expect(fetchedReporterAccount.account).toEqual(
        reporter.keypair.publicKey
      );
      expect(fetchedReporterAccount.name).toEqual(networkName);
      expect(fetchedReporterAccount.role).toEqual(reporterRole);
      expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.url).toEqual(reporter.url);
      expect(fetchedReporterAccount.bump).toEqual(bump);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
    });

    it("success - carol", async () => {
      const networkName = mainNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.authority;
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
          networkName,
          reporterRole,
          reporter.url,
          bump
        )
        .accounts({
          authority: authority.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect((fetchedReporterAccount.id as BN).eq(id)).toBeTruthy();
      expect(fetchedReporterAccount.network).toEqual(networkAccount);
      expect(fetchedReporterAccount.account).toEqual(
        reporter.keypair.publicKey
      );
      expect(fetchedReporterAccount.name).toEqual(networkName);
      expect(fetchedReporterAccount.role).toEqual(reporterRole);
      expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.url).toEqual(reporter.url);
      expect(fetchedReporterAccount.bump).toEqual(bump);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
    });

    it("fail - reporter already exists", async () => {
      const networkName = mainNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.publisher;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .createReporter(
              uuidToBn(reporter.id),
              reporter.keypair.publicKey,
              networkName,
              reporterRole,
              reporter.url,
              bump
            )
            .accounts({
              authority: authority.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .rpc(),
        /custom program error: 0x0/
      );
    });
  });

  describe("update_reporter", () => {
    it("fail - authority mismatch", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .updateReporter(
              reporter.keypair.publicKey,
              reporter.name,
              reporterRole,
              reporter.url
            )
            .accounts({
              authority: another_authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            })
            .signers([another_authority])
            .rpc(),
        programError("AuthorityMismatch")
      );
    });

    it("fail - reporter does not exists", async () => {
      const reporter = REPORTERS.validator;
      const networkName = mainNetwork;

      const [networkAccount] = program.findNetworkAddress(networkName);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .updateReporter(
              reporter.keypair.publicKey,
              reporter.name,
              reporterRole,
              reporter.url
            )
            .accounts({
              authority: authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            })
            .rpc(),
        /The program expected this account to be already initialized/
      );
    });

    it("fail - network mismatch", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(secondaryNetwork);
      const [reporterNetworkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        reporterNetworkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.program.methods
            .updateReporter(
              reporter.keypair.publicKey,
              reporter.name,
              reporterRole,
              reporter.url
            )
            .accounts({
              authority: authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            })
            .rpc(),
        /A seeds constraint was violated/
      );
    });

    it("success", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const reporterRole = ReporterRole[reporter.role];

      await program.program.methods
        .updateReporter(
          reporter.keypair.publicKey,
          reporter.name,
          reporterRole,
          reporter.url
        )
        .accounts({
          authority: authority.publicKey,
          reporter: reporterAccount,
          network: networkAccount,
        })
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(fetchedReporterAccount.account).toEqual(
        reporter.keypair.publicKey
      );
      expect(fetchedReporterAccount.name).toEqual(reporter.name);
      expect(fetchedReporterAccount.role).toEqual(reporterRole);
      expect(fetchedReporterAccount.url).toEqual(reporter.url);
    });
  });

  describe("activate_reporter", () => {
    it("fail - network mismatch", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(secondaryNetwork);
      const [reporterNetworkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        reporterNetworkAccount,
        reporter.id
      );

      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.program.methods
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
            .rpc(),
        /A seeds constraint was violated/
      );
    });

    it("fail - invalid reporter", async () => {
      const reporter = REPORTERS.publisher;
      const anotherReporter = REPORTERS.tracer;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

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

      await expectThrowError(
        () =>
          program.program.methods
            .activateReporter()
            .accounts({
              signer: anotherReporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            })
            .signers([anotherReporter.keypair])
            .rpc(),
        programError("InvalidReporter")
      );
    });

    it("fail - invalid network ATA mint", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const invalidNetworkStakeTokenAccount = await rewardToken.getTokenAccount(
        networkAccount,
        true
      );

      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.program.methods
            .activateReporter()
            .accounts({
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount: invalidNetworkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("InvalidToken")
      );
    });

    it("fail - invalid reporter ATA mint", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const invalidReporterStakeTokenAccount =
        await rewardToken.getTokenAccount(reporter.keypair.publicKey);

      await expectThrowError(
        () =>
          program.program.methods
            .activateReporter()
            .accounts({
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount: invalidReporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("InvalidToken")
      );
    });

    it("fail - invalid network ATA owner", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.program.methods
            .activateReporter()
            .accounts({
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount: reporterStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("IllegalOwner")
      );
    });

    it("fail - invalid reporter ATA owner", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      await expectThrowError(
        () =>
          program.program.methods
            .activateReporter()
            .accounts({
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount: networkStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("IllegalOwner")
      );
    });

    it("fail - insufficient funds", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

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

      await expectThrowError(
        () =>
          program.program.methods
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
            .rpc(),
        /Error processing Instruction 0: custom program error: 0x1/
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.publisher;
      let network = NETWORKS[mainNetwork];

      const [networkAccount] = program.findNetworkAddress(network.name);

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

      await stakeToken.transfer(
        null,
        reporter.keypair.publicKey,
        network.stakeConfiguration.publisherStake.toNumber()
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

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      const reporterBalanceAfter = await stakeToken.getBalance(
        reporter.keypair.publicKey
      );

      const networkBalanceAfter = await stakeToken.getBalance(
        networkAccount,
        true
      );

      expect(
        networkBalanceAfter.eq(network.stakeConfiguration.publisherStake)
      ).toBeTruthy();
      expect(reporterBalanceAfter.isZero()).toBeTruthy();

      expect(
        fetchedReporterAccount.stake.eq(
          network.stakeConfiguration.publisherStake
        )
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.tracer;
      let network = NETWORKS[secondaryNetwork];

      const [networkAccount] = program.findNetworkAddress(network.name);

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

      await stakeToken.transfer(
        null,
        reporter.keypair.publicKey,
        network.stakeConfiguration.tracerStake.toNumber()
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

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      const reporterBalanceAfter = await stakeToken.getBalance(
        reporter.keypair.publicKey
      );

      const networkBalanceAfter = await stakeToken.getBalance(
        networkAccount,
        true
      );

      expect(
        networkBalanceAfter.eq(network.stakeConfiguration.tracerStake)
      ).toBeTruthy();

      expect(reporterBalanceAfter.isZero()).toBeTruthy();

      expect(
        fetchedReporterAccount.stake.eq(network.stakeConfiguration.tracerStake)
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
    });

    it("fail - reporter is already activated", async () => {
      const reporter = REPORTERS.publisher;
      let network = NETWORKS[mainNetwork];
      const [networkAccount] = program.findNetworkAddress(network.name);

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

      await expectThrowError(
        () =>
          program.program.methods
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
            .rpc(),
        programError("InvalidReporterStatus")
      );
    });
  });

  describe("dectivate_reporter", () => {
    it("fail - reporter is not activated", async () => {
      const reporter = REPORTERS.authority;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      await expectThrowError(
        () =>
          program.program.methods
            .deactivateReporter()
            .accounts({
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("InvalidReporterStatus")
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.publisher;
      let network = NETWORKS[mainNetwork];
      const [networkAccount] = program.findNetworkAddress(network.name);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      let { slotIndex } = await provider.connection.getEpochInfo();
      const timestamp = await provider.connection.getBlockTime(slotIndex);

      await program.program.methods
        .deactivateReporter()
        .accounts({
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      let unlockDuration = network.stakeConfiguration.unlockDuration.toNumber();

      expect(
        fetchedReporterAccount.stake.eq(
          network.stakeConfiguration.publisherStake
        )
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
      expect(
        fetchedReporterAccount.unlockTimestamp.toNumber()
      ).toBeGreaterThanOrEqual(timestamp + unlockDuration);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.tracer;
      let network = NETWORKS[secondaryNetwork];
      const [networkAccount] = program.findNetworkAddress(network.name);

      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      let { slotIndex } = await provider.connection.getEpochInfo();
      const timestamp = await provider.connection.getBlockTime(slotIndex);

      await program.program.methods
        .deactivateReporter()
        .accounts({
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      let unlockDuration = network.stakeConfiguration.unlockDuration.toNumber();

      expect(
        fetchedReporterAccount.stake.eq(network.stakeConfiguration.tracerStake)
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
      expect(
        fetchedReporterAccount.unlockTimestamp.toNumber()
      ).toBeGreaterThanOrEqual(timestamp + unlockDuration);
    });
  });

  describe("unstake", () => {
    it("fail - reporter is not deacactivated", async () => {
      const reporter = REPORTERS.authority;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

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

      await expectThrowError(
        () =>
          program.program.methods
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
            .rpc(),
        programError("InvalidReporterStatus")
      );
    });

    it("fail - release epoch in future", async () => {
      const reporter = REPORTERS.tracer;
      const [networkAccount] = program.findNetworkAddress(secondaryNetwork);

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

      await expectThrowError(
        () =>
          program.program.methods
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
            .rpc(),
        programError("ReleaseEpochInFuture")
      );
    });

    it("success", async () => {
      const reporter = REPORTERS.publisher;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

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

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      const reporterBalanceAfter = await stakeToken.getBalance(
        reporter.keypair.publicKey
      );

      const networkBalanceAfter = await stakeToken.getBalance(
        networkAccount,
        true
      );

      expect(
        reporterBalanceAfter.eq(
          NETWORKS[mainNetwork].stakeConfiguration.publisherStake
        )
      ).toBeTruthy();

      expect(networkBalanceAfter.isZero()).toBeTruthy();

      expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
    });
  });
});
