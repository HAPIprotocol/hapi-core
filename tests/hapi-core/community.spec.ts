import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { TestToken, u64 } from "../util/token";
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

  let community: web3.Keypair;
  let stakeToken: TestToken;

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    await stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000));
  });

  describe("initialize_community", () => {
    beforeAll(async () => {
      community = web3.Keypair.generate();
    });

    it("fail - invalid token account", async () => {
      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(community.publicKey);

      const args = [
        new u64(3),
        3,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
      ];

      const tokenAccount = web3.Keypair.generate().publicKey;

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              stakeMint: stakeToken.mintAccount,
              tokenSigner: tokenSignerAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [community],
          }),
        "AnchorError caused by account: token_account. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - invalid mint account", async () => {
      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(community.publicKey);

      const args = [
        new u64(3),
        3,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
      ];

      const fakeMint = web3.Keypair.generate().publicKey;

      const tokenAccount = await stakeToken.createAccount();

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              stakeMint: fakeMint,
              tokenSigner: tokenSignerAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [community],
          }),
        "AnchorError caused by account: stake_mint. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("fail - invalid community", async () => {
      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(community.publicKey);

      const args = [
        new u64(3),
        3,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
      ];

      const tokenAccount = await stakeToken.createAccount();

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: nobody.publicKey,
              stakeMint: stakeToken.mintAccount,
              tokenSigner: tokenSignerAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [],
          }),
        "Signature verification failed"
      );
    });

    it("success", async () => {
      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(community.publicKey);

      const args = [
        new u64(3),
        3,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
      ];

      const tokenAccount = await stakeToken.createAccount(tokenSignerAccount);

      const tx = await program.rpc.initializeCommunity(...args, {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          stakeMint: stakeToken.mintAccount,
          tokenSigner: tokenSignerAccount,
          tokenAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [community],
      });

      expect(tx).toBeTruthy();

      const communityAccount = await program.account.community.fetch(
        community.publicKey
      );

      expect(communityAccount.authority).toEqual(authority.publicKey);
      expect(communityAccount.cases.toNumber()).toEqual(0);
      expect(communityAccount.tokenSignerBump).toEqual(tokenSignerBump);
      expect(communityAccount.tokenSigner).toEqual(tokenSignerAccount);
      expect(communityAccount.tokenAccount).toEqual(tokenAccount);
      expect(communityAccount.stakeMint).toEqual(stakeToken.mintAccount);

      const communityInfo = await provider.connection.getAccountInfoAndContext(
        community.publicKey
      );
      expect(communityInfo.value.owner).toEqual(program.programId);
      expect(communityInfo.value.data).toHaveLength(ACCOUNT_SIZE.community);
    });

    it("fail - already exists", async () => {
      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(community.publicKey);

      const args = [
        new u64(3),
        3,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
      ];

      const tokenAccount = await stakeToken.createAccount();

      await expectThrowError(
        () =>
          program.rpc.initializeCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              stakeMint: stakeToken.mintAccount,
              tokenSigner: tokenSignerAccount,
              tokenAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [community],
          }),
        /failed to send transaction/
      );
    });
  });

  describe("update_community", () => {
    beforeAll(async () => {
      community = web3.Keypair.generate();
    });

    it("fail - community doesn't exist", async () => {
      const args = [
        new u64(5),
        6,
        new u64(11_000),
        new u64(12_000),
        new u64(13_000),
        new u64(14_000),
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
      const args = [
        new u64(5),
        6,
        new u64(11_000),
        new u64(12_000),
        new u64(13_000),
        new u64(14_000),
      ];

      await expectThrowError(
        () =>
          program.rpc.updateCommunity(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success", async () => {
      {
        const [tokenSignerAccount, tokenSignerBump] =
          await program.pda.findCommunityTokenSignerAddress(
            community.publicKey
          );

        const args = [
          new u64(3),
          3,
          new u64(1_000),
          new u64(2_000),
          new u64(3_000),
          new u64(4_000),
          tokenSignerBump,
        ];

        const tokenAccount = await stakeToken.createAccount(tokenSignerAccount);

        const tx = await program.rpc.initializeCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenSigner: tokenSignerAccount,
            tokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [community],
        });

        expect(tx).toBeTruthy();
      }

      {
        const args = [
          new u64(5),
          6,
          new u64(11_000),
          new u64(12_000),
          new u64(13_000),
          new u64(14_000),
        ];

        const tx = await program.rpc.updateCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
          },
        });

        expect(tx).toBeTruthy();

        const communityAccount = await program.account.community.fetch(
          community.publicKey
        );

        expect(communityAccount.authority).toEqual(authority.publicKey);
        expect(communityAccount.cases.toNumber()).toEqual(0);

        expect(communityAccount.stakeUnlockEpochs.toNumber()).toEqual(5);
        expect(communityAccount.confirmationThreshold).toEqual(6);
        expect(communityAccount.validatorStake.toNumber()).toEqual(11_000);
        expect(communityAccount.tracerStake.toNumber()).toEqual(12_000);
        expect(communityAccount.fullStake.toNumber()).toEqual(13_000);
        expect(communityAccount.authorityStake.toNumber()).toEqual(14_000);
      }
    });

    it("fail - invalid authority", async () => {
      const args = [
        new u64(5),
        6,
        new u64(11_000),
        new u64(12_000),
        new u64(13_000),
        new u64(14_000),
      ];

      await expectThrowError(
        () =>
          program.rpc.updateCommunity(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });
  });

  describe("set_community_authority", () => {
    beforeAll(async () => {
      community = web3.Keypair.generate();
    });

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
      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              newAuthority: nobody.publicKey,
            },
          }),
        "AnchorError caused by account: community. Error Code: AccountNotInitialized. Error Number: 3012. Error Message: The program expected this account to be already initialized."
      );
    });

    it("success", async () => {
      {
        const [tokenSignerAccount, tokenSignerBump] =
          await program.pda.findCommunityTokenSignerAddress(
            community.publicKey
          );

        const args = [
          new u64(3),
          3,
          new u64(1_000),
          new u64(2_000),
          new u64(3_000),
          new u64(4_000),
          tokenSignerBump,
        ];

        const tokenAccount = await stakeToken.createAccount(tokenSignerAccount);

        const tx = await program.rpc.initializeCommunity(...args, {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenSigner: tokenSignerAccount,
            tokenAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [community],
        });

        expect(tx).toBeTruthy();
      }

      {
        const tx = await program.rpc.setCommunityAuthority({
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            newAuthority: nobody.publicKey,
          },
        });

        expect(tx).toBeTruthy();

        const communityAccount = await program.account.community.fetch(
          community.publicKey
        );

        expect(communityAccount.authority).toEqual(nobody.publicKey);
      }
    });

    it("fail - invalid authority", async () => {
      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              newAuthority: nobody.publicKey,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - can't set the same authority", async () => {
      await expectThrowError(
        () =>
          program.rpc.setCommunityAuthority({
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
              newAuthority: nobody.publicKey,
            },
            signers: [nobody],
          }),
        programError("AuthorityMismatch")
      );
    });
  });
});
