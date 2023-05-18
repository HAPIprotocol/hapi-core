import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";

import {
  ACCOUNT_SIZE,
  bufferFromString,
  initHapiCore,
  NetworkSchema,
} from "./lib";
import { programError } from "./util/error";
import { metadata } from "../target/idl/hapi_core_solana.json";

describe("HapiCore Network", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = web3.Keypair.generate();
  const another_authority = web3.Keypair.generate();

  const networkName = "near";

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

    await provider.connection.requestAirdrop(
      authority.publicKey,
      10_000_000
    );

    await provider.connection.requestAirdrop(
      another_authority.publicKey,
      10_000_000
    );

  });

  describe("create_network", () => {
    const name = bufferFromString(networkName, 32);
    const schema = NetworkSchema.Near;

    const stakeConfiguration = {
      unlockDuration: new BN(1_000),
      validatorStake: new BN(2_000),
      tracerStake: new BN(3_000),
      publisherStake: new BN(4_000),
      authorityStake: new BN(5_000),
    };

    const rewardConfiguration = {
      addressTracerReward: new BN(1_000),
      addressConfirmationReward: new BN(2_000),
      assetTracerReward: new BN(3_000),
      assetConfirmationReward: new BN(4_000),
    };

    it("fail - invalid token owner", async () => {

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        networkName
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        another_authority.publicKey,
      );

      const args = [
        name.toJSON().data,
        schema,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              stakeMint: stakeToken.mintAccount,
              stakeTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [authority]
          }),
        programError("IllegalOwner")
      );

    });

    it("fail - invalid token account", async () => {

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        networkName
      );

      const stakeTokenAccount = await rewardToken.getTokenAccount(
        another_authority.publicKey,
      );

      const args = [
        name.toJSON().data,
        schema,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              stakeMint: stakeToken.mintAccount,
              stakeTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [authority]
          }),
        programError("InvalidToken")
      );

    });

    it("success", async () => {

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        networkName
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const args = [
        name.toJSON().data,
        schema,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      const tx = await program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
          rewardMint: rewardToken.mintAccount,
          stakeMint: stakeToken.mintAccount,
          stakeTokenAccount,
          tokenProgram: rewardToken.programId,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [authority]
      });

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );

      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.authority).toEqual(authority.publicKey);
      expect(fetchedNetworkAccount.bump).toEqual(bump);
      expect(fetchedNetworkAccount.schema).toEqual(NetworkSchema.Near);
      expect(fetchedNetworkAccount.stakeMint).toEqual(stakeToken.mintAccount);
      expect(fetchedNetworkAccount.stakeInfo.authorityStake.eq(stakeConfiguration.authorityStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.publisherStake.eq(stakeConfiguration.publisherStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.tracerStake.eq(stakeConfiguration.tracerStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.unlockDuration.eq(stakeConfiguration.unlockDuration)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.validatorStake.eq(stakeConfiguration.validatorStake)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.rewardInfo.addressConfirmationReward.eq(rewardConfiguration.addressConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.addressTracerReward.eq(rewardConfiguration.addressTracerReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.assetConfirmationReward.eq(rewardConfiguration.assetConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.assetTracerReward.eq(rewardConfiguration.assetTracerReward)).toBeTruthy();

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data.length).toEqual(ACCOUNT_SIZE.network);
    });

    it("fail - network already exists", async () => {

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        networkName
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      const args = [
        name.toJSON().data,
        schema,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              stakeMint: stakeToken.mintAccount,
              stakeTokenAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [authority]
          }),
        /custom program error: 0x0/
      );
    });
  });

  describe("update_network_configuration", () => {
    const stakeConfiguration = {
      unlockDuration: new BN(2_000),
      validatorStake: new BN(3_000),
      tracerStake: new BN(4_000),
      publisherStake: new BN(5_000),
      authorityStake: new BN(6_000),
    };

    const rewardConfiguration = {
      addressTracerReward: new BN(2_000),
      addressConfirmationReward: new BN(3_000),
      assetTracerReward: new BN(4_000),
      assetConfirmationReward: new BN(5_000),
    };

    it("fail - authority mismatch", async () => {
      const [networkAccount, _] = await program.pda.findNetworkAddress(
        networkName
      );

      const args = [
        stakeConfiguration,
        rewardConfiguration,
      ];

      await expectThrowError(
        () =>
          program.rpc.updateConfiguration(...args, {
            accounts: {
              authority: another_authority.publicKey,
              network: networkAccount,
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );

    });

    it("success", async () => {

      const [networkAccount, _] = await program.pda.findNetworkAddress(
        networkName
      );

      const args = [
        stakeConfiguration,
        rewardConfiguration,
      ];

      const tx = await program.rpc.updateConfiguration(...args, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
        },
        signers: [authority]
      });

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.stakeInfo.authorityStake.eq(stakeConfiguration.authorityStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.publisherStake.eq(stakeConfiguration.publisherStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.tracerStake.eq(stakeConfiguration.tracerStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.unlockDuration.eq(stakeConfiguration.unlockDuration)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeInfo.validatorStake.eq(stakeConfiguration.validatorStake)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.addressConfirmationReward.eq(rewardConfiguration.addressConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.addressTracerReward.eq(rewardConfiguration.addressTracerReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.assetConfirmationReward.eq(rewardConfiguration.assetConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardInfo.assetTracerReward.eq(rewardConfiguration.assetTracerReward)).toBeTruthy();
    });
  });

  describe("set_network_authority", () => {

    it("fail - authority mismatch", async () => {
      const [networkAccount, _] = await program.pda.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.rpc.setNetworkAuthority({
            accounts: {
              authority: another_authority.publicKey,
              newAuthority: another_authority.publicKey,
              network: networkAccount,
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - same authority", async () => {
      const [networkAccount, _] = await program.pda.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.rpc.setNetworkAuthority({
            accounts: {
              authority: authority.publicKey,
              newAuthority: authority.publicKey,
              network: networkAccount,
            },
            signers: [authority]
          }),
        programError("AuthorityMismatch")
      );
    });

    it("success", async () => {

      const [networkAccount, _] = await program.pda.findNetworkAddress(
        networkName
      );

      const tx = await program.rpc.setNetworkAuthority({
        accounts: {
          authority: authority.publicKey,
          newAuthority: another_authority.publicKey,
          network: networkAccount,
        },
        signers: [authority]
      });

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.authority).toEqual(another_authority.publicKey);
    });
  });


});
