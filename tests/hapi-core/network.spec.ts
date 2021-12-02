import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import { bufferFromString, program } from "../../lib";

describe("HapiCore Network", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  let community: web3.Keypair;
  let otherCommunity: web3.Keypair;
  let stakeToken: TestToken;

  beforeAll(async () => {
    community = web3.Keypair.generate();
    otherCommunity = web3.Keypair.generate();

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    await stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000));

    const tokenAccount = await stakeToken.createAccount();

    await program.rpc.initialize(
      new u64(1),
      2,
      new u64(1_000),
      new u64(2_000),
      new u64(3_000),
      new u64(4_000),
      {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          stakeMint: stakeToken.mintAccount,
          tokenAccount: tokenAccount,
          tokenProgram: stakeToken.programId,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [community],
      }
    );
  });

  describe("create_network", () => {
    it("fail - invalid authority", async () => {
      let name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(10_000), bump];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /Cross-program invocation with unauthorized signer or writable account/
      );
    });

    it("fail - authority mismatch for community", async () => {
      let name = bufferFromString("near", 32);

      const otherTokenAccount = await stakeToken.createAccount();

      await program.rpc.initialize(
        new u64(1),
        2,
        new u64(1_000),
        new u64(2_000),
        new u64(3_000),
        new u64(4_000),
        {
          accounts: {
            authority: authority.publicKey,
            community: otherCommunity.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenAccount: otherTokenAccount,
            tokenProgram: stakeToken.programId,
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

      const [networkAccount, bump] = await program.findNetworkAddress(
        otherCommunity.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(10_000), bump];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunity.publicKey,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        "310: Authority mismatched"
      );
    });

    it("fail - community mismatch for network", async () => {
      let name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(10_000), bump];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: nobody.publicKey,
              community: community.publicKey,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /Cross-program invocation with unauthorized signer or writable account/
      );
    });

    it("success", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(10_000), bump];

      const tx = await program.rpc.createNetwork(...args, {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          network: networkAccount,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      expect(tx).toBeTruthy();

      const fetchedNetworkAccount = await program.account.network.fetch(
        networkAccount
      );
      expect(Buffer.from(fetchedNetworkAccount.name)).toEqual(name);
      expect(fetchedNetworkAccount.bump).toEqual(bump);

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        networkAccount
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data).toHaveLength(200);
    });

    it("fail - network already exists", async () => {
      let name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(10_000), bump];

      await expectThrowError(
        () =>
          program.rpc.createNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
          }),
        /failed to send transaction/
      );
    });
  });

  describe("update_network", () => {
    it.todo("fail - authority mismatch for community");

    it.todo("fail - network does not exist");

    it.todo("success");
  });
});
