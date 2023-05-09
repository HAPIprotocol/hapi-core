/* eslint-disable no-prototype-builtins */
import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  ACCOUNT_SIZE,
  bufferFromString,
  Category,
  initHapiCore,
  NetworkSchema,
  NetworkSchemaKeys,
  ReporterRole,
  ReporterStatus,
} from "../../lib";
import { errorRegexp, programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";

describe("HapiCore Reporter", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const communityId = new BN(10);
  const otherCommunityId = new BN(11);

  let currentEpoch: number;

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; role: keyof typeof ReporterRole }
  > = {
    alice: {
      name: "alice",
      keypair: web3.Keypair.generate(),
      role: "Publisher",
    },
    bob: { name: "bob", keypair: web3.Keypair.generate(), role: "Tracer" },
    carol: {
      name: "carol",
      keypair: web3.Keypair.generate(),
      role: "Authority",
    },
    dave: { name: "dave", keypair: web3.Keypair.generate(), role: "Publisher" },
    erin: { name: "erin", keypair: web3.Keypair.generate(), role: "Appraiser" },
  };

  const NETWORKS: Record<
    string,
    {
      name: string;
      schema: NetworkSchemaKeys;
      rewardToken: TestToken;
      addressTracerReward: BN;
      addressConfirmationReward: BN;
      assetTracerReward: BN;
      assetConfirmationReward: BN;
      reportPrice: BN;
    }
  > = {
    ethereum: {
      name: "ethereum",
      schema: "Ethereum",
      rewardToken: new TestToken(provider),
      addressTracerReward: new BN(1_000),
      addressConfirmationReward: new BN(2_000),
      assetTracerReward: new BN(3_000),
      assetConfirmationReward: new BN(4_000),
      reportPrice: new BN(1_000),
    },
  };

  const CASES: Record<
    string,
    {
      network: keyof typeof NETWORKS;
      caseId: BN;
      name: string;
      reporter: keyof typeof REPORTERS;
    }
  > = {
    safe: {
      network: "ethereum",
      caseId: new BN(1),
      name: "safe network addresses",
      reporter: "carol",
    },
  };

  const ADDRESSES: Record<
    string,
    {
      pubkey: Buffer;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
    }
  > = {
    blackhole1: {
      pubkey: Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000000001",
        "hex"
      ),
      network: "ethereum",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
  };

  const reporterStake = {
    Validator: 1_000,
    Tracer: 2_000,
    Publisher: 3_000,
    Authority: 4_000,
    Appraiser: 5_000,
  };

  beforeAll(async () => {
    const wait: Promise<unknown>[] = [];

    const { epoch } = await provider.connection.getEpochInfo();
    currentEpoch = epoch;

    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);
    wait.push(stakeToken.transfer(null, nobody.publicKey, 1_000_000));

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

    const tx = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: nobody.publicKey,
        lamports: 10_000_000,
      }),
      ...Object.keys(REPORTERS).map((key) =>
        web3.SystemProgram.transfer({
          fromPubkey: authority.publicKey,
          toPubkey: REPORTERS[key].keypair.publicKey,
          lamports: 10_000_000,
        })
      )
    );

    wait.push(provider.sendAndConfirm(tx));

    for (const reporter of Object.keys(REPORTERS)) {
      wait.push(
        stakeToken.transfer(
          null,
          REPORTERS[reporter].keypair.publicKey,
          1_000_000
        )
      );

      wait.push(
        rewardToken.transfer(
          null,
          REPORTERS[reporter].keypair.publicKey,
          1_000_000
        )
      );
    }

    const [communityAccount, communityBump] =
      await program.pda.findCommunityAddress(communityId);

    const [otherCommunityAccount, otherCommunityBump] =
      await program.pda.findCommunityAddress(otherCommunityId);

    const communityTokenAccount = await stakeToken.getTokenAccount(
      communityAccount,
      true
    );

    const otherTokenAccount = await stakeToken.getTokenAccount(
      otherCommunityAccount,
      true
    );

    wait.push(
      program.rpc.initializeCommunity(
        communityId,
        communityBump,
        new BN(0), // unlocks in current epoch
        1,
        new BN(20_000_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            stakeMint: stakeToken.mintAccount,
            tokenAccount: communityTokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      ),
      program.rpc.initializeCommunity(
        otherCommunityId,
        otherCommunityBump,
        new BN(10), // unlocks in the future
        2,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunityAccount,
            stakeMint: stakeToken.mintAccount,
            tokenAccount: otherTokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      )
    );

    await Promise.all(wait);

    for (const key of Object.keys(NETWORKS)) {
      const network = NETWORKS[key];

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount,
        true
      );

      wait.push(
        program.rpc.createNetwork(
          bufferFromString(network.name, 32).toJSON().data,
          NetworkSchema[network.schema],
          network.addressTracerReward,
          network.addressConfirmationReward,
          network.assetTracerReward,
          network.assetConfirmationReward,
          bump,
          network.reportPrice,
          {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              treasuryTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }
        )
      );
    }

    await Promise.all(wait);
  });

  describe("create_reporter", () => {
    it("fail - invalid authority", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
            accounts: {
              authority: nobody.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
              pubkey: reporter.keypair.publicKey,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });

    it("success - alice, community 1", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("success - alice, community 2", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [otherCommunityAccount] = await program.pda.findCommunityAddress(
        otherCommunityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        otherCommunityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("success - carol", async () => {
      const reporter = REPORTERS.carol;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("success - erin", async () => {
      const reporter = REPORTERS.erin;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await program.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(program.programId);
      expect(reporterInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporter);
    });

    it("fail - reporter alice already exists", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
              pubkey: reporter.keypair.publicKey,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /custom program error: 0x0/
      );
    });
  });

  describe("update_reporter", () => {
    it("fail - reporter doesn't exist", async () => {
      const reporter = {
        name: "nobody",
        keypair: nobody,
        role: "Validator",
      };

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.updateReporter(reporterRole, name.toJSON().data, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
            },
          }),
        "AnchorError caused by account: reporter. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString("ecila", 32);

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole.Validator;

      const tx = await program.rpc.updateReporter(
        reporterRole,
        name.toJSON().data,
        {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            reporter: reporterAccount,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );

      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole.Validator);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
    });
  });

  describe("initialize_reporter_reward", () => {
    it("success - alice", async () => {
      const reporter = REPORTERS.alice;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await program.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await program.account.reporterReward.fetch(
        reporterRewardAccount
      );
      expect(fetchedAccount.bump).toEqual(bump);
      expect(fetchedAccount.network).toEqual(networkAccount);
      expect(fetchedAccount.reporter).toEqual(reporterAccount);
      expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);

      const accountInfo = await provider.connection.getAccountInfoAndContext(
        reporterRewardAccount
      );
      expect(accountInfo.value.owner).toEqual(program.programId);
      expect(accountInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporterReward);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await program.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await program.account.reporterReward.fetch(
        reporterRewardAccount
      );
      expect(fetchedAccount.bump).toEqual(bump);
      expect(fetchedAccount.network).toEqual(networkAccount);
      expect(fetchedAccount.reporter).toEqual(reporterAccount);
      expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);

      const accountInfo = await provider.connection.getAccountInfoAndContext(
        reporterRewardAccount
      );
      expect(accountInfo.value.owner).toEqual(program.programId);
      expect(accountInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporterReward);
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await program.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await program.account.reporterReward.fetch(
        reporterRewardAccount
      );
      expect(fetchedAccount.bump).toEqual(bump);
      expect(fetchedAccount.network).toEqual(networkAccount);
      expect(fetchedAccount.reporter).toEqual(reporterAccount);
      expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);

      const accountInfo = await provider.connection.getAccountInfoAndContext(
        reporterRewardAccount
      );
      expect(accountInfo.value.owner).toEqual(program.programId);
      expect(accountInfo.value.data).toHaveLength(ACCOUNT_SIZE.reporterReward);
    });
  });

  describe("activate_reporter", () => {
    it("fail - alice doesn't have enough tokens for a stake", async () => {
      const reporter = REPORTERS.alice;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      await expectThrowError(
        () =>
          program.rpc.activateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount,
              communityTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        /Error processing Instruction 0: custom program error: 0x1/
      );
    });

    it("success - alice, community 2", async () => {
      const reporter = REPORTERS.alice;

      const [otherCommunityAccount] = await program.pda.findCommunityAddress(
        otherCommunityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        otherCommunityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        otherCommunityAccount,
        true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: otherCommunityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      if (!reporterStake.hasOwnProperty(reporter.role))
        throw new Error("Invalid reporter role");

      const stake = new BN(reporterStake[reporter.role]);

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      if (!reporterStake.hasOwnProperty(reporter.role))
        throw new Error("Invalid reporter role");

      const stake = new BN(reporterStake[reporter.role]);

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - carol", async () => {
      const reporter = REPORTERS.carol;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      if (!reporterStake.hasOwnProperty(reporter.role))
        throw new Error("Invalid reporter role");

      const stake = new BN(reporterStake[reporter.role]);

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      if (!reporterStake.hasOwnProperty(reporter.role))
        throw new Error("Invalid reporter role");

      const stake = new BN(reporterStake[reporter.role]);

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - erin", async () => {
      const reporter = REPORTERS.erin;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      if (!reporterStake.hasOwnProperty(reporter.role))
        throw new Error("Invalid reporter role");

      const stake = new BN(reporterStake[reporter.role]);

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("fail - bob is already activated", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      await expectThrowError(
        () =>
          program.rpc.activateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount,
              communityTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });
  });

  describe("claim_reporter_reward", () => {
    it("setup - create cases", async () => {
      for (const key of Object.keys(CASES)) {
        const cs = CASES[key];

        const reporter = REPORTERS[cs.reporter].keypair;

        const caseName = bufferFromString(cs.name, 32);

        const [communityAccount] = await program.pda.findCommunityAddress(
          communityId
        );

        const [caseAccount, bump] = await program.pda.findCaseAddress(
          communityAccount,
          cs.caseId
        );

        const [reporterAccount] = await program.pda.findReporterAddress(
          communityAccount,
          reporter.publicKey
        );

        await program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: communityAccount,
            case: caseAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        });
      }
    });

    it("setup - create address", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        communityAccount,
        addr.caseId
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount,
        true
      );

      const reporterPaymentTokenAccount = await rewardToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      await program.rpc.createAddress(
        [...addr.pubkey],
        Category[addr.category],
        addr.risk,
        bump,
        {
          accounts: {
            sender: reporter.keypair.publicKey,
            address: addressAccount,
            community: communityAccount,
            network: networkAccount,
            reporter: reporterAccount,
            case: caseAccount,
            reporterPaymentTokenAccount,
            treasuryTokenAccount,
            tokenProgram: stakeToken.programId,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter.keypair],
        }
      );
    });

    it("setup - confirm address by dave", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.dave.keypair;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.publicKey
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const addressInfo = await program.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        communityAccount,
        addr.caseId
      );

      const tx = await program.rpc.confirmAddress({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          addressReporterReward: addressReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();
    });

    it("success - bob's claim", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await rewardToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const supplyBefore = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );

      const tx = await program.rpc.claimReporterReward({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          reporterTokenAccount,
          rewardMint: rewardToken.mintAccount,
          tokenProgram: rewardToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const reporterBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      expect(
        reporterBalanceAfter.sub(reporterBalanceBefore).toNumber()
      ).toEqual(network.addressTracerReward.toNumber());

      const supplyAfter = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );

      expect(new BN(supplyAfter.value.amount)).toEqual(
        new BN(supplyBefore.value.amount).add(network.addressTracerReward)
      );

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );
        expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);
        expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      }
    });

    it("success - dave's claim", async () => {
      const reporter = REPORTERS.dave;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await rewardToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        network.name
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const supplyBefore = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );

      const tx = await program.rpc.claimReporterReward({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          reporterTokenAccount,
          rewardMint: rewardToken.mintAccount,
          tokenProgram: rewardToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const reporterBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      expect(
        reporterBalanceAfter.sub(reporterBalanceBefore).toNumber()
      ).toEqual(network.addressConfirmationReward.toNumber());

      const supplyAfter = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );

      expect(new BN(supplyAfter.value.amount).toNumber()).toEqual(
        new BN(supplyBefore.value.amount)
          .add(network.addressConfirmationReward)
          .toNumber()
      );

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );
        expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);
        expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      }
    });
  });

  describe("deactivate_reporter", () => {
    it("fail - alice is not activated in community 1", async () => {
      const reporter = REPORTERS.alice;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("success - alice in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [otherCommunityAccount] = await program.pda.findCommunityAddress(
        otherCommunityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        otherCommunityAccount,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: otherCommunityAccount,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
      expect(fetchedReporterAccount.unlockEpoch.toNumber()).toEqual(
        currentEpoch + 10
      );
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
      expect(fetchedReporterAccount.unlockEpoch.toNumber()).toEqual(
        currentEpoch
      );
    });

    it("fail - bob is already deactivated", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });
  });

  describe("release_reporter", () => {
    it("fail - alice is not deactivated", async () => {
      const reporter = REPORTERS.alice;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      await expectThrowError(
        () =>
          program.rpc.releaseReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount,
              communityTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("fail - alice is not ready to be released in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [otherCommunityAccount] = await program.pda.findCommunityAddress(
        otherCommunityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        otherCommunityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        otherCommunityAccount,
        true
      );

      await expectThrowError(
        () =>
          program.rpc.releaseReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: otherCommunityAccount,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount,
              communityTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("ReleaseEpochInFuture")
      );
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount,
        true
      );

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const communityBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(
            communityTokenAccount
          )
        ).value.amount,
        10
      );

      const tx = await program.rpc.releaseReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockEpoch.toNumber()).toEqual(0);

      const reporterBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const communityBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(
            communityTokenAccount
          )
        ).value.amount,
        10
      );

      // Expect bob to get his 2_000 stake back
      expect(
        reporterBalanceAfter.sub(reporterBalanceBefore).toNumber()
      ).toEqual(2_000);

      // Expect community to return 2_000 of stake back to bob
      expect(
        communityBalanceBefore.sub(communityBalanceAfter).toNumber()
      ).toEqual(2_000);
    });

    it("fail - bob is inactive", async () => {
      const reporter = REPORTERS.bob;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });
  });

  describe("freeze_reporter", () => {
    it("success", async () => {
      const reporter = REPORTERS.carol;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.freezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
      expect(fetchedReporterAccount.isFrozen).toBe(true);

      // Reporter shouldn't be able to report when it's frozen
      {
        const caseId = new BN(1);
        const caseName = bufferFromString("Case 1", 32);

        const [caseAccount, bump] = await program.pda.findCaseAddress(
          communityAccount,
          caseId
        );

        await expectThrowError(
          () =>
            program.rpc.createCase(caseId, caseName.toJSON().data, bump, {
              accounts: {
                reporter: reporterAccount,
                sender: reporter.keypair.publicKey,
                community: communityAccount,
                case: caseAccount,
                systemProgram: web3.SystemProgram.programId,
              },

              signers: [reporter.keypair],
            }),
          errorRegexp(0)
        );
      }

      {
        await expectThrowError(
          () =>
            program.rpc.deactivateReporter({
              accounts: {
                sender: reporter.keypair.publicKey,
                community: communityAccount,
                reporter: reporterAccount,
              },
              signers: [reporter.keypair],
            }),
          programError("FrozenReporter")
        );
      }
    });
  });

  describe("unfreeze_reporter", () => {
    it("success", async () => {
      const reporter = REPORTERS.carol;

      const [communityAccount] = await program.pda.findCommunityAddress(
        communityId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.unfreezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
      expect(fetchedReporterAccount.isFrozen).toBe(false);
    });
  });
});
