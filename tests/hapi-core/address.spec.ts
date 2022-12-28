import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken } from "../util/token";
import { expectThrowError, listenSolanaLogs } from "../util/console";
import {
  ACCOUNT_SIZE,
  bufferFromString,
  CaseStatus,
  Category,
  initHapiCore,
  NetworkSchema,
  NetworkSchemaKeys,
  padBuffer,
  ReporterRole,
} from "../../lib";
import { programError } from "../util/error";
import { metadata } from "../../target/idl/hapi_core.json";

describe("HapiCore Address", () => {
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
      reportPrice: BN
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
    newCase: {
      network: "ethereum",
      caseId: new BN(3),
      name: "new case",
      reporter: "alice",
    },
  };

  const ADDRESSES: Record<
    string,
    {
      pubkey: Buffer;
      network: keyof typeof NETWORKS;
      category: keyof typeof Category;
      reporter: keyof typeof REPORTERS;
      caseId: BN;
      risk: number;
    }
  > = {
    blackhole1: {
      pubkey: Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000000001",
        "hex"
      ),
      network: "ethereum",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
    blackhole2: {
      pubkey: Buffer.from(
        "0000000000000000000000000000000000000000000000000000000000000001",
        "hex"
      ),
      network: "solana",
      category: "None",
      reporter: "alice",
      caseId: new BN(1),
      risk: 0,
    },
    nftMerchant: {
      pubkey: Buffer.from(
        "6923f8792e9b41a2cc735d4c995b20c8d717cfda8d30e216fe1857389da71c94",
        "hex"
      ),
      network: "ethereum",
      reporter: "bob",
      category: "MerchantService",
      caseId: new BN(2),
      risk: 2,
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
        lamports: 1_000_000_000,
      }),
      ...Object.keys(REPORTERS).map((key) =>
        web3.SystemProgram.transfer({
          fromPubkey: authority.publicKey,
          toPubkey: REPORTERS[key].keypair.publicKey,
          lamports: 1_000_000_000,
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

  describe("create_address", () => {
    it("fail - risk range", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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
          program.rpc.createAddress(
            [...addr.pubkey],
            Category[addr.category],
            100,
            bump,
            {
              accounts: {
                sender: reporter.publicKey,
                address: addressAccount,
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
      const addr = ADDRESSES.nftMerchant;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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
          program.rpc.createAddress(
            [...addr.pubkey],
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

    it("success - blackhole1", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const tx = await program.rpc.createAddress(
        [...addr.pubkey],
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
            reporterPaymentTokenAccount,
            treasuryTokenAccount: communityTreasuryTokenAccount,
            tokenProgram: stakeToken.programId,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedAddressAccount = await program.account.address.fetch(
        addressAccount
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

      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.caseId.toNumber()).toEqual(
        addr.caseId.toNumber()
      );
      expect(fetchedAddressAccount.category).toEqual(Category[addr.category]);
      expect(fetchedAddressAccount.confirmations).toEqual(0);
      expect(fetchedAddressAccount.risk).toEqual(addr.risk);
      expect(fetchedAddressAccount.community).toEqual(community.publicKey);
      expect(Buffer.from(fetchedAddressAccount.address)).toEqual(
        padBuffer(addr.pubkey, 64)
      );
      expect(
        program.util.decodeAddress(fetchedAddressAccount.address, "Ethereum")
      ).toEqual("0x0000000000000000000000000000000000000000");
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.reporter).toEqual(reporterAccount);

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.address);

      expect(
        reporterBalanceBefore.sub(reporterBalanceAfter).toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());

      expect(
        treasuryCommunityBalance.toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());

      expect(true).toBeTruthy();
    });

    it("success - blackhole2", async () => {
      const addr = ADDRESSES.blackhole2;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const tx = await program.rpc.createAddress(
        [...addr.pubkey],
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
            reporterPaymentTokenAccount,
            treasuryTokenAccount: communityTreasuryTokenAccount,
            tokenProgram: stakeToken.programId,
            systemProgram: web3.SystemProgram.programId,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedAddressAccount = await program.account.address.fetch(
        addressAccount
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

      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.caseId.toNumber()).toEqual(
        addr.caseId.toNumber()
      );
      expect(fetchedAddressAccount.category).toEqual(Category[addr.category]);
      expect(fetchedAddressAccount.confirmations).toEqual(0);
      expect(fetchedAddressAccount.risk).toEqual(addr.risk);
      expect(fetchedAddressAccount.community).toEqual(community.publicKey);
      expect(Buffer.from(fetchedAddressAccount.address)).toEqual(
        padBuffer(addr.pubkey, 64)
      );
      expect(
        program.util.decodeAddress(fetchedAddressAccount.address, "Solana")
      ).toEqual("11111111111111111111111111111112");
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.reporter).toEqual(reporterAccount);

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.address);

      expect(
        reporterBalanceBefore.sub(reporterBalanceAfter).toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());

      expect(
        treasuryCommunityBalance.toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());

      expect(true).toBeTruthy();
    });

    it("fail - duplicate", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount, bump] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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
          program.rpc.createAddress(
            [...addr.pubkey],
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

  describe("update_address", () => {
    it("fail - validator can't update an address", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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
          program.rpc.updateAddress(Category[addr.category], addr.risk, {
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
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
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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
          program.rpc.updateAddress(Category[addr.category], addr.risk, {
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
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
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const reporterBalanceBefore = new BN(
        (
          await provider.connection.getTokenAccountBalance(reporterPaymentTokenAccount)
        ).value.amount,
        10
      );

      const tx = await program.rpc.updateAddress(Category.Gambling, 8, {
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
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

      const fetchedAddressAccount = await program.account.address.fetch(
        addressAccount
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

      expect(fetchedAddressAccount.caseId.toNumber()).toEqual(
        addr.caseId.toNumber()
      );
      expect(fetchedAddressAccount.category).toEqual(Category.Gambling);
      expect(fetchedAddressAccount.confirmations).toEqual(0);
      expect(fetchedAddressAccount.risk).toEqual(8);
      expect(fetchedAddressAccount.community).toEqual(community.publicKey);
      expect(Buffer.from(fetchedAddressAccount.address)).toEqual(
        padBuffer(addr.pubkey, 64)
      );
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.reporter).toEqual(reporterAccount);

      expect(
        reporterBalanceBefore.sub(reporterBalanceAfter).toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());

      expect(
        treasuryCommunityBalance.toNumber()
      ).toEqual(NETWORKS[addr.network].reportPrice.toNumber());
    });
  });

  describe("confirm_address", () => {
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

    it("fail - can't confirm an address reported by yourself", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const addressInfo = await program.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.confirmAddress({
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              reporterReward: reporterRewardAccount,
              addressReporterReward: addressReporterRewardAccount,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        programError("Unauthorized")
      );
    });

    it("success - bob", async () => {
      const l = listenSolanaLogs(provider.connection);

      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const addressInfo = await program.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const tx = await program.rpc.confirmAddress({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          addressReporterReward: addressReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      l.close();

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.address.fetch(
          addressAccount
        );
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          addr.caseId.toNumber()
        );
        expect(fetchedAccount.category).toEqual(Category.Gambling);
        expect(fetchedAccount.confirmations).toEqual(1);
        expect(fetchedAccount.risk).toEqual(8);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.address)).toEqual(
          padBuffer(addr.pubkey, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );

        console.log("!!!!!!!!!!!!!!!!!!!!!!!!!!");
        console.log(fetchedAccount.addressConfirmationCounter);
        

        expect(
          fetchedAccount.addressConfirmationCounter.eqn(1)
        ).toBeTruthy();
        expect(
          fetchedAccount.addressTracerCounter.eqn(0)
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          addressReporterRewardAccount
        );

        expect(
          fetchedAccount.addressConfirmationCounter.toNumber()
        ).toEqual(0);
        expect(
          fetchedAccount.addressTracerCounter.toNumber()
        ).toEqual(0);
      }
    });

    it("success - dave", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const addressInfo = await program.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const tx = await program.rpc.confirmAddress({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          addressReporterReward: addressReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.address.fetch(
          addressAccount
        );
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          addr.caseId.toNumber()
        );
        expect(fetchedAccount.category).toEqual(Category.Gambling);
        expect(fetchedAccount.confirmations).toEqual(2);
        expect(fetchedAccount.risk).toEqual(8);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.address)).toEqual(
          padBuffer(addr.pubkey, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );
        expect(
          fetchedAccount.addressConfirmationCounter.eqn(1)
        ).toBeTruthy();
        expect(
          fetchedAccount.addressTracerCounter.eqn(0)
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          addressReporterRewardAccount
        );
        expect(
          fetchedAccount.addressConfirmationCounter.toNumber()
        ).toEqual(0);
        expect(
          fetchedAccount.addressTracerCounter.toNumber()
        ).toEqual(1);
      }
    });

    it("success - carol", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS.carol.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
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

      const addressInfo = await program.account.address.fetch(addressAccount);

      const [addressReporterRewardAccount] =
        await program.pda.findReporterRewardAddress(
          networkAccount,
          addressInfo.reporter
        );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      const tx = await program.rpc.confirmAddress({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          reporterReward: reporterRewardAccount,
          addressReporterReward: addressReporterRewardAccount,
          case: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      {
        const fetchedAccount = await program.account.address.fetch(
          addressAccount
        );
        expect(fetchedAccount.caseId.toNumber()).toEqual(
          addr.caseId.toNumber()
        );
        expect(fetchedAccount.category).toEqual(Category.Gambling);
        expect(fetchedAccount.confirmations).toEqual(3);
        expect(fetchedAccount.risk).toEqual(8);
        expect(fetchedAccount.community).toEqual(community.publicKey);
        expect(Buffer.from(fetchedAccount.address)).toEqual(
          padBuffer(addr.pubkey, 64)
        );
        expect(fetchedAccount.network).toEqual(networkAccount);
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          reporterRewardAccount
        );
        expect(
          fetchedAccount.addressConfirmationCounter.eqn(0)
        ).toBeTruthy();
        expect(
          fetchedAccount.addressTracerCounter.eqn(0)
        ).toBeTruthy();
      }

      {
        const fetchedAccount = await program.account.reporterReward.fetch(
          addressReporterRewardAccount
        );
        expect(
          fetchedAccount.addressConfirmationCounter.toNumber()
        ).toEqual(0);
        expect(
          fetchedAccount.addressTracerCounter.toNumber()
        ).toEqual(0);
      }
    });
  });

  describe("change_address_case", () => {
    it("fail - validator can't update an address", async () => {
      const addr = ADDRESSES.blackhole1;
      const cs = CASES.newCase;

      const reporter = REPORTERS.dave.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        cs.caseId
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
          program.rpc.updateAddress(Category[addr.category], addr.risk, {
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
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
      const addr = ADDRESSES.blackhole1;
      const cs = CASES.newCase;

      const reporter = REPORTERS.bob.keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        cs.caseId
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
          program.rpc.updateAddress(Category[addr.category], addr.risk, {
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
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

    it("fail - same case can not be provided", async () => {
      const addr = ADDRESSES.blackhole1;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        addr.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.changeAddressCase({
            accounts: {
              sender: reporter.publicKey,
              address: addressAccount,
              community: community.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              newCase: caseAccount,
            },
            signers: [reporter],
          }),
        programError("SameCase")
      );
    });

    it("success", async () => {
      const addr = ADDRESSES.blackhole1;
      const cs = CASES.newCase;

      const reporter = REPORTERS[addr.reporter].keypair;

      const [networkAccount] = await program.pda.findNetworkAddress(
        community.publicKey,
        addr.network
      );

      const [addressAccount] = await program.pda.findAddressAddress(
        networkAccount,
        addr.pubkey
      );

      const [reporterAccount] = await program.pda.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.pda.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const tx = await program.rpc.changeAddressCase({
        accounts: {
          sender: reporter.publicKey,
          address: addressAccount,
          community: community.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          newCase: caseAccount,
        },
        signers: [reporter],
      });

      expect(tx).toBeTruthy();

      const fetchedAddressAccount = await program.account.address.fetch(
        addressAccount
      );
      expect(fetchedAddressAccount.caseId.toNumber()).toEqual(
        cs.caseId.toNumber()
      );
    });
  });
});
