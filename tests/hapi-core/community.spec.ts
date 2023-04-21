import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken } from "../util/token";
import { expectThrowError } from "../util/console";
import { pubkeyFromHex } from "../util/crypto";
import { programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";
import { ACCOUNT_SIZE, initHapiCore } from "../../lib";

describe("HapiCore Community", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  let stakeToken: TestToken;

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);
    await stakeToken.transfer(null, nobody.publicKey, 1_000_000);
  });

  describe("initialize_community", () => {
    it("fail - invalid token account", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(1)
        );

      const args = [
        new BN(1),
        communityBump,
        new BN(3),
        3,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
      ];

      const tokenAccount = web3.Keypair.generate().publicKey;

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              stakeMint: stakeToken.mintAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        "AnchorError caused by account: token_account. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - invalid mint account", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(1)
        );

      const args = [
        new BN(1),
        communityBump,
        new BN(3),
        3,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
      ];

      const fakeMint = web3.Keypair.generate().publicKey;

      const tokenAccount = await stakeToken.createAccount();

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              stakeMint: fakeMint,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        "AnchorError caused by account: stake_mint. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - invalid community", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(2)
        );

      const args = [
        new BN(1),
        communityBump,
        new BN(3),
        3,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
      ];

      const tokenAccount = await stakeToken.createAccount();

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: nobody.publicKey,
              stakeMint: stakeToken.mintAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [],
          }),
        "A seeds constraint was violated"
      );
    });

    it("success", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(1)
        );

      const args = [
        new BN(1),
        communityBump,
        new BN(3),
        3,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
      ];

      const tokenAccount = await stakeToken.getTokenAccount(
        communityAccount, true
      );

      const tx = await program.rpc.initializeCommunity(...args, {
        accounts: {
          authority: authority.publicKey,
          community: communityAccount,
          stakeMint: stakeToken.mintAccount,
          tokenAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      expect(tx).toBeTruthy();

      const communityData = await program.account.community.fetch(
        communityAccount
      );

      expect(communityData.authority).toEqual(authority.publicKey);
      expect(communityData.cases.toNumber()).toEqual(0);
      expect(communityData.stakeMint).toEqual(stakeToken.mintAccount);
      expect(communityData.bump).toEqual(communityBump);
      expect(communityData.communityId.toNumber()).toEqual(1);

      const communityInfo = await provider.connection.getAccountInfoAndContext(
        communityAccount
      );
      expect(communityInfo.value.owner).toEqual(program.programId);
      expect(communityInfo.value.data).toHaveLength(ACCOUNT_SIZE.community);
    });

    it("fail - already exists", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(1)
        );

      const args = [
        new BN(1),
        communityBump,
        new BN(3),
        3,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        new BN(5_000),
      ];

      const tokenAccount = await stakeToken.getTokenAccount(
        communityAccount, true
      );

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              stakeMint: stakeToken.mintAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /failed to send transaction/
      );
    });
  });

  describe("update_community", () => {
    it("fail - community doesn't exist", async () => {
      const args = [
        new BN(5),
        6,
        new BN(11_000),
        new BN(12_000),
        new BN(13_000),
        new BN(14_000),
        new BN(15_000),
      ];

      const someKey = pubkeyFromHex(
        "e1230a131f3747484f98d10b8d2a8759dcf597db3cf87c9b53e7862e756f0663"
      );

      await expectThrowError(
        () =>
          program.rpc.updateCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: someKey,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - community not initialized", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(2)
        );

      const args = [
        new BN(5),
        6,
        new BN(11_000),
        new BN(12_000),
        new BN(13_000),
        new BN(14_000),
        new BN(15_000),
      ];

      await expectThrowError(
        () =>
          program.rpc.updateCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success", async () => {
      {
        const [communityAccount, communityBump] =
          await program.pda.findCommunityAddress(
            new BN(2)
          );

        const args = [
          new BN(2),
          communityBump,
          new BN(3),
          3,
          new BN(1_000),
          new BN(2_000),
          new BN(3_000),
          new BN(4_000),
          new BN(5_000),
        ];

        const tokenAccount = await stakeToken.getTokenAccount(
          communityAccount, true
        );

        const tx = await program.rpc.initializeCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            stakeMint: stakeToken.mintAccount,
            tokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
        });

        expect(tx).toBeTruthy();
      }

      {
        const [communityAccount, communityBump] =
          await program.pda.findCommunityAddress(
            new BN(2)
          );

        const args = [
          new BN(5),
          6,
          new BN(11_000),
          new BN(12_000),
          new BN(13_000),
          new BN(14_000),
          new BN(15_000),
        ];

        const tx = await program.rpc.updateCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
          },
        });

        expect(tx).toBeTruthy();

        const communityData = await program.account.community.fetch(
          communityAccount
        );

        expect(communityData.authority).toEqual(authority.publicKey);
        expect(communityData.cases.toNumber()).toEqual(0);
        expect(communityData.bump).toEqual(communityBump);
        expect(communityData.communityId.toNumber()).toEqual(2);

        expect(communityData.stakeUnlockEpochs.toNumber()).toEqual(5);
        expect(communityData.confirmationThreshold).toEqual(6);
        expect(communityData.validatorStake.toNumber()).toEqual(11_000);
        expect(communityData.tracerStake.toNumber()).toEqual(12_000);
        expect(communityData.fullStake.toNumber()).toEqual(13_000);
        expect(communityData.authorityStake.toNumber()).toEqual(14_000);
        expect(communityData.appraiserStake.toNumber()).toEqual(15_000);
      }
    });

    it("fail - invalid authority", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(2)
        );

      const args = [
        new BN(5),
        6,
        new BN(11_000),
        new BN(12_000),
        new BN(13_000),
        new BN(14_000),
        new BN(15_000),
      ];

      await expectThrowError(
        () =>
          program.rpc.updateCommunity(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: communityAccount,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });
  });

  describe("set_community_authority", () => {
    it("fail - community doesn't exist", async () => {
      const someKey = pubkeyFromHex(
        "e1230a131f3747484f98d10b8d2a8759dcf597db3cf87c9b53e7862e756f0663"
      );

      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: authority.publicKey,
              newAuthority: nobody.publicKey,
              community: someKey,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - community not initialized", async () => {
      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          new BN(3)
        );

      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              newAuthority: nobody.publicKey,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success", async () => {
      const [communityAccount, communityBump] =
        await program.pda.findCommunityAddress(
          new BN(3)
        );

      {
        const args = [
          new BN(3),
          communityBump,
          new BN(3),
          3,
          new BN(1_000),
          new BN(2_000),
          new BN(3_000),
          new BN(4_000),
          new BN(5_000),
        ];

        const tokenAccount = await stakeToken.getTokenAccount(
          communityAccount, true
        );

        const tx = await program.rpc.initializeCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            stakeMint: stakeToken.mintAccount,
            tokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
        });

        expect(tx).toBeTruthy();
      }

      {
        const tx = await program.rpc.setCommunityAuthority({
          accounts: {
            authority: authority.publicKey,
            community: communityAccount,
            newAuthority: nobody.publicKey,
          },
        });

        expect(tx).toBeTruthy();

        const communityData = await program.account.community.fetch(
          communityAccount
        );

        expect(communityData.authority).toEqual(nobody.publicKey);
      }
    });

    it("fail - invalid authority", async () => {
      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          new BN(3)
        );


      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: authority.publicKey,
              community: communityAccount,
              newAuthority: nobody.publicKey,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - can't set the same authority", async () => {
      const [communityAccount, _] =
        await program.pda.findCommunityAddress(
          new BN(3)
        );


      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: nobody.publicKey,
              community: communityAccount,
              newAuthority: nobody.publicKey,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });
  });
});
