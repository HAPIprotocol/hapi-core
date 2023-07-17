import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import {
  getReporters,
  getNetworks,
  getCases,
  getAssets,
  setupNetworks,
  setupReporters,
  setupCases,
  HAPI_CORE_TEST_ID,
} from "./util/setup";

import {
  ACCOUNT_SIZE,
  HapiCoreProgram,
  Category,
  uuidToBn,
  CaseStatus,
  decodeAddress,
} from "../lib";

describe("HapiCoreAsset ", () => {
  const program = new HapiCoreProgram(new web3.PublicKey(HAPI_CORE_TEST_ID));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "AssetMainNetwork";

  const REPORTERS = getReporters();
  const NETWORKS = getNetworks([mainNetwork]);
  const CASES = getCases();
  const ASSETS = getAssets();

  beforeAll(async () => {
    stakeToken = new TestToken(provider);
    await stakeToken.mint(1_000_000_000);

    rewardToken = new TestToken(provider);
    await rewardToken.mint(1_000_000_000);

    await setupNetworks(
      program,
      NETWORKS,
      rewardToken.mintAccount,
      stakeToken.mintAccount
    );

    await setupReporters(program, REPORTERS, mainNetwork, stakeToken);
    await setupCases(program, CASES, mainNetwork, REPORTERS.publisher);
  });

  describe("create_asset", () => {
    it("fail - validator can't report asset", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAsset(
              [...asset.address],
              [...asset.id],
              Category[asset.category],
              asset.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't report asset", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAsset(
              [...asset.address],
              [...asset.id],
              Category[asset.category],
              asset.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - risk out of range", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAsset(
              [...asset.address],
              [...asset.id],
              Category[asset.category],
              11,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("RiskOutOfRange")
      );
    });

    it("success - publisher creates first asset ", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .createAsset(
          [...asset.address],
          [...asset.id],
          Category[asset.category],
          asset.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.bump).toEqual(bump);
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.category).toEqual(Category[asset.category]);
      expect(fetchedAssetAccount.riskScore).toEqual(asset.riskScore);
      expect(fetchedAssetAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAssetAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAssetAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAssetAccount.address)).toEqual(
        decodeAddress(asset.address)
      );
      expect(decodeAddress(fetchedAssetAccount.id as number[])).toEqual(
        decodeAddress(asset.id)
      );

      const assetInfo = await provider.connection.getAccountInfoAndContext(
        assetAccount
      );
      expect(assetInfo.value.owner).toEqual(program.programId);
      expect(assetInfo.value.data).toHaveLength(ACCOUNT_SIZE.asset);
    });

    it("success - tracer creates second asset ", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.tracer;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .createAsset(
          [...asset.address],
          [...asset.id],
          Category[asset.category],
          asset.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.bump).toEqual(bump);
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.category).toEqual(Category[asset.category]);
      expect(fetchedAssetAccount.riskScore).toEqual(asset.riskScore);
      expect(fetchedAssetAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAssetAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAssetAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAssetAccount.address)).toEqual(
        decodeAddress(asset.address)
      );
      expect(decodeAddress(fetchedAssetAccount.id as number[])).toEqual(
        decodeAddress(asset.id)
      );

      const assetInfo = await provider.connection.getAccountInfoAndContext(
        assetAccount
      );
      expect(assetInfo.value.owner).toEqual(program.programId);
      expect(assetInfo.value.data).toHaveLength(ACCOUNT_SIZE.asset);
    });

    it("success - authority creates third asset ", async () => {
      const asset = ASSETS.thirdAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .createAsset(
          [...asset.address],
          [...asset.id],
          Category[asset.category],
          asset.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.bump).toEqual(bump);
      expect(fetchedAssetAccount.network).toEqual(networkAccount);
      expect(fetchedAssetAccount.category).toEqual(Category[asset.category]);
      expect(fetchedAssetAccount.riskScore).toEqual(asset.riskScore);
      expect(fetchedAssetAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAssetAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAssetAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAssetAccount.address)).toEqual(
        decodeAddress(asset.address)
      );
      expect(decodeAddress(fetchedAssetAccount.id as number[])).toEqual(
        decodeAddress(asset.id)
      );

      const assetInfo = await provider.connection.getAccountInfoAndContext(
        assetAccount
      );
      expect(assetInfo.value.owner).toEqual(program.programId);
      expect(assetInfo.value.data).toHaveLength(ACCOUNT_SIZE.asset);
    });

    it("fail - asset can be reported only once", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount, bump] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAsset(
              [...asset.address],
              [...asset.id],
              Category[asset.category],
              asset.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        / Error processing Instruction 0: custom program error: 0x0/
      );
    });
  });

  describe("update_asset", () => {
    it("fail - validator can't update asset", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAsset(Category[asset.category], asset.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't update asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.tracer;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAsset(Category[asset.category], asset.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't update asset", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAsset(Category[asset.category], asset.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - risk out of range", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAsset(Category[asset.category], 11)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("RiskOutOfRange")
      );
    });

    it("success - publisher updates first case", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .updateAsset(Category["Scam"], 10)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.category).toEqual(Category["Scam"]);
      expect(fetchedAssetAccount.riskScore).toEqual(10);
      expect(fetchedAssetAccount.caseId).toEqual(uuidToBn(cs.id));
    });

    it("success - authority updates second case", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      await program.program.methods
        .updateAsset(Category["Gambling"], 7)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.category).toEqual(Category["Gambling"]);
      expect(fetchedAssetAccount.riskScore).toEqual(7);
      expect(fetchedAssetAccount.caseId).toEqual(uuidToBn(cs.id));
    });
  });

  describe("confirm_asset", () => {
    it("fail - reporter can't confirm asset reported by himself", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't confirm asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - authority can't confirm asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't confirm asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.tracer;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - case closed", async () => {
      const asset = ASSETS.firstAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.firstCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      {
        const reporter = REPORTERS.authority;
        const [reporterAccount] = program.findReporterAddress(
          networkAccount,
          reporter.id
        );

        await program.program.methods
          .updateCase(cs.name, cs.url, CaseStatus.Closed)
          .accounts({
            sender: reporter.keypair.publicKey,
            network: networkAccount,
            reporter: reporterAccount,
            case: caseAccount,
            systemProgram: web3.SystemProgram.programId,
          })
          .signers([reporter.keypair])
          .rpc();
      }

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("CaseClosed")
      );
    });

    it("success - publisher confirms second asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      const confirmationsBefore = (
        await program.program.account.asset.fetch(assetAccount)
      ).confirmations;

      await program.program.methods
        .confirmAsset(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc(),
        programError("Unauthorized");

      const fetchedConfirmationAccount =
        await program.program.account.confirmation.fetch(confirmationAccount);

      expect(fetchedConfirmationAccount.bump).toEqual(bump);
      expect(fetchedConfirmationAccount.network).toEqual(networkAccount);
      expect(fetchedConfirmationAccount.account).toEqual(assetAccount);
      expect(
        fetchedConfirmationAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();

      let fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.confirmations).toEqual(
        confirmationsBefore + 1
      );

      const assetInfo = await provider.connection.getAccountInfoAndContext(
        confirmationAccount
      );
      expect(assetInfo.value.owner).toEqual(program.programId);
      expect(assetInfo.value.data).toHaveLength(ACCOUNT_SIZE.confirmation);
    });

    it("success - validator confirms second asset", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      const confirmationsBefore = (
        await program.program.account.asset.fetch(assetAccount)
      ).confirmations;

      await program.program.methods
        .confirmAsset(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          asset: assetAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc(),
        programError("Unauthorized");

      const fetchedConfirmationAccount =
        await program.program.account.confirmation.fetch(confirmationAccount);

      expect(fetchedConfirmationAccount.bump).toEqual(bump);
      expect(fetchedConfirmationAccount.network).toEqual(networkAccount);
      expect(fetchedConfirmationAccount.account).toEqual(assetAccount);
      expect(
        fetchedConfirmationAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();

      let fetchedAssetAccount = await program.program.account.asset.fetch(
        assetAccount
      );

      expect(fetchedAssetAccount.confirmations).toEqual(
        confirmationsBefore + 1
      );

      const assetInfo = await provider.connection.getAccountInfoAndContext(
        confirmationAccount
      );
      expect(assetInfo.value.owner).toEqual(program.programId);
      expect(assetInfo.value.data).toHaveLength(ACCOUNT_SIZE.confirmation);
    });

    it("fail - reporter can confirm asset only once", async () => {
      const asset = ASSETS.secondAsset;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      const reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const cs = CASES.secondCase;
      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const [assetAccount] = await program.findAssetAddress(
        networkAccount,
        asset.address,
        asset.id
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        assetAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAsset(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              asset: assetAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        /Error processing Instruction 0: custom program error: 0x0/
      );
    });
  });
});
