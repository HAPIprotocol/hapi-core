import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import { bufferFromString, program } from "../../lib";

jest.setTimeout(10_000);

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

    const [tokenSignerAccount, tokenSignerBump] =
      await program.findCommunityTokenSignerAddress(community.publicKey);

    const tokenAccount = await stakeToken.createAccount(tokenSignerAccount);

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
          community: community.publicKey,
          stakeMint: stakeToken.mintAccount,
          tokenAccount: tokenAccount,
          tokenSigner: tokenSignerAccount,
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

      const [tokenSignerAccount, tokenSignerBump] =
        await program.findCommunityTokenSignerAddress(otherCommunity.publicKey);

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
              community: otherCommunity.publicKey,
              network: networkAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [nobody],
          }),
        /(Cross-program invocation with unauthorized signer or writable account|Program failed to complete)/
      );
    });

    it("success", async () => {
      const name = bufferFromString("near", 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [name.toJSON().data, new u64(10_000), new u64(20_000), bump];

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
      expect(fetchedNetworkAccount.tracerReward.toNumber()).toEqual(10_000);
      expect(fetchedNetworkAccount.confirmationReward.toNumber()).toEqual(
        20_000
      );

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
    it("fail - authority mismatch for community", async () => {
      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [new u64(40_000), new u64(50_000)];

      await expectThrowError(
        () =>
          program.rpc.updateNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: otherCommunity.publicKey,
              network: networkAccount,
            },
          }),
        "310: Authority mismatched"
      );
    });

    it("fail - network does not exist", async () => {
      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        "unknown"
      );

      const args = [new u64(40_000), new u64(50_000)];

      await expectThrowError(
        () =>
          program.rpc.updateNetwork(...args, {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
            },
          }),
        "167: The given account is not owned by the executing program"
      );
    });

    it("success", async () => {
      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        "near"
      );

      const args = [new u64(40_000), new u64(50_000)];

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
      expect(fetchedNetworkAccount.tracerReward.toNumber()).toEqual(40_000);
      expect(fetchedNetworkAccount.confirmationReward.toNumber()).toEqual(
        50_000
      );
    });
  });
});
