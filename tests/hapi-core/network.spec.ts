import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  ACCOUNT_SIZE,
  bufferFromString,
  initHapiCore,
  NetworkSchema,
  ReporterRole,
  ReporterStatus
} from "../../lib";
import { programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";

describe("HapiCore Network", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const addressTracerReward = new BN(1_000);
  const addressConfirmationReward = new BN(2_000);
  const assetTracerReward = new BN(3_000);
  const assetConfirmationReward = new BN(4_000);
  const appraiserStake = new BN(5_000);
  const reportPrice = new BN(1_000);

  const communityId = new BN(8);
  const otherCommunityId = new BN(9);

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
    erin: { name: "erin", keypair: web3.Keypair.generate(), role: "Appraiser" }
  };

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);
    await stakeToken.transfer(null, nobody.publicKey, 1_000_000);

    await provider.connection.requestAirdrop(nobody.publicKey, 1000000000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(0);

    const [communityAccount, communityBump] =
      await program.pda.findCommunityAddress(
        communityId
      );

    const tokenAccount = await stakeToken.getTokenAccount(
      communityAccount, true
    );

    await program.rpc.initializeCommunity(
      communityId,
      communityBump,
      new BN(1),
      2,
      addressTracerReward,
      addressConfirmationReward,
      assetTracerReward,
      assetConfirmationReward,
      appraiserStake,
      {
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          stakeMint: stakeToken.mintAccount,
          tokenAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      }
    );

  });

  describe("create_network", () => {
    it("fail - invalid authority", async () => {
      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [networkAccount, networkBump] =
        await program.pda.findNetworkAddress(communityAccount, "near");

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount, true
      );

      const args = [
        name.toJSON().data,
        schema,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        networkBump,
        reportPrice
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: communityAccount,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              treasuryTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - authority mismatch for community", async () => {
      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [otherCommunityAccount, otherCommunityBump] =
        await program.pda.findCommunityAddress(
          otherCommunityId
        );

      const otherTokenAccount = await stakeToken.getTokenAccount(
        otherCommunityAccount, true
      );

      await program.rpc.initializeCommunity(
        otherCommunityId,
        otherCommunityBump,
        new BN(1),
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
      );

      await program.rpc.setCommunityAuthority({
        accounts: {
          authority: authority.publicKey,
          community: otherCommunityAccount,
          newAuthority: nobody.publicKey,
        },
      });

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        otherCommunityAccount,
        "near"
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount, true
      );

      const args = [
        name.toJSON().data,
        schema,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        reportPrice
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunityAccount,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              treasuryTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - community mismatch for network", async () => {
      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [otherCommunityAccount, __] =
        await program.pda.findCommunityAddress(
          otherCommunityId
        );

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount, true
      );

      const args = [
        name.toJSON().data,
        schema,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        reportPrice
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunityAccount,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              treasuryTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /(A seeds constraint was violated)/
      );
    });

    it("success", async () => {
      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount, true
      );

      const args = [
        name.toJSON().data,
        schema,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        reportPrice
      ];

      const tx = await program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          network: networkAccount,
          rewardMint: rewardToken.mintAccount,
          treasuryTokenAccount,
          tokenProgram: rewardToken.programId,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );
      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.schema).toEqual(NetworkSchema.Near);
      expect(fetchedNetworkAccount.bump).toEqual(bump);
      expect(fetchedNetworkAccount.addressTracerReward.toNumber()).toEqual(
        addressTracerReward.toNumber()
      );
      expect(
        fetchedNetworkAccount.addressConfirmationReward.toNumber()
      ).toEqual(addressConfirmationReward.toNumber());
      expect(fetchedNetworkAccount.assetTracerReward.toNumber()).toEqual(
        assetTracerReward.toNumber()
      );
      expect(fetchedNetworkAccount.assetConfirmationReward.toNumber()).toEqual(
        assetConfirmationReward.toNumber()
      );
      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.replicationPrice.eq(reportPrice)).toBeTruthy();

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data).toHaveLength(ACCOUNT_SIZE.network);
    });

    it("fail - network already exists", async () => {
      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const treasuryTokenAccount = await rewardToken.getTokenAccount(
        networkAccount, true
      );

      const args = [
        name.toJSON().data,
        schema,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        reportPrice
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              treasuryTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /failed to send transaction/
      );
    });
  });

  describe("update_network", () => {
    it("fail - authority mismatch for community", async () => {

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [otherCommunityAccount, __] =
        await program.pda.findCommunityAddress(
          otherCommunityId
        );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const args = [
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
      ];

      await expectThrowError(
        () =>
          program.rpc.updateNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunityAccount,
              network: networkAccount,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - network does not exist", async () => {
      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "unknown"
      );

      const args = [
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
      ];

      await expectThrowError(
        () =>
          program.rpc.updateNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              network: networkAccount,
            },
          }),
        "AnchorError caused by account: network. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success", async () => {
      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const args = [
        addressTracerReward.addn(1),
        addressConfirmationReward.addn(1),
        assetTracerReward.addn(1),
        assetConfirmationReward.addn(1),
      ];

      const tx = await program.rpc.updateNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          network: networkAccount,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );
      expect(fetchedNetworkAccount.addressTracerReward.toNumber()).toEqual(
        addressTracerReward.addn(1).toNumber()
      );
      expect(
        fetchedNetworkAccount.addressConfirmationReward.toNumber()
      ).toEqual(addressConfirmationReward.addn(1).toNumber());
      expect(fetchedNetworkAccount.assetTracerReward.toNumber()).toEqual(
        assetTracerReward.addn(1).toNumber()
      );
      expect(fetchedNetworkAccount.assetConfirmationReward.toNumber()).toEqual(
        assetConfirmationReward.addn(1).toNumber()
      );
    });
  });

  describe("update_replication_price", () => {

    beforeAll(async () => {
      const wait: Promise<unknown>[] = [];

      const tx = new web3.Transaction().add(
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
        }),
        web3.SystemProgram.transfer({
          fromPubkey: authority.publicKey,
          toPubkey: REPORTERS.erin.keypair.publicKey,
          lamports: 10_000_000,
        })
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
          rewardToken.getTokenAccount(REPORTERS[reporter].keypair.publicKey)
        );
      }

      await Promise.all(wait);
    });

    it.each(Object.keys(REPORTERS))("Setup - reporter %s is created", async (key) => {
      const reporter = REPORTERS[key];

      const name = bufferFromString(reporter.name, 32);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
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
    });

    it.each(Object.keys(REPORTERS))("Setup - reporter %s is activated", async (key) => {
      const reporter = REPORTERS[key];

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const tokenAccount = await stakeToken.getTokenAccount(
        reporter.keypair.publicKey
      );

      const communityTokenAccount = await stakeToken.getTokenAccount(
        communityAccount, true
      );

      const tx = await program.rpc.activateReporter({
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          reporter: reporterAccount,
          stakeMint: stakeToken.mintAccount,
          reporterTokenAccount: tokenAccount,
          communityTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();
    });

    it("success", async () => {
      const reporter = REPORTERS.erin;

      const newPrice = new BN(2_000);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      const tx = await program.rpc.updateReplicationPrice(newPrice, {
        accounts: {
          sender: reporter.keypair.publicKey,
          community: communityAccount,
          network: networkAccount,
          reporter: reporterAccount,
        },
        signers: [reporter.keypair],
      });

      expect(tx).toBeTruthy();

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );
      expect(fetchedNetworkAccount.replicationPrice.eq(newPrice)).toBeTruthy();
    });

    it("fail - validator can't update replication price", async () => {
      const reporter = REPORTERS.alice;

      const newPrice = new BN(2_000);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      await expectThrowError(
        () =>
          program.rpc.updateReplicationPrice(newPrice, {
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              network: networkAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't update replication price", async () => {
      const reporter = REPORTERS.bob;

      const newPrice = new BN(2_000);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      await expectThrowError(
        () =>
          program.rpc.updateReplicationPrice(newPrice, {
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              network: networkAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("Unauthorized")
      );
    });

    it("fail - authority can't update replication price", async () => {
      const reporter = REPORTERS.carol;

      const newPrice = new BN(2_000);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      await expectThrowError(
        () =>
          program.rpc.updateReplicationPrice(newPrice, {
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              network: networkAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("Unauthorized")
      );
    });

    it("fail - publisher can't update replication price", async () => {
      const reporter = REPORTERS.dave;

      const newPrice = new BN(2_000);

      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          communityId
        );

      const [reporterAccount] = await program.pda.findReporterAddress(
        communityAccount,
        reporter.keypair.publicKey
      );

      const [networkAccount] = await program.pda.findNetworkAddress(
        communityAccount,
        "near"
      );

      await expectThrowError(
        () =>
          program.rpc.updateReplicationPrice(newPrice, {
            accounts: {
              sender: reporter.keypair.publicKey,
              community: communityAccount,
              network: networkAccount,
              reporter: reporterAccount,
            },
            signers: [reporter.keypair],
          }),
        programError("Unauthorized")
      );
    });
  });
});
