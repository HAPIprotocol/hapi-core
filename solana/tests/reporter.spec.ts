import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import { getReporters, getNetwotks, setupNetworks } from "./util/setup";

import {
  ACCOUNT_SIZE,
  ReporterRole,
  HapiCoreProgram,
  ReporterStatus,
} from "../lib";

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
  let NETWORKS = getNetwotks([mainNetwork, secondaryNetwork]);

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

      const [networkAccount, _] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.alice;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.id,
        reporter.keypair.publicKey,
        networkName,
        reporterRole,
        reporter.url,
        bump,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.createReporter(...args, {
            accounts: {
              authority: another_authority.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [another_authority],
          }),
        programError("AuthorityMismatch")
      );
    });

    it("success - alice", async () => {
      const networkName = mainNetwork;

      const [networkAccount, _] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.alice;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.id,
        reporter.keypair.publicKey,
        networkName,
        reporterRole,
        reporter.url,
        bump,
      ];

      await program.program.rpc.createReporter(...args, {
        accounts: {
          authority: authority.publicKey,
          reporter: reporterAccount,
          network: networkAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
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

      const [networkAccount, _] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.bob;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.id,
        reporter.keypair.publicKey,
        networkName,
        reporterRole,
        reporter.url,
        bump,
      ];

      await program.program.rpc.createReporter(...args, {
        accounts: {
          authority: authority.publicKey,
          reporter: reporterAccount,
          network: networkAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
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

      const [networkAccount, _] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.carol;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.id,
        reporter.keypair.publicKey,
        networkName,
        reporterRole,
        reporter.url,
        bump,
      ];

      await program.program.rpc.createReporter(...args, {
        accounts: {
          authority: authority.publicKey,
          reporter: reporterAccount,
          network: networkAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
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

      const [networkAccount, _] = program.findNetworkAddress(networkName);

      const reporter = REPORTERS.alice;
      const [reporterAccount, bump] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.id,
        reporter.keypair.publicKey,
        networkName,
        reporterRole,
        reporter.url,
        bump,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.createReporter(...args, {
            accounts: {
              authority: authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /custom program error: 0x0/
      );
    });
  });

  describe("update_reporter", () => {
    it("fail - authority mismatch", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.keypair.publicKey,
        reporter.name,
        reporterRole,
        reporter.url,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.updateReporter(...args, {
            accounts: {
              authority: another_authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            },
            signers: [another_authority],
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - reporter does not exists", async () => {
      const reporter = REPORTERS.dave;
      const networkName = mainNetwork;

      const networkAccount = program.findNetworkAddress(networkName)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.keypair.publicKey,
        reporter.name,
        reporterRole,
        reporter.url,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.updateReporter(...args, {
            accounts: {
              authority: authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            },
          }),
        /The program expected this account to be already initialized/
      );
    });

    it("fail - network mismatch", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(secondaryNetwork)[0];

      const reporterNetworkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        reporterNetworkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.keypair.publicKey,
        reporter.name,
        reporterRole,
        reporter.url,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.updateReporter(...args, {
            accounts: {
              authority: authority.publicKey,
              reporter: reporterAccount,
              network: networkAccount,
            },
          }),
        /A seeds constraint was violated/
      );
    });

    it("success", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterRole = ReporterRole[reporter.role];

      const args = [
        reporter.keypair.publicKey,
        reporter.name,
        reporterRole,
        reporter.url,
      ];

      await program.program.rpc.updateReporter(...args, {
        accounts: {
          authority: authority.publicKey,
          reporter: reporterAccount,
          network: networkAccount,
        },
      });
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
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(secondaryNetwork)[0];
      const reporterNetworkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        /A seeds constraint was violated/
      );
    });

    it("fail - invalid reporter", async () => {
      const reporter = REPORTERS.alice;
      const anotherReporter = REPORTERS.bob;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.activateReporter({
            accounts: {
              signer: anotherReporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [anotherReporter.keypair],
          }),
        programError("InvalidReporter")
      );
    });

    it("fail - invalid network ATA mint", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount: invalidNetworkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidToken")
      );
    });

    it("fail - invalid reporter ATA mint", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount: invalidReporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidToken")
      );
    });

    it("fail - invalid network ATA owner", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount: reporterStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("IllegalOwner")
      );
    });

    it("fail - invalid reporter ATA owner", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const networkStakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      await expectThrowError(
        () =>
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount: networkStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("IllegalOwner")
      );
    });

    it("fail - insufficient funds", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        /Error processing Instruction 0: custom program error: 0x1/
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.alice;
      let network = NETWORKS[mainNetwork];

      const networkAccount = program.findNetworkAddress(network.name)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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

      await program.program.rpc.activateReporter({
        accounts: {
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          networkStakeTokenAccount,
          reporterStakeTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(
        fetchedReporterAccount.stake.eq(
          network.stakeConfiguration.publisherStake
        )
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;
      let network = NETWORKS[secondaryNetwork];

      const networkAccount = program.findNetworkAddress(network.name)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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

      await program.program.rpc.activateReporter({
        accounts: {
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          networkStakeTokenAccount,
          reporterStakeTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(
        fetchedReporterAccount.stake.eq(network.stakeConfiguration.tracerStake)
      ).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
    });

    it("fail - reporter is already activated", async () => {
      const reporter = REPORTERS.alice;
      let network = NETWORKS[mainNetwork];
      const networkAccount = program.findNetworkAddress(network.name)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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

      await expectThrowError(
        () =>
          program.program.rpc.activateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });
  });

  describe("dectivate_reporter", () => {
    it("fail - reporter is not activated", async () => {
      const reporter = REPORTERS.carol;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.rpc.deactivateReporter({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.alice;
      let network = NETWORKS[mainNetwork];
      const networkAccount = program.findNetworkAddress(network.name)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      let { slotIndex } = await provider.connection.getEpochInfo();
      const timestamp = await provider.connection.getBlockTime(slotIndex);

      await program.program.rpc.deactivateReporter({
        accounts: {
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

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
      const reporter = REPORTERS.bob;
      let network = NETWORKS[secondaryNetwork];
      const networkAccount = program.findNetworkAddress(network.name)[0];

      const [reporterAccount, _] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      let { slotIndex } = await provider.connection.getEpochInfo();
      const timestamp = await provider.connection.getBlockTime(slotIndex);

      await program.program.rpc.deactivateReporter({
        accounts: {
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

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
      const reporter = REPORTERS.carol;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.unstake({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("fail - release epoch in future", async () => {
      const reporter = REPORTERS.bob;
      const networkAccount = program.findNetworkAddress(secondaryNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
          program.program.rpc.unstake({
            accounts: {
              signer: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              networkStakeTokenAccount,
              reporterStakeTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("ReleaseEpochInFuture")
      );
    });

    it("success", async () => {
      const reporter = REPORTERS.alice;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      const [reporterAccount, _] = program.findReporterAddress(
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
      await program.program.rpc.unstake({
        accounts: {
          signer: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          networkStakeTokenAccount,
          reporterStakeTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      const fetchedReporterAccount =
        await program.program.account.reporter.fetch(reporterAccount);

      expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
    });
  });
});
