import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";

import { HapiCore } from "../target/types/hapi_core";
import { silenceConsole } from "./util/console";

export const ReporterType = {
  Inactive: { inactive: {} },
  Tracer: { tracer: {} },
  Full: { full: {} },
  Authority: { authority: {} },
};

export const CaseStatus = {
  Closed: { closed: {} },
  Open: { open: {} },
};

export const Category = {
  None: { none: {} },
  WalletService: { walletService: {} },
  MerchantService: { merchantService: {} },
  MiningPool: { miningPool: {} },
  LowRiskExchange: { lowRiskExchange: {} },
  MediumRiskExchange: { mediumRiskExchange: {} },
  DeFi: { deFi: {} },
  OTCBroker: { oTCBroker: {} },
  ATM: { aTM: {} },
  Gambling: { gambling: {} },
  IllicitOrganization: { illicitOrganization: {} },
  Mixer: { mixer: {} },
  DarknetService: { darknetService: {} },
  Scam: { scam: {} },
  Ransomware: { ransomware: {} },
  Theft: { theft: {} },
  Counterfeit: { counterfeit: {} },
  TerroristFinancing: { terroristFinancing: {} },
  Sanctions: { sanctions: {} },
  ChildAbuse: { childAbuse: {} },
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
    expect(communityAccount.cases.toNumber()).toEqual(0);

    const communityInfo = await provider.connection.getAccountInfoAndContext(
      community.publicKey
    );
    expect(communityInfo.value.owner).toEqual(program.programId);
    expect(communityInfo.value.data).toHaveLength(100);
  });

  it("Community shouldn't be initialized twice", async () => {
    const silencer = silenceConsole();

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

    silencer.close();
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

      const silencer = silenceConsole();

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

      silencer.close();
    }
  );

  it("Unauthorized users shouldn't be able to create a network in a community", async () => {
    let name = bufferFromString("bitcoin", 32);

    const [network, bump] = await web3.PublicKey.findProgramAddress(
      [bufferFromString("network"), community.publicKey.toBytes(), name],
      program.programId
    );

    const silencer = silenceConsole();

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

    silencer.close();
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
    expect(fetchedReporterAccount.reporterType).toEqual(
      ReporterType[reporter.type]
    );

    const reporterInfo = await provider.connection.getAccountInfoAndContext(
      reporterAccount
    );
    expect(reporterInfo.value.owner).toEqual(program.programId);
    expect(reporterInfo.value.data).toHaveLength(200);
  });

  it("Non-whitelisted actor should not be able to create cases", async () => {
    const reporter = nobody;
    const caseId = new anchor.BN(1);
    const caseName = bufferFromString("Case 1", 32);

    const [caseAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("case"),
        community.publicKey.toBytes(),
        caseId.toBuffer("le", 8),
      ],
      program.programId
    );

    // Direct attempt to report
    {
      const [reporterAccount] = await web3.PublicKey.findProgramAddress(
        [
          bufferFromString("reporter"),
          community.publicKey.toBytes(),
          reporter.publicKey.toBytes(),
        ],
        program.programId
      );

      const silencer = silenceConsole();

      await expect(() =>
        program.rpc.createCase(caseId, caseName.toJSON().data, bump, {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
            case: caseAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        })
      ).rejects.toThrowError(
        // This fails because reporterAccount should not exist or does not belong to the program
        /The given account is not owned by the executing program/
      );

      silencer.close();
    }

    // Attempt to impersonate a reporter that has correct permissions
    {
      const [reporterAccount] = await web3.PublicKey.findProgramAddress(
        [
          bufferFromString("reporter"),
          community.publicKey.toBytes(),
          REPORTERS.alice.keypair.publicKey.toBytes(),
        ],
        program.programId
      );

      const silencer = silenceConsole();

      await expect(() =>
        program.rpc.createCase(caseId, caseName.toJSON().data, bump, {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
            case: caseAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        })
      ).rejects.toThrowError(
        // This should fail because sender pubkey does not match reporterAccount pubkey
        /A raw constraint was violated/
      );

      silencer.close();
    }
  });

  it("Reporter without permission should not be able to create cases", async () => {
    const reporter = REPORTERS.carol.keypair;
    const caseId = new anchor.BN(1);
    const caseName = bufferFromString("Case 1", 32);

    const [caseAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("case"),
        community.publicKey.toBytes(),
        caseId.toBuffer("le", 8),
      ],
      program.programId
    );

    const [reporterAccount] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("reporter"),
        community.publicKey.toBytes(),
        reporter.publicKey.toBytes(),
      ],
      program.programId
    );

    const silencer = silenceConsole();

    await expect(() =>
      program.rpc.createCase(caseId, caseName.toJSON().data, bump, {
        accounts: {
          reporter: reporterAccount,
          sender: reporter.publicKey,
          community: community.publicKey,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter],
      })
    ).rejects.toThrowError(/A raw constraint was violated/);

    silencer.close();
  });

  it("Case 1 is created", async () => {
    const reporter = REPORTERS.alice.keypair;
    const caseId = new anchor.BN(1);
    const caseName = bufferFromString("Case 1", 32);

    const [caseAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("case"),
        community.publicKey.toBytes(),
        caseId.toBuffer("le", 8),
      ],
      program.programId
    );

    const [reporterAccount] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("reporter"),
        community.publicKey.toBytes(),
        reporter.publicKey.toBytes(),
      ],
      program.programId
    );

    const tx = await program.rpc.createCase(
      caseId,
      caseName.toJSON().data,
      bump,
      {
        accounts: {
          reporter: reporterAccount,
          sender: reporter.publicKey,
          community: community.publicKey,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter],
      }
    );

    expect(tx).toBeTruthy();

    const fetchedCaseAccount = await program.account.case.fetch(caseAccount);
    expect(Buffer.from(fetchedCaseAccount.name)).toEqual(caseName);
    expect(fetchedCaseAccount.bump).toEqual(bump);
    expect(fetchedCaseAccount.reporter).toEqual(reporterAccount);
    expect(fetchedCaseAccount.status).toEqual(CaseStatus.Open);
    expect(fetchedCaseAccount.id.toNumber()).toEqual(caseId.toNumber());

    const communityAccount = await program.account.community.fetch(
      community.publicKey
    );
    expect(communityAccount.cases.toNumber()).toEqual(caseId.toNumber());
  });

  it("Address created", async () => {
    const reporter = REPORTERS.bob.keypair;
    const pubkey = web3.PublicKey.decodeUnchecked(
      Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000000001",
        "hex"
      )
    );

    let networkName = bufferFromString("ethereum", 32);
    const [networkAccount, networkBump] =
      await web3.PublicKey.findProgramAddress(
        [
          bufferFromString("network"),
          community.publicKey.toBytes(),
          networkName,
        ],
        program.programId
      );

    const [addressAccount, bump] = await web3.PublicKey.findProgramAddress(
      [bufferFromString("address"), networkAccount.toBytes(), pubkey.toBytes()],
      program.programId
    );

    const [reporterAccount] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("reporter"),
        community.publicKey.toBytes(),
        reporter.publicKey.toBytes(),
      ],
      program.programId
    );

    const caseId = new anchor.BN(1);
    const [caseAccount] = await web3.PublicKey.findProgramAddress(
      [
        bufferFromString("case"),
        community.publicKey.toBytes(),
        caseId.toBuffer("le", 8),
      ],
      program.programId
    );

    const tx = await program.rpc.createAddress(pubkey, Category.None, 0, bump, {
      accounts: {
        sender: reporter.publicKey,
        address: addressAccount,
        community: community.publicKey,
        network: networkAccount,
        reporter: reporterAccount,
        case: caseAccount,
        systemProgram: web3.SystemProgram.programId,
      },
      signers: [reporter],
    });
    expect(tx).toBeTruthy();

    const fetchedAddressAccount = await program.account.address.fetch(
      addressAccount
    );
    expect(fetchedAddressAccount.bump).toEqual(bump);
    expect(fetchedAddressAccount.caseId.toNumber()).toEqual(caseId.toNumber());
    expect(fetchedAddressAccount.category).toEqual(Category.None);
    expect(fetchedAddressAccount.confidence).toEqual(0);
    expect(fetchedAddressAccount.risk).toEqual(0);
    expect(fetchedAddressAccount.community).toEqual(community.publicKey);
    expect(fetchedAddressAccount.address).toEqual(pubkey);
    expect(fetchedAddressAccount.network).toEqual(networkAccount);
    expect(fetchedAddressAccount.reporter).toEqual(reporterAccount);

    const addressInfo = await provider.connection.getAccountInfoAndContext(
      addressAccount
    );
    expect(addressInfo.value.owner).toEqual(program.programId);
    expect(addressInfo.value.data).toHaveLength(148);
  });
});
