import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";

import { HapiCore } from "../target/types/hapi_core";

export const ReporterType = {
  Inactive: { inactive: {} },
  Tracer: { tracer: {} },
  Full: { full: {} },
  Authority: { authority: {} },
};

function bufferFromString(str: string, bufferSize?: number) {
  const utf = anchor.utils.bytes.utf8.encode(str);

  if (!bufferSize || utf.byteLength === bufferSize) {
    return Buffer.from(utf);
  }

  if (bufferSize && utf.byteLength > bufferSize) {
    throw RangeError("Buffer size too small to fit the string");
  }

  return Buffer.concat(
    [Buffer.from(utf), Buffer.alloc(bufferSize - utf.byteLength)],
    bufferSize
  );
}

describe("hapi-core", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HapiCore as Program<HapiCore>;
  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; type: keyof typeof ReporterType }
  > = {
    alice: { name: "alice", keypair: web3.Keypair.generate(), type: "Full" },
    bob: { name: "bob", keypair: web3.Keypair.generate(), type: "Tracer" },
    carol: {
      name: "carol",
      keypair: web3.Keypair.generate(),
      type: "Inactive",
    },
  };

  let community: web3.Keypair;

  beforeAll(async () => {
    const tx = new web3.Transaction();

    tx.add(
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: nobody.publicKey,
        lamports: 10_000_000,
      }),
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
      })
    );

    await provider.send(tx);
  });

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
    expect(communityInfo.value.data).toHaveLength(100);
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
      let name = bufferFromString(rawName, 32);

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
      expect(networkInfo.value.data).toHaveLength(100);
    }
  );

  it.each(["ethereum", "solana", "near"])(
    "Network '%s' shouldn't be initialized twice",
    async (rawName) => {
      let name = bufferFromString(rawName, 32);

      const [network, bump] = await web3.PublicKey.findProgramAddress(
        [bufferFromString("network"), community.publicKey.toBytes(), name],
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
    let name = bufferFromString("bitcoin", 32);

    const [network, bump] = await web3.PublicKey.findProgramAddress(
      [bufferFromString("network"), community.publicKey.toBytes(), name],
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

  it.each(Object.keys(REPORTERS))("Reporter %s is created", async (key) => {
    const reporter = REPORTERS[key];

    const name = bufferFromString(reporter.name, 32);

    const [reporterAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("reporter"),
        community.publicKey.toBytes(),
        reporter.keypair.publicKey.toBytes(),
      ],
      program.programId
    );

    const reporterType = ReporterType[reporter.type];

    const tx = await program.rpc.createReporter(
      reporterType,
      name.toJSON().data,
      bump,
      {
        accounts: {
          authority: authority.publicKey,
          community: community.publicKey,
          reporter: reporterAccount,
          pubkey: reporter.keypair.publicKey,
          systemProgram: web3.SystemProgram.programId,
        },
      }
    );

    expect(tx).toBeTruthy();

    const fetchedReporterAccount = await program.account.reporter.fetch(
      reporterAccount
    );
    expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
    expect(fetchedReporterAccount.bump).toEqual(bump);
    expect(fetchedReporterAccount.reporterType).toEqual(ReporterType[reporter.type]);

    const networkInfo = await provider.connection.getAccountInfoAndContext(
      reporterAccount
    );
    expect(networkInfo.value.owner).toEqual(program.programId);
    expect(networkInfo.value.data).toHaveLength(100);
  });
});
