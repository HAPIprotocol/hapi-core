import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  bufferFromString,
  program,
  ReporterRole,
  ReporterStatus,
} from "../../lib";

jest.setTimeout(10_000);

describe("HapiCore Reporter", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  const community = web3.Keypair.generate();
  const otherCommunity = web3.Keypair.generate();
  let stakeToken: TestToken;
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
  };

  beforeAll(async () => {
    const wait: Promise<unknown>[] = [];

    const { epoch } = await provider.connection.getEpochInfo();
    currentEpoch = epoch;

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    wait.push(stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000)));

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
    }

    const [tokenSignerAccount, tokenSignerBump] =
      await program.findCommunityTokenSignerAddress(community.publicKey);

    const communityTokenAccount = await stakeToken.createAccount(
      tokenSignerAccount
    );

    const [otherTokenSignerAccount, otherStashBump] =
      await program.findCommunityTokenSignerAddress(otherCommunity.publicKey);

    const otherTokenAccount = await stakeToken.createAccount(
      otherTokenSignerAccount
    );

    wait.push(
      program.rpc.initializeCommunity(
        new u64(0), // unlocks in current epoch
        2,
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
      program.rpc.initializeCommunity(
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
  });

  describe("create_reporter", () => {
    it("fail - invalid authority", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
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

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
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
            community: community.publicKey,
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
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - alice, community 2", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await program.findReporterAddress(
        otherCommunity.publicKey,
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
            community: otherCommunity.publicKey,
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
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
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
            community: community.publicKey,
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
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("success - carol", async () => {
      const reporter = REPORTERS.carol;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
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
            community: community.publicKey,
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
      expect(reporterInfo.value.data).toHaveLength(200);
    });

    it("fail - reporter alice already exists", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString(reporter.name, 32);

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.createReporter(reporterRole, name.toJSON().data, bump, {
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole[reporter.role];

      await expectThrowError(
        () =>
          program.rpc.updateReporter(reporterRole, name.toJSON().data, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
          }),
        "167: The given account is not owned by the executing program"
      );
    });

    it("success - alice", async () => {
      const reporter = REPORTERS.alice;

      const name = bufferFromString("ecila", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const reporterRole = ReporterRole.Validator;

      const tx = await program.rpc.updateReporter(
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

      const fetchedReporterAccount = await program.account.reporter.fetch(
        reporterAccount
      );
      expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
      expect(fetchedReporterAccount.role).toEqual(ReporterRole.Validator);
      expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
    });
  });

  describe("activate_reporter", () => {
    it("fail - alice doesn't have enough tokens for a stake", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.activateReporter({
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

      const [reporterAccount] = await program.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        otherCommunity.publicKey
      );

      const tx = await program.rpc.activateReporter({
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

      const fetchedReporterAccount = await program.account.reporter.fetch(
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const tx = await program.rpc.activateReporter({
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

      const fetchedReporterAccount = await program.account.reporter.fetch(
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const tx = await program.rpc.activateReporter({
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

      const fetchedReporterAccount = await program.account.reporter.fetch(
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.activateReporter({
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
        "309: Invalid reporter status"
      );
    });
  });

  describe("deactivate_reporter", () => {
    it("fail - alice is not activated in community 1", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        "309: Invalid reporter status"
      );
    });

    it("success - alice in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await program.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: otherCommunity.publicKey,
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.deactivateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: community.publicKey,
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        "309: Invalid reporter status"
      );
    });
  });

  describe("release_reporter", () => {
    it("fail - alice is not deactivated", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.releaseReporter({
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
        "309: Invalid reporter status"
      );
    });

    it("fail - alice is not ready to be released in community 2", async () => {
      const reporter = REPORTERS.alice;

      const [reporterAccount] = await program.findReporterAddress(
        otherCommunity.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
        otherCommunity.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.releaseReporter({
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
        "303: Release epoch is in future"
      );
    });

    it("success - bob", async () => {
      const reporter = REPORTERS.bob;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityInfo = await program.account.community.fetch(
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

      const tx = await program.rpc.releaseReporter({
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

      const fetchedReporterAccount = await program.account.reporter.fetch(
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

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.deactivateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        "309: Invalid reporter status"
      );
    });
  });

  describe("freeze_reporter", () => {
    it("success", async () => {
      const reporter = REPORTERS.carol;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.freezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
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

        const [caseAccount, bump] = await program.findCaseAddress(
          community.publicKey,
          caseId
        );

        await expectThrowError(
          () =>
            program.rpc.createCase(caseId, caseName.toJSON().data, bump, {
              accounts: {
                reporter: reporterAccount,
                sender: reporter.keypair.publicKey,
                community: community.publicKey,
                case: caseAccount,
                systemProgram: web3.SystemProgram.programId,
              },

              signers: [reporter.keypair],
            }),
          "312: This reporter is frozen"
        );
      }

      {
        await expectThrowError(
          () =>
            program.rpc.deactivateReporter({
              accounts: {
                sender: reporter.keypair.publicKey,
                community: community.publicKey,
                reporter: reporterAccount,
              },
              signers: [reporter.keypair],
            }),
          "312: This reporter is frozen"
        );
      }
    });
  });

  describe("unfreeze_reporter", () => {
    it("success", async () => {
      const reporter = REPORTERS.carol;

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      const tx = await program.rpc.unfreezeReporter({
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
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
