import * as anchor from "@project-serum/anchor";
import { web3 } from "@project-serum/anchor";

import { silenceConsole } from "./util/console";
import {
  CaseStatus,
  Category,
  program,
  ReporterType,
  bufferFromString,
} from "../lib";

describe("hapi-core", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

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
      type: "Validator",
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
    expect(communityInfo.value.data).toHaveLength(200);
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

      const [network, bump] = await program.findNetworkAddress(
        community.publicKey,
        rawName
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
      expect(networkInfo.value.data).toHaveLength(200);
    }
  );

  it.each(["ethereum", "solana", "near"])(
    "Network '%s' shouldn't be initialized twice",
    async (rawName) => {
      let name = bufferFromString(rawName, 32);

      const [network, bump] = await program.findNetworkAddress(
        community.publicKey,
        rawName
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

    const [network, bump] = await program.findNetworkAddress(
      community.publicKey,
      "bitcoin"
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

    const [reporterAccount, bump] = await program.findReporterAddress(
      community.publicKey,
      reporter.keypair.publicKey
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

    const [caseAccount, bump] = await program.findCaseAddress(
      community.publicKey,
      caseId
    );

    // Direct attempt to report
    {
      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
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
      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        REPORTERS.alice.keypair.publicKey
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

    const [caseAccount, bump] = await program.findCaseAddress(
      community.publicKey,
      caseId
    );

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.publicKey
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

    const [caseAccount, bump] = await program.findCaseAddress(
      community.publicKey,
      caseId
    );

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.publicKey
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

    const [networkAccount] = await program.findNetworkAddress(
      community.publicKey,
      "ethereum"
    );

    const [addressAccount, bump] = await program.findAddressAddress(
      networkAccount,
      pubkey
    );

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.publicKey
    );

    const caseId = new anchor.BN(1);
    const [caseAccount] = await program.findCaseAddress(
      community.publicKey,
      caseId
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
    expect(fetchedAddressAccount.confirmations).toEqual(0);
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
