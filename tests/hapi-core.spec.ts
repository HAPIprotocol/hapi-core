import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { silenceConsole } from "./util/console";
import {
  CaseStatus,
  Category,
  program,
  ReporterRole,
  bufferFromString,
  ReporterStatus,
} from "../lib";

function pubkeyFromHex(hex: string): web3.PublicKey {
  return web3.PublicKey.decodeUnchecked(Buffer.from(hex, "hex"));
}

describe("hapi-core", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; type: keyof typeof ReporterRole }
  > = {
    alice: { name: "alice", keypair: web3.Keypair.generate(), type: "Full" },
    bob: { name: "bob", keypair: web3.Keypair.generate(), type: "Tracer" },
    carol: {
      name: "carol",
      keypair: web3.Keypair.generate(),
      type: "Validator",
    },
  };

  const NETWORKS: Record<string, { name: string }> = {
    ethereum: { name: "ethereum" },
    solana: { name: "solana" },
    near: { name: "near" },
  };

  const CASES: Record<
    string,
    {
      network: keyof typeof NETWORKS;
      caseId: BN;
      name: string;
      reporter: keyof typeof REPORTERS;
    }
  > = {
    safe: {
      network: "ethereum",
      caseId: new BN(1),
      name: "safe network addresses",
      reporter: "alice",
    },
    nftTracking: {
      network: "ethereum",
      caseId: new BN(2),
      name: "suspicious nft txes",
      reporter: "alice",
    },
  };

  const ADDRESSES: Record<
    string,
    {
      pubkey: web3.PublicKey;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
    }
  > = {
    blackhole: {
      pubkey: pubkeyFromHex(
        "0000000000000000000000000000000000000000000000000000000000000001"
      ),
      network: "ethereum",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
    nftMerchant: {
      pubkey: pubkeyFromHex(
        "6923f8792e9b41a2cc735d4c995b20c8d717cfda8d30e216fe1857389da71c94"
      ),
      network: "ethereum",
      reporter: "bob",
      category: "MerchantService",
      caseId: new BN(2),
      risk: 2,
    },
  };

  const ASSETS: Record<
    string,
    {
      mint: web3.PublicKey;
      assetId: Buffer;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
    }
  > = {
    stolenNft: {
      mint: pubkeyFromHex(
        "2873d85250e84e093c3f38c78e74c060c834db3cdaa4c09b4ed6aea9718959a8"
      ),
      assetId: Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000000001",
        "hex"
      ),
      caseId: new BN(2),
      category: "Theft",
      reporter: "bob",
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

    const tx = await program.rpc.initialize(new BN(4), 3, {
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
    expect(communityInfo.value.data).toHaveLength(256);
  });

  it("Community shouldn't be initialized twice", async () => {
    const silencer = silenceConsole();

    await expect(() =>
      program.rpc.initialize(new BN(3), 3, {
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

  it.each(Object.keys(NETWORKS))("Network '%s' is created", async (rawName) => {
    const network = NETWORKS[rawName];

    const name = bufferFromString(network.name, 32);

    const [networkAccount, bump] = await program.findNetworkAddress(
      community.publicKey,
      network.name
    );

    const tx = await program.rpc.createNetwork(name.toJSON().data, bump, {
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

  it.each(Object.keys(NETWORKS))(
    "Network '%s' shouldn't be initialized twice",
    async (rawName) => {
      const network = NETWORKS[rawName];

      let name = bufferFromString(network.name, 32);

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const silencer = silenceConsole();

      await expect(() =>
        program.rpc.createNetwork(name.toJSON().data, bump, {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            network: networkAccount,
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

    const reporterRole = ReporterRole[reporter.type];

    const tx = await program.rpc.createReporter(
      reporterRole,
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
    expect(fetchedReporterAccount.role).toEqual(
      ReporterRole[reporter.type]
    );
    expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);

    const reporterInfo = await provider.connection.getAccountInfoAndContext(
      reporterAccount
    );
    expect(reporterInfo.value.owner).toEqual(program.programId);
    expect(reporterInfo.value.data).toHaveLength(200);
  });

  it.todo("Inactive reporter can't create addresses");

  it.todo("Inactive reporter can't create assets");

  it("Non-whitelisted actor can't create cases", async () => {
    const reporter = nobody;
    const caseId = new BN(1);
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
    const caseId = new BN(1);
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

  it.each(Object.keys(REPORTERS))("Reporter %s is activated", async (key) => {
    const reporter = REPORTERS[key];

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.keypair.publicKey
    );

    const tx = await program.rpc.activateReporter({
      accounts: {
        sender: reporter.keypair.publicKey,
        community: community.publicKey,
        reporter: reporterAccount,
      },
      signers: [reporter.keypair],
    });

    expect(tx).toBeTruthy();

    const fetchedReporterAccount = await program.account.reporter.fetch(
      reporterAccount
    );
    expect(fetchedReporterAccount.role).toEqual(
      ReporterRole[reporter.type]
    );
    expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);
  });

  it.each(Object.keys(CASES))(
    "Case '%s' is created",
    async (key: keyof typeof CASES) => {
      const cs = CASES[key];

      const reporter = REPORTERS[cs.reporter].keypair;
      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const tx = await program.rpc.createCase(
        cs.caseId,
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
      expect(fetchedCaseAccount.id.toNumber()).toEqual(cs.caseId.toNumber());

      const communityAccount = await program.account.community.fetch(
        community.publicKey
      );
      expect(communityAccount.cases.toNumber()).toEqual(cs.caseId.toNumber());
    }
  );

  it.each(Object.keys(ADDRESSES))(
    "Address '%s' created",
    async (key: keyof typeof ADDRESSES) => {
      const addr = ADDRESSES[key];

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const tx = await program.rpc.createAddress(
        addr.pubkey,
        Category[addr.category],
        addr.risk,
        bump,
        {
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
        }
      );

      expect(tx).toBeTruthy();

      const fetchedAddressAccount = await program.account.address.fetch(
        addressAccount
      );
      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.caseId.toNumber()).toEqual(
        addr.caseId.toNumber()
      );
      expect(fetchedAddressAccount.category).toEqual(Category[addr.category]);
      expect(fetchedAddressAccount.confirmations).toEqual(0);
      expect(fetchedAddressAccount.risk).toEqual(addr.risk);
      expect(fetchedAddressAccount.community).toEqual(community.publicKey);
      expect(fetchedAddressAccount.address).toEqual(addr.pubkey);
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.reporter).toEqual(reporterAccount);

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(148);
    }
  );

  it.todo("Reporter can't create the same address twice");

  it.each(Object.keys(ASSETS))("Asset '%s' created", async (key) => {
    const asset = ASSETS[key];

    const reporter = REPORTERS[asset.reporter].keypair;

    const [networkAccount] = await program.findNetworkAddress(
      community.publicKey,
      "ethereum"
    );

    const [assetAccount, bump] = await program.findAssetAddress(
      networkAccount,
      asset.mint,
      asset.assetId
    );

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.publicKey
    );

    const caseId = new BN(1);
    const [caseAccount] = await program.findCaseAddress(
      community.publicKey,
      caseId
    );

    const tx = await program.rpc.createAsset(
      asset.mint,
      asset.assetId,
      Category.None,
      0,
      bump,
      {
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter],
      }
    );

    expect(tx).toBeTruthy();

    const fetchedAssetAccount = await program.account.asset.fetch(assetAccount);
    expect(fetchedAssetAccount.bump).toEqual(bump);
    expect(fetchedAssetAccount.caseId.toNumber()).toEqual(caseId.toNumber());
    expect(fetchedAssetAccount.category).toEqual(Category.None);
    expect(fetchedAssetAccount.confirmations).toEqual(0);
    expect(fetchedAssetAccount.risk).toEqual(0);
    expect(fetchedAssetAccount.community).toEqual(community.publicKey);
    expect(fetchedAssetAccount.mint).toEqual(asset.mint);
    expect(fetchedAssetAccount.assetId).toEqual(asset.assetId.toJSON().data);
    expect(fetchedAssetAccount.network).toEqual(networkAccount);
    expect(fetchedAssetAccount.reporter).toEqual(reporterAccount);

    const addressInfo = await provider.connection.getAccountInfoAndContext(
      assetAccount
    );
    expect(addressInfo.value.owner).toEqual(program.programId);
    expect(addressInfo.value.data).toHaveLength(180);
  });

  it.todo("Reporter can't create the same asset twice");

  it.each(Object.keys(REPORTERS))("Reporter %s is deactivated", async (key) => {
    const reporter = REPORTERS[key];

    const [reporterAccount] = await program.findReporterAddress(
      community.publicKey,
      reporter.keypair.publicKey
    );

    const tx = await program.rpc.deactivateReporter({
      accounts: {
        sender: reporter.keypair.publicKey,
        community: community.publicKey,
        reporter: reporterAccount,
      },
      signers: [reporter.keypair],
    });

    expect(tx).toBeTruthy();

    const fetchedReporterAccount = await program.account.reporter.fetch(
      reporterAccount
    );
    expect(fetchedReporterAccount.role).toEqual(
      ReporterRole[reporter.type]
    );
    expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
    expect(fetchedReporterAccount.unlockEpoch.toNumber()).toBeGreaterThan(0);
  });

  it.todo("Deactivated reporter can't create new address");

  it.todo("Reporter can't release their stake before unlock epoch");
});
