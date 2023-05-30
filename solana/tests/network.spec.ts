import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";

import {
  ACCOUNT_SIZE,
  bufferFromString,
  HapiCoreProgram,
} from "../lib";
import { programError } from "./util/error";
import { PublicKey } from "@solana/web3.js";

describe("HapiCore Network", () => {
  const program = new HapiCoreProgram(new web3.PublicKey("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk"));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;
  const another_authority = web3.Keypair.generate();

  const networkName = "NetworkTest";

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const programDataAddress = web3.PublicKey.findProgramAddressSync(
    [program.programId.toBytes()],
    new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
  )[0];

  beforeAll(async () => {

    const wait: Promise<unknown>[] = [];

    stakeToken = new TestToken(provider);
    wait.push(stakeToken.mint(1_000_000_000));

    rewardToken = new TestToken(provider);
    wait.push(rewardToken.mint(1_000_000_000));

    wait.push(provider.connection.requestAirdrop(
      another_authority.publicKey,
      10_000_000
    ));

    await Promise.all(wait);

  });

  describe("create_network", () => {
    const name = bufferFromString(networkName, 32);

    const stakeConfiguration = {
      unlockDuration: new BN(1_000),
      validatorStake: new BN(2_000),
      tracerStake: new BN(3_000),
      publisherStake: new BN(4_000),
      authorityStake: new BN(5_000),
      appraiserStake: new BN(6_000),

    };

    const rewardConfiguration = {
      addressTracerReward: new BN(1_000),
      addressConfirmationReward: new BN(2_000),
      assetTracerReward: new BN(3_000),
      assetConfirmationReward: new BN(4_000),
    };

    it("fail - authority mismatch", async () => {

      const [networkAccount, bump] = program.findNetworkAddress(
        networkName
      );

      const args = [
        name.toJSON().data,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.createNetwork(...args, {
            accounts: {
              authority: another_authority.publicKey,
              network: networkAccount,
              rewardMint: PublicKey.default,
              stakeMint: PublicKey.default,
              programAccount: program.programId,
              programData: programDataAddress,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );

    });

    it("success - with default mints", async () => {

      const [networkAccount, bump] = program.findNetworkAddress(
        networkName
      );

      const args = [
        name.toJSON().data,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await program.program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
          rewardMint: PublicKey.default,
          stakeMint: PublicKey.default,
          programAccount: program.programId,
          programData: programDataAddress,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.authority).toEqual(authority.publicKey);
      expect(fetchedNetworkAccount.bump).toEqual(bump);
      expect(fetchedNetworkAccount.stakeMint).toEqual(PublicKey.default);
      expect(fetchedNetworkAccount.stakeConfiguration.authorityStake.eq(stakeConfiguration.authorityStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.appraiserStake.eq(stakeConfiguration.appraiserStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.publisherStake.eq(stakeConfiguration.publisherStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.tracerStake.eq(stakeConfiguration.tracerStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.unlockDuration.eq(stakeConfiguration.unlockDuration)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.validatorStake.eq(stakeConfiguration.validatorStake)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardMint).toEqual(PublicKey.default);
      expect(fetchedNetworkAccount.rewardConfiguration.addressConfirmationReward.eq(rewardConfiguration.addressConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.addressTracerReward.eq(rewardConfiguration.addressTracerReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetConfirmationReward.eq(rewardConfiguration.assetConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetTracerReward.eq(rewardConfiguration.assetTracerReward)).toBeTruthy();

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data.length).toEqual(ACCOUNT_SIZE.network);
    });

    it("success - with existing mints", async () => {
      const name = bufferFromString("networTest2", 32);
      const [networkAccount, bump] = program.findNetworkAddress(
        "networTest2"
      );

      const args = [
        name.toJSON().data,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await program.program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
          rewardMint: rewardToken.mintAccount,
          stakeMint: stakeToken.mintAccount,
          programAccount: program.programId,
          programData: programDataAddress,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.authority).toEqual(authority.publicKey);
      expect(fetchedNetworkAccount.bump).toEqual(bump);
      expect(fetchedNetworkAccount.stakeMint).toEqual(stakeToken.mintAccount);
      expect(fetchedNetworkAccount.stakeConfiguration.authorityStake.eq(stakeConfiguration.authorityStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.appraiserStake.eq(stakeConfiguration.appraiserStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.publisherStake.eq(stakeConfiguration.publisherStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.tracerStake.eq(stakeConfiguration.tracerStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.unlockDuration.eq(stakeConfiguration.unlockDuration)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.validatorStake.eq(stakeConfiguration.validatorStake)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.rewardConfiguration.addressConfirmationReward.eq(rewardConfiguration.addressConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.addressTracerReward.eq(rewardConfiguration.addressTracerReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetConfirmationReward.eq(rewardConfiguration.assetConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetTracerReward.eq(rewardConfiguration.assetTracerReward)).toBeTruthy();

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data.length).toEqual(ACCOUNT_SIZE.network);
    });

    it("fail - network already exists", async () => {

      const [networkAccount, bump] = program.findNetworkAddress(
        networkName
      );

      const args = [
        name.toJSON().data,
        stakeConfiguration,
        rewardConfiguration,
        bump,
      ];

      await expectThrowError(
        () =>
          program.program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              rewardMint: PublicKey.default,
              stakeMint: PublicKey.default,
              programAccount: program.programId,
              programData: programDataAddress,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /custom program error: 0x0/
      );
    });
  });

  describe("update_stake_configuration", () => {
    const stakeConfiguration = {
      unlockDuration: new BN(2_000),
      validatorStake: new BN(3_000),
      tracerStake: new BN(4_000),
      publisherStake: new BN(5_000),
      authorityStake: new BN(6_000),
      appraiserStake: new BN(7_000),
    };

    it("fail - authority mismatch", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.updateStakeConfiguration(stakeConfiguration, {
            accounts: {
              authority: another_authority.publicKey,
              network: networkAccount,
              stakeMint: stakeToken.mintAccount,
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );

    });

    it("success", async () => {

      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await program.program.rpc.updateStakeConfiguration(stakeConfiguration, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
          stakeMint: stakeToken.mintAccount,
        },
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.stakeMint).toEqual(stakeToken.mintAccount);
      expect(fetchedNetworkAccount.stakeConfiguration.authorityStake.eq(stakeConfiguration.authorityStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.appraiserStake.eq(stakeConfiguration.appraiserStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.publisherStake.eq(stakeConfiguration.publisherStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.tracerStake.eq(stakeConfiguration.tracerStake)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.unlockDuration.eq(stakeConfiguration.unlockDuration)).toBeTruthy();
      expect(fetchedNetworkAccount.stakeConfiguration.validatorStake.eq(stakeConfiguration.validatorStake)).toBeTruthy();
    });

    it("fail - mint has already been updated", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.updateStakeConfiguration(stakeConfiguration, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              stakeMint: rewardToken.mintAccount,
            },
          }),
        programError("UpdatedMint")
      );

    });
  });

  describe("update_reward_configuration", () => {
    const rewardConfiguration = {
      addressTracerReward: new BN(2_000),
      addressConfirmationReward: new BN(3_000),
      assetTracerReward: new BN(4_000),
      assetConfirmationReward: new BN(5_000),
    };

    it("fail - authority mismatch", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.updateRewardConfiguration(rewardConfiguration, {
            accounts: {
              authority: another_authority.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );

    });

    it("success", async () => {

      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await program.program.rpc.updateRewardConfiguration(rewardConfiguration, {
        accounts: {
          authority: authority.publicKey,
          network: networkAccount,
          rewardMint: rewardToken.mintAccount
        },
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.rewardConfiguration.addressConfirmationReward.eq(rewardConfiguration.addressConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.addressTracerReward.eq(rewardConfiguration.addressTracerReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetConfirmationReward.eq(rewardConfiguration.assetConfirmationReward)).toBeTruthy();
      expect(fetchedNetworkAccount.rewardConfiguration.assetTracerReward.eq(rewardConfiguration.assetTracerReward)).toBeTruthy();
    });

    it("fail - mint has already been updated", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.updateRewardConfiguration(rewardConfiguration, {
            accounts: {
              authority: authority.publicKey,
              network: networkAccount,
              rewardMint: stakeToken.mintAccount,
            },
          }),
        programError("UpdatedMint")
      );

    });
  });

  describe("set_network_authority", () => {

    it("fail - authority mismatch", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.setAuthority({
            accounts: {
              authority: another_authority.publicKey,
              newAuthority: another_authority.publicKey,
              network: networkAccount,
              programAccount: program.programId,
              programData: programDataAddress
            },
            signers: [another_authority]
          }),
        programError("AuthorityMismatch")
      );
    });

    it("fail - same authority", async () => {
      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await expectThrowError(
        () =>
          program.program.rpc.setAuthority({
            accounts: {
              authority: authority.publicKey,
              newAuthority: authority.publicKey,
              network: networkAccount,
              programAccount: program.programId,
              programData: programDataAddress
            },
          }),
        programError("AuthorityMismatch")
      );
    });

    it("success - program update authority signer", async () => {

      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await program.program.rpc.setAuthority({
        accounts: {
          authority: authority.publicKey,
          newAuthority: another_authority.publicKey,
          network: networkAccount,
          programAccount: program.programId,
          programData: programDataAddress
        },
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.authority).toEqual(another_authority.publicKey);
    });

    it("success - network authority signer", async () => {

      const [networkAccount, _] = program.findNetworkAddress(
        networkName
      );

      await program.program.rpc.setAuthority({
        accounts: {
          authority: another_authority.publicKey,
          newAuthority: authority.publicKey,
          network: networkAccount,
          programAccount: program.programId,
          programData: programDataAddress
        },
        signers: [another_authority]
      });

      const fetchedNetworkAccount = await program.program.account.network.fetch(
        networkAccount
      );

      expect(fetchedNetworkAccount.authority).toEqual(authority.publicKey);
    });
  });
});
