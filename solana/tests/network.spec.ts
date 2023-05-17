import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";

import {
  ACCOUNT_SIZE,
  bufferFromString,
  initHapiCore,
  NetworkSchema,
} from "../lib";
import { programError } from "./util/error";
import { metadata } from "../target/idl/hapi_core.json";

describe("HapiCore Network", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;
  const another_authority = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

  });

  describe("create_network", () => {

    it("fail - invalid token account", async () => {

      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        "near"
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        another_authority.publicKey,
      );

      let stakeConfiguration = {
        unlockDuration: new BN(1_000),
        validatorStake: new BN(2_000),
        tracerStake: new BN(3_000),
        publisherStake: new BN(4_000),
        authorityStake: new BN(5_000),
      };

      let rewardConfiguration = {
        addressTracerReward: new BN(1_000),
        addressConfirmationReward: new BN(2_000),
        assetTracerReward: new BN(3_000),
        assetConfirmationReward: new BN(4_000),
      };

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
          }),
        programError("InvalidToken")
      );

    });

    it("success", async () => {

      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        "near"
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      console.log(stakeTokenAccount);


      let stakeConfiguration = {
        unlockDuration: new BN(1_000),
        validatorStake: new BN(2_000),
        tracerStake: new BN(3_000),
        publisherStake: new BN(4_000),
        authorityStake: new BN(5_000),
      };

      let rewardConfiguration = {
        addressTracerReward: new BN(1_000),
        addressConfirmationReward: new BN(2_000),
        assetTracerReward: new BN(3_000),
        assetConfirmationReward: new BN(4_000),
      };

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
      });

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );

      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.authority).toEqual(authority);
      expect(fetchedNetworkAccount.bump).toEqual(bump);
      expect(fetchedNetworkAccount.schema).toEqual(NetworkSchema.Near);
      expect(fetchedNetworkAccount.stakeMint).toEqual(stakeToken.mintAccount);
      expect(fetchedNetworkAccount.stakeInfo).toEqual(stakeConfiguration);
      expect(fetchedNetworkAccount.rewardMint).toEqual(rewardToken.mintAccount);
      expect(fetchedNetworkAccount.rewardInfo).toEqual(rewardConfiguration);

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data).toEqual(ACCOUNT_SIZE.network);
    });

    it("fail - network already exists", async () => {

      const name = bufferFromString("near", 32);

      const schema = NetworkSchema.Near;

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        "near"
      );

      const stakeTokenAccount = await stakeToken.getTokenAccount(
        networkAccount,
        true
      );

      console.log(stakeTokenAccount);


      let stakeConfiguration = {
        unlockDuration: new BN(1_000),
        validatorStake: new BN(2_000),
        tracerStake: new BN(3_000),
        publisherStake: new BN(4_000),
        authorityStake: new BN(5_000),
      };

      let rewardConfiguration = {
        addressTracerReward: new BN(1_000),
        addressConfirmationReward: new BN(2_000),
        assetTracerReward: new BN(3_000),
        assetConfirmationReward: new BN(4_000),
      };

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
          }),
        /custom program error: 0x0/
      );
    });
  });


});
