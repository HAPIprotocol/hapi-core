import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken } from "../util/token";
import { expectThrowError } from "../util/console";
import {
  ACCOUNT_SIZE,
  bufferFromString,
  CaseStatus,
  toNativeBn,
  Category,
  initHapiCore,
  NetworkSchema,
  NetworkSchemaKeys,
  padBuffer,
  ReporterRole,
} from "../../lib";
import { programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";

describe("HapiCore Asset", () => {
  const program = initHapiCore(new web3.PublicKey(metadata.address));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const authority = provider.wallet;

  const nobody = web3.Keypair.generate();
  const community = web3.Keypair.generate();

  let stakeToken: TestToken;

  const REPORTERS: Record<
    string,
    { name: string; keypair: web3.Keypair; role: keyof typeof ReporterRole }
  > = {
    alice: {
      name: "alice",
      keypair: web3.Keypair.generate(),
      role: "Publisher",
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

  const NETWORKS: Record<
    string,
    {
      name: string;
      schema: NetworkSchemaKeys;
      rewardToken: TestToken;
      addressTracerReward: BN;
      addressConfirmationReward: BN;
      assetTracerReward: BN;
      assetConfirmationReward: BN;
      reportPrice: BN;
    }
  > = {
    ethereum: {
      name: "ethereum",
      schema: "Ethereum",
      rewardToken: new TestToken(provider),
      addressTracerReward: new BN(1_000),
      addressConfirmationReward: new BN(2_000),
      assetTracerReward: new BN(3_000),
      assetConfirmationReward: new BN(4_000),
      reportPrice: new BN(1_000),
    },
    solana: {
      name: "solana",
      schema: "Solana",
      rewardToken: new TestToken(provider),
      addressTracerReward: new BN(1_001),
      addressConfirmationReward: new BN(2_001),
      assetTracerReward: new BN(3_001),
      assetConfirmationReward: new BN(4_001),
      reportPrice: new BN(1_001),
    },
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
      mint: Buffer;
      assetId: Buffer;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
    }
  > = {
    stolenNft: {
      mint: Buffer.from(
        "2873d85250e84e093c3f38c78e74c060c834db3cdaa4c09b4ed6aea9718959a8",
        "hex"
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
      mint: Buffer.from(
        "227b144f6d3dafb46cb632f12a260fac968455be71e289102ead3f95db7685bf",
        "hex"
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
    const wait: Promise<unknown>[] = [];

    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);
    wait.push(stakeToken.transfer(null, nobody.publicKey, 1_000_000));

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

    wait.push(provider.sendAndConfirm(tx));

    for (const reporter of Object.keys(REPORTERS)) {
      wait.push(
        stakeToken.transfer(
          null,
          REPORTERS[reporter].keypair.publicKey,
          1_000_000
        )
      );
    }

    const [tokenSignerAccount, tokenSignerBump] =
      await program.pda.findCommunityTokenSignerAddress(community.publicKey);

    const communityTokenAccount = await stakeToken.createAccount(
      tokenSignerAccount
    );
    const communityTreasuryTokenAccount = await stakeToken.createAccount(tokenSignerAccount);

    wait.push(
      program.rpc.initializeCommunity(
        new BN(10),
        2,
        new BN(1_000),
        new BN(2_000),
        new BN(3_000),
        new BN(4_000),
        tokenSignerBump,
        {
          accounts: {
            authority: authority.publicKey,
            community: community.publicKey,
            stakeMint: stakeToken.mintAccount,
            tokenAccount: communityTokenAccount,
            treasuryTokenAccount: communityTreasuryTokenAccount,
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

      const [reporterAccount, bump] = await program.pda.findReporterAddress(
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

      await network.rewardToken.mint();

      const [networkAccount, bump] = await program.pda.findNetworkAddress(
        community.publicKey,
        network.name
      );

      const [rewardSignerAccount, rewardSignerBump] =
        await program.pda.findNetworkRewardSignerAddress(networkAccount);

      wait.push(
        program.rpc.createNetwork(
          bufferFromString(network.name, 32).toJSON().data,
          NetworkSchema[network.schema],
          network.addressTracerReward,
          network.addressConfirmationReward,
          network.assetTracerReward,
          network.assetConfirmationReward,
          bump,
          rewardSignerBump,
          network.reportPrice,
          {
            accounts: {
              authority: authority.publicKey,
              community: community.publicKey,
              network: networkAccount,
              rewardMint: network.rewardToken.mintAccount,
              rewardSigner: rewardSignerAccount,
              tokenProgram: network.rewardToken.programId,
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
          const [reporterAccount] = await program.pda.findReporterAddress(
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

      const [caseAccount, bump] = await program.pda.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
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

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
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

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [assetAccount, bump] = await program.pda.findAssetAddress(
        networkAccount,
        addr.mint,
        addr.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            [...addr.mint],
            [...addr.assetId],
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
                reporterPaymentTokenAccount,
                treasuryTokenAccount: communityTreasuryTokenAccount,
                tokenProgram: stakeToken.programId,
                systemProgram: web3.SystemProgram.programId,
              },
              signers: [reporter],
            }
          ),
        programError("RiskOutOfRange")
      );
    });

    it("fail - case closed", async () => {
      const asset = ASSETS.niceNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount, bump] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            [...asset.mint],
            [...asset.assetId],
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
                reporterPaymentTokenAccount,
                treasuryTokenAccount: communityTreasuryTokenAccount,
                tokenProgram: stakeToken.programId,
                systemProgram: web3.SystemProgram.programId,
              },
              signers: [reporter],
            }
          ),
        programError("CaseClosed")
      );
    });

    it("success - stolenNft", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount, bump] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
      );

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const tx = await program.rpc.createAsset(
        [...asset.mint],
        [...asset.assetId],
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
            reporterPaymentTokenAccount,
            treasuryTokenAccount: communityTreasuryTokenAccount,
            tokenProgram: stakeToken.programId,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedAssetAccount = await program.account.asset.fetch(
        assetAccount
      );

      const reporterBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const treasuryCommunityBalance = new BN(
        (
          await provider.connection.getTokenAccountBalance(communityTreasuryTokenAccount)
        ).value.amount,
        10
      );

      let reportPrice = NETWORKS[asset.network].reportPrice.toNumber();

      expect(fetchedAssetAccount.bump).toEqual(bump);
      expect(fetchedAssetAccount.caseId.toNumber()).toEqual(
        asset.caseId.toNumber()
      );
      expect(fetchedAssetAccount.category).toEqual(Category[asset.category]);
      expect(fetchedAssetAccount.confirmations).toEqual(0);
      expect(fetchedAssetAccount.risk).toEqual(asset.risk);
      expect(fetchedAssetAccount.community).toEqual(community.publicKey);
      expect(Buffer.from(fetchedAssetAccount.mint)).toEqual(
        padBuffer(asset.mint, 64)
      );
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.reporter).toEqual(reporterAccount);
      expect(fetchedAssetAccount.replicationBounty.toNumber()).toEqual(reportPrice);

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        assetAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.asset);

      expect(true).toBeTruthy();

      expect(
        reporterBalanceBefore.sub(reporterBalanceAfter).toNumber()
      ).toEqual(reportPrice);

      expect(
        treasuryCommunityBalance.toNumber()
      ).toEqual(reportPrice);
    });

    it("fail - duplicate", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount, bump] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createAsset(
            [...asset.mint],
            [...asset.assetId],
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
                reporterPaymentTokenAccount,
                treasuryTokenAccount: communityTreasuryTokenAccount,
                tokenProgram: stakeToken.programId,
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

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
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
              reporterPaymentTokenAccount,
              treasuryTokenAccount: communityTreasuryTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter],
          }),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't update an address", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
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
              reporterPaymentTokenAccount,
              treasuryTokenAccount: communityTreasuryTokenAccount,
              tokenProgram: stakeToken.programId,
            },
            signers: [reporter],
          }),
        programError("Unauthorized")
      );
    });

    it("success", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const communityInfo = await program.account.community.fetch(
        community.publicKey
      );

      const communityTreasuryTokenAccount = await stakeToken.createAccount(communityInfo.tokenSigner);

      const reporterPaymentTokenAccount = await stakeToken.getTokenAccount(
        reporter.publicKey
      );

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const fetchedAssetAccountBefore = await program.account.asset.fetch(
        assetAccount
      );

      const replicationBountyBefore = fetchedAssetAccountBefore.replicationBounty.toNumber();

      const tx = await program.rpc.updateAsset(Category.Exchange, 8, {
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          reporterPaymentTokenAccount,
          treasuryTokenAccount: communityTreasuryTokenAccount,
          tokenProgram: stakeToken.programId,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      const fetchedAssetAccount = await program.account.asset.fetch(
        assetAccount
      );

      const reporterBalanceAfter = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const treasuryCommunityBalance = new BN(
        (
          await provider.connection.getTokenAccountBalance(communityTreasuryTokenAccount)
        ).value.amount,
        10
      );

      let reportPrice = NETWORKS[asset.network].reportPrice.toNumber();

      expect(fetchedAssetAccount.caseId.toNumber()).toEqual(
        asset.caseId.toNumber()
      );
      expect(fetchedAssetAccount.category).toEqual(Category.Exchange);
      expect(fetchedAssetAccount.confirmations).toEqual(0);
      expect(fetchedAssetAccount.risk).toEqual(8);
      expect(fetchedAssetAccount.community).toEqual(community.publicKey);
      expect(Buffer.from(fetchedAssetAccount.mint)).toEqual(
        padBuffer(asset.mint, 64)
      );
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.reporter).toEqual(reporterAccount);
      expect(fetchedAssetAccount.replicationBounty.toNumber()).toEqual(replicationBountyBefore + reportPrice);

      expect(
        reporterBalanceBefore.sub(reporterBalanceAfter).toNumber()
      ).toEqual(reportPrice);

      expect(
        treasuryCommunityBalance.toNumber()
      ).toEqual(reportPrice);
    });
  });

  describe("confirm_asset", () => {
    beforeAll(async () => {
      for (const reporterKey of Object.keys(REPORTERS)) {
        const reporter = REPORTERS[reporterKey];

        const [reporterAccount] = await program.pda.findReporterAddress(
          community.publicKey,
          reporter.keypair.publicKey
        );

        for (const networkKey of Object.keys(NETWORKS)) {
          const [networkAccount] = await program.pda.findNetworkAddress(
            community.publicKey,
            NETWORKS[networkKey].name
          );

          const [reporterRewardAccount, bump] =
            await program.pda.findReporterRewardAddress(
              networkAccount,
              reporterAccount
            );

          await program.rpc.initializeReporterReward(bump, {
            accounts: {
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              reporterReward: reporterRewardAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [reporter.keypair],
          });
        }
      }
    });

    it("fail - can't confirm an asset reported by yourself", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS[asset.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const assetInfo = await program.account.asset.fetch(assetAccount);

      const [assetReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          assetInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.confirmAsset({
            accounts: {
              sender: reporter.publicKey,
              asset: assetAccount,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              reporterReward: reporterRewardAccount,
              assetReporterReward: assetReporterRewardAccount,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        programError("Unauthorized")
      );
    });

    it("success - bob", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const assetInfo = await program.account.asset.fetch(assetAccount);

      const [assetReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          assetInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const tx = await program.rpc.confirmAsset({
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          assetReporterReward: assetReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.asset.fetch(assetAccount);
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          asset.caseId.toNumber()
        );
        expect(fetchedAccount.confirmations).toEqual(1);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.mint)).toEqual(
          padBuffer(asset.mint, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );

        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).eq(new BN(1))
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).isZero()
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          assetReporterRewardAccount
        );
        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).isZero()
        ).toBeTruthy();
      }
    });

    it("success - dave", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const assetInfo = await program.account.asset.fetch(assetAccount);

      const [assetReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          assetInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const tx = await program.rpc.confirmAsset({
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          assetReporterReward: assetReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.asset.fetch(assetAccount);
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          asset.caseId.toNumber()
        );
        expect(fetchedAccount.confirmations).toEqual(2);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.mint)).toEqual(
          padBuffer(asset.mint, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );

        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).eq(new BN(1))
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).isZero()
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          assetReporterRewardAccount
        );
        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).eq(new BN(1))
        ).toBeTruthy();
      }
    });

    it("success - carol", async () => {
      const asset = ASSETS.stolenNft;

      const reporter = REPORTERS.carol.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        asset.network
      );

      const [assetAccount] = await program.pda.findAssetAddress(
        networkAccount,
        asset.mint,
        asset.assetId
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [reporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          reporterAccount
        );

      const assetInfo = await program.account.asset.fetch(assetAccount);

      const [assetReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          assetInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        asset.caseId
      );

      const tx = await program.rpc.confirmAsset({
        accounts: {
          sender: reporter.publicKey,
          asset: assetAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          assetReporterReward: assetReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.asset.fetch(assetAccount);
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          asset.caseId.toNumber()
        );
        expect(fetchedAccount.confirmations).toEqual(3);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.mint)).toEqual(
          padBuffer(asset.mint, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );

        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).eq(new BN(1))
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).isZero()
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          assetReporterRewardAccount
        );
        expect(
          toNativeBn(fetchedAccount.addressConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.addressTracerCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetConfirmationCounter).isZero()
        ).toBeTruthy();
        expect(
          toNativeBn(fetchedAccount.assetTracerCounter).eq(new BN(1))
        ).toBeTruthy();
      }
    });
  });
});
