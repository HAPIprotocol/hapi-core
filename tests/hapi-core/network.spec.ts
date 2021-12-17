import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import { bufferFromString, initHapiCore } from "../../lib";
import { programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";

describe("HapiCore Network", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  let community: web3.Keypair;
  let otherCommunity: web3.Keypair;
  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const addressTracerReward = new u64(1_000);
  const addressConfirmationReward = new u64(2_000);
  const assetTracerReward = new u64(3_000);
  const assetConfirmationReward = new u64(4_000);

  beforeAll(async () => {
    community = web3.Keypair.generate();
    otherCommunity = web3.Keypair.generate();

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    await stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000));

    rewardToken = new TestToken(provider);
    await rewardToken.mint(new u64(0));

    const [tokenSignerAccount, tokenSignerBump] =
      await program.pda.findCommunityTokenSignerAddress(community.publicKey);

    const tokenAccount = await stakeToken.createAccount(tokenSignerAccount);

    await program.rpc.initializeCommunity(
      new u64(1),
      2,
      addressTracerReward,
      addressConfirmationReward,
      assetTracerReward,
      assetConfirmationReward,
      tokenSignerBump,
      {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          stakeMint: stakeToken.mintAccount,
          tokenAccount: tokenAccount,
          tokenSigner: tokenSignerAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [community],
      }
    );
  });

  describe("create_network", () => {
    it("fail - invalid authority", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, networkBump] =
        await program.pda.findNetworkAddress(community.publicKey, "near");

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(networkAccount);

      const args = [
        name.toJSON().data,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        networkBump,
        rewardSignerBump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /Cross-program invocation with unauthorized signer or writable account/
      );
    });

    it("fail - authority mismatch for community", async () => {
      const name = bufferFromString("near", 32);

      const [tokenSignerAccount, tokenSignerBump] =
        await program.pda.findCommunityTokenSignerAddress(
          otherCommunity.publicKey
        );

      const otherTokenAccount = await stakeToken.createAccount(
        tokenSignerAccount
      );

      await program.rpc.initializeCommunity(
        new u64(1),
        2,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        tokenSignerBump,
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunity.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenAccount: otherTokenAccount,
            tokenSigner: tokenSignerAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [otherCommunity],
        }
      );

      await program.rpc.setCommunityAuthority({
        accounts: {
          authority: authority.publicKey,
          community: otherCommunity.publicKey,
          newAuthority: nobody.publicKey,
        },
      });

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        otherCommunity.publicKey,
        "near"
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(networkAccount);

      const args = [
        name.toJSON().data,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        rewardSignerBump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunity.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - community mismatch for network", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(networkAccount);

      const args = [
        name.toJSON().data,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        rewardSignerBump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: otherCommunity.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /(custom program error: 0xbc4|Cross-program invocation with unauthorized signer or writable account|Program failed to complete)/
      );
    });

    it("success", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(networkAccount);

      const args = [
        name.toJSON().data,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        rewardSignerBump,
      ];

      const tx = await program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          network: networkAccount,
          rewardMint: rewardToken.mintAccount,
          rewardSigner: rewardSignerAccount,
          tokenProgram: rewardToken.programId,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );
      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
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
      expect(fetchedNetworkAccount.rewardSignerBump).toEqual(rewardSignerBump);
      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.rewardSigner).toEqual(rewardSignerAccount);

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data).toHaveLength(200);
    });

    it("fail - network already exists", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(community.publicKey);

      const args = [
        name.toJSON().data,
        addressTracerReward,
        addressConfirmationReward,
        assetTracerReward,
        assetConfirmationReward,
        bump,
        rewardSignerBump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
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
      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
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
              community: otherCommunity.publicKey,
              network: networkAccount,
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - network does not exist", async () => {
      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
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
              community: community.publicKey,
              network: networkAccount,
            },
          }),
        "3012: The program expected this account to be already initialized"
      );
    });

    it("success", async () => {
      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
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
          community: community.publicKey,
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
});
