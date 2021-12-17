import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  bufferFromString,
  Category,
  HapiCore,
  ReporterRole,
  ReporterStatus,
} from "../../lib";
import { pubkeyFromHex } from "../util/crypto";
import { errorRegexp, programError } from "../util/error";

describe("HapiCore Reporter", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  const community = web3.Keypair.generate();
  const otherCommunity = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  let currentEpoch: number;

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; role: keyof typeof ReporterRole }
  > = {
    alice: { name: "alice", keypair: web3.Keypair.generate(), role: "Full" },
    bob: { name: "bob", keypair: web3.Keypair.generate(), role: "Tracer" },
    carol: {
      name: "carol",
      keypair: web3.Keypair.generate(),
      role: "Authority",
    },
    dave: { name: "dave", keypair: web3.Keypair.generate(), role: "Tracer" },
  };

  const NETWORKS: Record<
    string,
    {
      name: string;
      rewardToken: TestToken;
      addressTracerReward: u64;
      addressConfirmationReward: u64;
      assetTracerReward: u64;
      assetConfirmationReward: u64;
    }
  > = {
    ethereum: {
      name: "ethereum",
      rewardToken: new TestToken(provider),
      addressTracerReward: new u64(1_000),
      addressConfirmationReward: new u64(2_000),
      assetTracerReward: new u64(3_000),
      assetConfirmationReward: new u64(4_000),
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
      pubkey: web3.PublicKey;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
    }
  > = {
    blackhole1: {
      pubkey: pubkeyFromHex(
        "0000000000000000000000000000000000000000000000000000000000000001"
      ),
      network: "ethereum",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
  };

  beforeAll(async () => {
    const wait: Promise<unknown>[] = [];

    const { epoch } = await provider.connection.getEpochInfo();
    currentEpoch = epoch;

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    wait.push(stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000)));

    rewardToken = new TestToken(provider);
    await rewardToken.mint(new u64(0));

    const tx = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: nobody.publicKey,
        lamports: 10_000_000,
      }),
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: REPORTERS.alice.keypair.publicKey,
        lamports: 10_000_000,
      }),
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: REPORTERS.bob.keypair.publicKey,
        lamports: 10_000_000,
      }),
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: REPORTERS.carol.keypair.publicKey,
        lamports: 10_000_000,
      }),
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: REPORTERS.dave.keypair.publicKey,
        lamports: 10_000_000,
      })
    );

    wait.push(provider.send(tx));

    for (const reporter of Object.keys(REPORTERS)) {
      wait.push(
        stakeToken.transfer(
          null,
          REPORTERS[reporter].keypair.publicKey,
          new u64(1_000_000)
        )
      );

      wait.push(
        rewardToken.getTokenAccount(REPORTERS[reporter].keypair.publicKey)
      );
    }

    const [tokenSignerAccount, tokenSignerBump] =
      await HapiCore.findCommunityTokenSignerAddress(community.publicKey);

    const communityTokenAccount = await stakeToken.createAccount(
      tokenSignerAccount
    );

    const [otherTokenSignerAccount, otherStashBump] =
      await HapiCore.findCommunityTokenSignerAddress(otherCommunity.publicKey);

    const otherTokenAccount = await stakeToken.createAccount(
      otherTokenSignerAccount
    );

    wait.push(
      HapiCore.rpc.initializeCommunity(
        new u64(0), // unlocks in current epoch
        1,
        new u64(20_000_000),
        new u64(2_000),
        new u64(3_000),
        new u64(5_000),
        tokenSignerBump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenSigner: tokenSignerAccount,
            tokenAccount: communityTokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [community],
        }
      ),
      HapiCore.rpc.initializeCommunity(
        new u64(10), // unlocks in the future
        2,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        otherStashBump,
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunity.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenSigner: otherTokenSignerAccount,
            tokenAccount: otherTokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [otherCommunity],
        }
      )
    );

    await Promise.all(wait);

    for (const key of Object.keys(NETWORKS)) {
      const network = NETWORKS[key];

      const [networkAccount, bump] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await HapiCore.findNetworkRewardSignerAddress(networkAccount);

      wait.push(
        HapiCore.rpc.createNetwork(
          bufferFromString(network.name, 32).toJSON().data,
          network.addressTracerReward,
          network.addressConfirmationReward,
          network.assetTracerReward,
          network.assetConfirmationReward,
          bump,
          rewardSignerBump,
          {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
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

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          HapiCore.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
              pubkey: reporter.keypair.publicKey,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /Cross-program invocation with unauthorized signer or writable account/
      );
    });

    it("success - alice, community 1", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await HapiCore.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(HapiCore.programId);
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - alice, community 2", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await HapiCore.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunity.publicKey,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(HapiCore.programId);
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await HapiCore.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(HapiCore.programId);
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - carol", async () => {
      const reporter = REPORTERS.carol;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await HapiCore.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(HapiCore.programId);
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      const tx = await HapiCore.rpc.createReporter(
        reporterRole,
        name.toJSON().data,
        bump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            reporter: reporterAccount,
            pubkey: reporter.keypair.publicKey,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.bump).toEqual(bump);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

      const reporterInfo = await provider.connection.getAccountInfoAndContext(
        reporterAccount
      );
      expect(reporterInfo.value.owner).toEqual(HapiCore.programId);
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("fail - reporter alice already exists", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          HapiCore.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          HapiCore.rpc.updateReporter(reporterRole, name.toJSON().data, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
          }),
        "3012: The program expected this account to be already initialized"
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString("ecila", 32);

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole.Validator;

      const tx = await HapiCore.rpc.updateReporter(
        reporterRole,
        name.toJSON().data,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            reporter: reporterAccount,
          },
        }
      );

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await HapiCore.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await HapiCore.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await HapiCore.account.reporterReward.fetch(
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
      expect(accountInfo.value.owner).toEqual(HapiCore.programId);
      expect(accountInfo.value.data).toHaveLength(105);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await HapiCore.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await HapiCore.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await HapiCore.account.reporterReward.fetch(
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
      expect(accountInfo.value.owner).toEqual(HapiCore.programId);
      expect(accountInfo.value.data).toHaveLength(105);
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [reporterRewardAccount, bump] =
        await HapiCore.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const tx = await HapiCore.rpc.initializeReporterReward(bump, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedAccount = await HapiCore.account.reporterReward.fetch(
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
      expect(accountInfo.value.owner).toEqual(HapiCore.programId);
      expect(accountInfo.value.data).toHaveLength(105);
    });
  });

  describe("activate_reporter", () => {
    it("fail - alice doesn't have enough tokens for a stake", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.activateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount: tokenAccount,
              communityTokenAccount: communityInfo.tokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        /Error processing Instruction 0: custom program error: 0x1/
      );
    });

    it("success - alice, community 2", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        otherCommunity.publicKey
      );

      const tx = await HapiCore.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: otherCommunity.publicKey,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenAccount: communityInfo.tokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      let stake: u64;
      if (reporter.role === "Validator") {
        stake = new u64(1_000);
      } else if (reporter.role === "Tracer") {
        stake = new u64(2_000);
      } else if (reporter.role === "Full") {
        stake = new u64(3_000);
      } else if (reporter.role === "Authority") {
        stake = new u64(4_000);
      } else {
        throw new Error("Invalid reporter role");
      }

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      const tx = await HapiCore.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenAccount: communityInfo.tokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      let stake: u64;
      if (reporter.role === "Validator") {
        stake = new u64(20_000_000);
      } else if (reporter.role === "Tracer") {
        stake = new u64(2_000);
      } else if (reporter.role === "Full") {
        stake = new u64(3_000);
      } else if (reporter.role === "Authority") {
        stake = new u64(5_000);
      } else {
        throw new Error("Invalid reporter role");
      }

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - carol", async () => {
      const reporter = REPORTERS.carol;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      const tx = await HapiCore.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenAccount: communityInfo.tokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      let stake: u64;
      if (reporter.role === "Validator") {
        stake = new u64(20_000_000);
      } else if (reporter.role === "Tracer") {
        stake = new u64(2_000);
      } else if (reporter.role === "Full") {
        stake = new u64(3_000);
      } else if (reporter.role === "Authority") {
        stake = new u64(5_000);
      } else {
        throw new Error("Invalid reporter role");
      }

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("success - dave", async () => {
      const reporter = REPORTERS.dave;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      const tx = await HapiCore.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenAccount: communityInfo.tokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

      let stake: u64;
      if (reporter.role === "Validator") {
        stake = new u64(20_000_000);
      } else if (reporter.role === "Tracer") {
        stake = new u64(2_000);
      } else if (reporter.role === "Full") {
        stake = new u64(3_000);
      } else if (reporter.role === "Authority") {
        stake = new u64(5_000);
      } else {
        throw new Error("Invalid reporter role");
      }

      const balance = await stakeToken.getBalance(reporter.keypair.publicKey);
      expect(balance.add(stake).toString(10)).toEqual("1000000");
    });

    it("fail - bob is already activated", async () => {
      const reporter = REPORTERS.bob;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.activateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount: tokenAccount,
              communityTokenAccount: communityInfo.tokenAccount,
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

        const [caseAccount, bump] = await HapiCore.findCaseAddress(
          community.publicKey,
          cs.caseId
        );

        const [reporterAccount] = await HapiCore.findReporterAddress(
          community.publicKey,
          reporter.publicKey
        );

        await HapiCore.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
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

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await HapiCore.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const [caseAccount] = await HapiCore.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      await HapiCore.rpc.createAddress(
        addr.pubkey,
        Category[addr.category],
        addr.risk,
        bump,
        {
          accounts: {
            sender: reporter.keypair.publicKey,
            address: addressAccount,
            community: community.publicKey,
            network: networkAccount,
            reporter: reporterAccount,
            case: caseAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter.keypair],
        }
      );
    });

    it("setup - confirm address by dave", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await HapiCore.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [reporterRewardAccount] = await HapiCore.findReporterRewardAddress(
        networkAccount,
        reporterAccount
      );

      const addressInfo = await HapiCore.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await HapiCore.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await HapiCore.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const tx = await HapiCore.rpc.confirmAddress({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: community.publicKey,
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await rewardToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [reporterRewardAccount] = await HapiCore.findReporterRewardAddress(
        networkAccount,
        reporterAccount
      );

      const [rewardSignerAccount] =
        await HapiCore.findNetworkRewardSignerAddress(networkAccount);

      const reporterBalanceBefore = new u64(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const tx = await HapiCore.rpc.claimReporterReward({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          reporterTokenAccount,
          rewardMint: rewardToken.mintAccount,
          rewardSigner: rewardSignerAccount,
          tokenProgram: rewardToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const reporterBalanceAfter = new u64(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      expect(
        reporterBalanceAfter.sub(reporterBalanceBefore).toNumber()
      ).toEqual(network.addressTracerReward.toNumber());

      const supply = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );
      expect(supply.value.amount).toEqual(
        network.addressTracerReward.toString()
      );

      {
        const fetchedAccount = await HapiCore.account.reporterReward.fetch(
          reporterRewardAccount
        );
        expect(fetchedAccount.addressConfirmationCounter.toNumber()).toEqual(0);
        expect(fetchedAccount.addressTracerCounter.toNumber()).toEqual(0);
      }
    });

    it("success - dave's claim", async () => {
      const reporter = REPORTERS.dave;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterTokenAccount = await rewardToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const network = NETWORKS.ethereum;

      const [networkAccount] = await HapiCore.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [reporterRewardAccount] = await HapiCore.findReporterRewardAddress(
        networkAccount,
        reporterAccount
      );

      const [rewardSignerAccount] =
        await HapiCore.findNetworkRewardSignerAddress(networkAccount);

      const reporterBalanceBefore = new u64(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      const tx = await HapiCore.rpc.claimReporterReward({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          reporterTokenAccount,
          rewardMint: rewardToken.mintAccount,
          rewardSigner: rewardSignerAccount,
          tokenProgram: rewardToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const reporterBalanceAfter = new u64(
        (
          await provider.connection.getTokenAccountBalance(reporterTokenAccount)
        ).value.amount,
        10
      );

      expect(
        reporterBalanceAfter.sub(reporterBalanceBefore).toNumber()
      ).toEqual(network.addressConfirmationReward.toNumber());

      const supply = await provider.connection.getTokenSupply(
        rewardToken.mintAccount
      );
      expect(supply.value.amount).toEqual(
        network.addressTracerReward
          .add(network.addressConfirmationReward)
          .toString()
      );

      {
        const fetchedAccount = await HapiCore.account.reporterReward.fetch(
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("success - alice in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await HapiCore.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: otherCommunity.publicKey,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await HapiCore.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.releaseReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount: tokenAccount,
              communityTokenSigner: communityInfo.tokenSigner,
              communityTokenAccount: communityInfo.tokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("InvalidReporterStatus")
      );
    });

    it("fail - alice is not ready to be released in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        otherCommunity.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.releaseReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: otherCommunity.publicKey,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount: tokenAccount,
              communityTokenSigner: communityInfo.tokenSigner,
              communityTokenAccount: communityInfo.tokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          }),
        programError("ReleaseEpochInFuture")
      );
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await HapiCore.account.community.fetch(
        community.publicKey
      );

      const reporterBalanceBefore = new u64(
        (
          await provider.connection.getTokenAccountBalance(tokenAccount)
        ).value.amount,
        10
      );

      const communityBalanceBefore = new u64(
        (
          await provider.connection.getTokenAccountBalance(
            communityInfo.tokenAccount
          )
        ).value.amount,
        10
      );

      const tx = await HapiCore.rpc.releaseReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenSigner: communityInfo.tokenSigner,
          communityTokenAccount: communityInfo.tokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
      expect(fetchedReporterAccount.unlockEpoch.toNumber()).toEqual(0);

      const reporterBalanceAfter = new u64(
        (
          await provider.connection.getTokenAccountBalance(tokenAccount)
        ).value.amount,
        10
      );

      const communityBalanceAfter = new u64(
        (
          await provider.connection.getTokenAccountBalance(
            communityInfo.tokenAccount
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          HapiCore.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await HapiCore.rpc.freezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
      expect(fetchedReporterAccount.isFrozen).toBe(true);

      // Reporter shouldn't be able to report when it's frozen
      {
        const caseId = new BN(1);
        const caseName = bufferFromString("Case 1", 32);

        const [caseAccount, bump] = await HapiCore.findCaseAddress(
          community.publicKey,
          caseId
        );

        await expectThrowError(
          () =>
            HapiCore.rpc.createCase(caseId, caseName.toJSON().data, bump, {
              accounts: {
                reporter: reporterAccount,
                sender: reporter.keypair.publicKey,
                community: community.publicKey,
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
            HapiCore.rpc.deactivateReporter({
              accounts: {
                sender: reporter.keypair.publicKey,
                community: community.publicKey,
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

      const [reporterAccount] = await HapiCore.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await HapiCore.rpc.unfreezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedReporterAccount = await HapiCore.account.reporter.fetch(
        reporterAccount
      );
      expect(fetchedReporterAccount.role).toEqual(ReporterRole[reporter.role]);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
      expect(fetchedReporterAccount.isFrozen).toBe(false);
    });
  });
});