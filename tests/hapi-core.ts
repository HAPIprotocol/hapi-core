import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";

import { HapiCore } from "../target/types/hapi_core";

describe("hapi-core", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HapiCore as Program<HapiCore>;
  const authority = provider.wallet;
  const nobody = web3.Keypair.generate();

  let community: web3.Keypair;

  it("Community is initialized", async () => {
    community = web3.Keypair.generate();

    const tx = await program.rpc.initialize({
      accounts: {
        authority: authority.publicKey,
        community: community.publicKey,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [community],
    });

    expect(tx).toBeTruthy();

    const communityAccount = await program.account.community.fetch(
      community.publicKey
    );

    expect(communityAccount.authority).toEqual(authority.publicKey);
    expect(communityAccount.caseCount.toNumber()).toEqual(0);

    const communityInfo = await provider.connection.getAccountInfoAndContext(
      community.publicKey
    );
    expect(communityInfo.value.owner).toEqual(program.programId);
    expect(communityInfo.value.data).toHaveLength(48);
  });

  it("Community shouldn't be initialized twice", async () => {
    await expect(() =>
      program.rpc.initialize({
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [community],
      })
    ).rejects.toThrowError(/failed to send transaction/);
  });

  it.each(["ethereum", "solana", "near"])(
    "Network '%s' is created",
    async (rawName) => {
      let name = Buffer.alloc(32);
      name.write(rawName);

      const [network, bump] = await web3.PublicKey.findProgramAddress(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode("network")),
          community.publicKey.toBytes(),
          name,
        ],
        program.programId
      );

      const tx = await program.rpc.createNetwork(name.toJSON().data, bump, {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          network,
          systemProgram: web3.SystemProgram.programId,
        },
      });

      expect(tx).toBeTruthy();

      const networkAccount = await program.account.network.fetch(network);
      expect(Buffer.from(networkAccount.name)).toEqual(name);
      expect(networkAccount.bump).toEqual(bump);

      const networkInfo = await provider.connection.getAccountInfoAndContext(
        network
      );
      expect(networkInfo.value.owner).toEqual(program.programId);
      expect(networkInfo.value.data).toHaveLength(41);
    }
  );

  it.each(["ethereum", "solana", "near"])(
    "Network '%s' shouldn't be initialized twice",
    async (rawName) => {
      let name = Buffer.alloc(32);
      name.write(rawName);

      const [network, bump] = await web3.PublicKey.findProgramAddress(
        [
          Buffer.from(anchor.utils.bytes.utf8.encode("network")),
          community.publicKey.toBytes(),
          name,
        ],
        program.programId
      );

      await expect(() =>
        program.rpc.createNetwork(name.toJSON().data, bump, {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            network,
            systemProgram: web3.SystemProgram.programId,
          },
        })
      ).rejects.toThrowError(/failed to send transaction/);
    }
  );

  it("Unauthorized users shouldn't be able to create a network in a community", async () => {
    let name = Buffer.alloc(32);
    name.write("bitcoin");

    const [network, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("network")),
        community.publicKey.toBytes(),
        name,
      ],
      program.programId
    );

    await expect(() =>
      program.rpc.createNetwork(name.toJSON().data, bump, {
        accounts: {
          authority: nobody.publicKey,
          community: community.publicKey,
          network,
          systemProgram: web3.SystemProgram.programId,
        },
      })
    ).rejects.toThrowError(/Signature verification failed/);
  });
});
