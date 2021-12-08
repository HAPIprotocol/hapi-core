import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  bufferFromString,
  CaseStatus,
  Category,
  program,
  ReporterRole,
} from "../../lib";
import { pubkeyFromHex } from "../util/crypto";

jest.setTimeout(10_000);

describe("HapiCore Asset", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();
  const community = web3.Keypair.generate();

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; role: keyof typeof ReporterRole }
  > = {
    alice: {
      name: "alice",
      keypair: web3.Keypair.generate(),
      role: "Full",
    },
    bob: {
      name: "bob",
      keypair: web3.Keypair.generate(),
      role: "Tracer",
    },
    carol: {
      name: "carol",
      keypair: web3.Keypair.generate(),
      role: "Authority",
    },
    dave: {
      name: "dave",
      keypair: web3.Keypair.generate(),
      role: "Validator",
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
      reporter: "carol",
    },
  };

  const ASSETS: Record<
    string,
    {
      mint: web3.PublicKey;
      assetId: Buffer;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
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
      network: "ethereum",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
    niceNft: {
      mint: pubkeyFromHex(
        "227b144f6d3dafb46cb632f12a260fac968455be71e289102ead3f95db7685bf"
      ),
      assetId: Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000042069",
        "hex"
      ),
      network: "solana",
      category: "None",
      reporter: "alice",
      caseId: new BN(2),
      risk: 0,
    },
  };

  beforeAll(async () => {
    let wait: Promise<unknown>[] = [];

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    wait.push(stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000)));

    rewardToken = new TestToken(provider);
    wait.push(rewardToken.mint(new u64(0)));

    const tx = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: nobody.publicKey,
        lamports: 10_000_000,
      }),
      ...Object.keys(REPORTERS).map((key) =>
        web3.SystemProgram.transfer({
          fromPubkey: authority.publicKey,
          toPubkey: REPORTERS[key].keypair.publicKey,
          lamports: 10_000_000,
        })
      )
    );

    wait.push(provider.send(tx));

    for (const reporter of Object.keys(REPORTERS)) {
      wait.push(
        stakeToken.transfer(
          null,
          REPORTERS[reporter].keypair.publicKey,
          new u64(1_000_000)
        )
      );
    }

    const [tokenSignerAccount, tokenSignerBump] =
      await program.findCommunityTokenSignerAddress(community.publicKey);

    const communityTokenAccount = await stakeToken.createAccount(
      tokenSignerAccount
    );

    wait.push(
      program.rpc.initializeCommunity(
        new u64(10),
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
            tokenAccount: communityTokenAccount,
            tokenSigner: tokenSignerAccount,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [community],
        }
      )
    );

    await Promise.all(wait);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      const [reporterAccount, bump] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      wait.push(
        program.rpc.createReporter(
          ReporterRole[reporter.role],
          bufferFromString(reporter.name, 32).toJSON().data,
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
        )
      );
    }

    for (const key of Object.keys(NETWORKS)) {
      const network = NETWORKS[key];

      const [networkAccount, bump] = await program.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.findNetworkRewardSignerAddress(networkAccount);

      wait.push(
        program.rpc.createNetwork(
          bufferFromString(network.name, 32).toJSON().data,
          new u64(10_000),
          new u64(20_000),
          bump,
          rewardSignerBump,
          {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
              rewardMint: rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
              tokenProgram: rewardToken.programId,
              systemProgram: web3.SystemProgram.programId,
            },
          }
        )
      );
    }

    await Promise.all(wait);

    for (const key of Object.keys(REPORTERS)) {
      const reporter = REPORTERS[key];

      wait.push(
        (async () => {
          const [reporterAccount] = await program.findReporterAddress(
            community.publicKey,
            reporter.keypair.publicKey
          );

          const reporterTokenAccount = await stakeToken.getTokenAccount(
            reporter.keypair.publicKey
          );

          await program.rpc.activateReporter({
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              reporter: reporterAccount,
              stakeMint: stakeToken.mintAccount,
              reporterTokenAccount: reporterTokenAccount,
              communityTokenAccount: communityTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter.keypair],
          });
        })()
      );
    }

    await Promise.all(wait);

    for (const key of Object.keys(CASES)) {
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

      await program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
        accounts: {
          reporter: reporterAccount,
          sender: reporter.publicKey,
          community: community.publicKey,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        },
        signers: [reporter],
      });
    }

    // Close the 'nftTracking' case
    {
      const cs = CASES.nftTracking;

      const reporter = REPORTERS[cs.reporter].keypair;
      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      await program.rpc.updateCase(caseName.toJSON().data, CaseStatus.Closed, {
        accounts: {
          reporter: reporterAccount,
          sender: reporter.publicKey,
          community: community.publicKey,
          case: caseAccount,
        },
        signers: [reporter],
      });
    }
  });

  describe("create_asset", () => {
    it("fail - risk range", async () => {
      const addr = ASSETS.stolenNft;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        addr.mint,
        addr.assetId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            addr.mint,
            addr.assetId,
            Category[addr.category],
            100,
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
          ),
        "313: Risk score must be in 0..10 range"
      );
    });

    it("fail - case closed", async () => {
      const asset = ASSETS.niceNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
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

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            asset.mint,
            asset.assetId,
            Category[asset.category],
            asset.risk,
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
          ),
        "308: Case closed"
      );
    });

    it("success - stolenNft", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
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

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const tx = await program.rpc.createAsset(
        asset.mint,
        asset.assetId,
        Category[asset.category],
        asset.risk,
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

      const fetchedAssetAccount = await program.account.asset.fetch(
        assetAccount
      );
      expect(fetchedAssetAccount.bump).toEqual(bump);
      expect(fetchedAssetAccount.caseId.toNumber()).toEqual(
        asset.caseId.toNumber()
      );
      expect(fetchedAssetAccount.category).toEqual(Category[asset.category]);
      expect(fetchedAssetAccount.confirmations).toEqual(0);
      expect(fetchedAssetAccount.risk).toEqual(asset.risk);
      expect(fetchedAssetAccount.community).toEqual(community.publicKey);
      expect(fetchedAssetAccount.mint).toEqual(asset.mint);
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.reporter).toEqual(reporterAccount);

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        assetAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(180);

      expect(true).toBeTruthy();
    });

    it("fail - duplicate", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
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

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            asset.mint,
            asset.assetId,
            Category[asset.category],
            asset.risk,
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
          ),
        /Error processing Instruction 0: custom program error: 0x0/
      );
    });
  });

  describe("update_asset", () => {
    it("fail - validator can't update an address", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.updateAsset(Category[asset.category], asset.risk, {
            accounts: {
              sender: reporter.publicKey,
              asset: assetAccount,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("fail - tracer can't update an address", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.updateAsset(Category[asset.category], asset.risk, {
            accounts: {
              sender: reporter.publicKey,
              asset: assetAccount,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("success", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const tx = await program.rpc.updateAsset(Category.LowRiskExchange, 8, {
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      const fetchedAssetAccount = await program.account.asset.fetch(
        assetAccount
      );
      expect(fetchedAssetAccount.caseId.toNumber()).toEqual(
        asset.caseId.toNumber()
      );
      expect(fetchedAssetAccount.category).toEqual(Category.LowRiskExchange);
      expect(fetchedAssetAccount.confirmations).toEqual(0);
      expect(fetchedAssetAccount.risk).toEqual(8);
      expect(fetchedAssetAccount.community).toEqual(community.publicKey);
      expect(fetchedAssetAccount.mint).toEqual(asset.mint);
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.reporter).toEqual(reporterAccount);
    });
  });
});
