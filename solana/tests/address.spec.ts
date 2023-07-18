import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import {
  getReporters,
  getNetworks,
  getCases,
  getAddresses,
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

describe("HapiCore Address", () => {
  const program = new HapiCoreProgram(new web3.PublicKey(HAPI_CORE_TEST_ID));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "AddressMainNetwork";

  const REPORTERS = getReporters();
  const NETWORKS = getNetworks([mainNetwork]);
  const CASES = getCases();
  const ADDRESSES = getAddresses();

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

  describe("create_address", () => {
    it("fail - validator can't report address", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAddress(
              [...address.address],
              Category[address.category],
              address.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't report address", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAddress(
              [...address.address],
              Category[address.category],
              address.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - risk out of range", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAddress(
              [...address.address],
              Category[address.category],
              11,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("RiskOutOfRange")
      );
    });

    it("success - publisher creates first address ", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .createAddress(
          [...address.address],
          Category[address.category],
          address.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.category).toEqual(
        Category[address.category]
      );
      expect(fetchedAddressAccount.riskScore).toEqual(address.riskScore);
      expect(fetchedAddressAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAddressAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAddressAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAddressAccount.address)).toEqual(
        decodeAddress(address.address)
      );

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.address);
    });

    it("success - tracer creates second address ", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .createAddress(
          [...address.address],
          Category[address.category],
          address.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.category).toEqual(
        Category[address.category]
      );
      expect(fetchedAddressAccount.riskScore).toEqual(address.riskScore);
      expect(fetchedAddressAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAddressAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAddressAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAddressAccount.address)).toEqual(
        decodeAddress(address.address)
      );

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.address);
    });

    it("success - authority creates third address ", async () => {
      const address = ADDRESSES.thirdAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .createAddress(
          [...address.address],
          Category[address.category],
          address.riskScore,
          bump
        )
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.bump).toEqual(bump);
      expect(fetchedAddressAccount.network).toEqual(networkAccount);
      expect(fetchedAddressAccount.category).toEqual(
        Category[address.category]
      );
      expect(fetchedAddressAccount.riskScore).toEqual(address.riskScore);
      expect(fetchedAddressAccount.caseId.eq(uuidToBn(cs.id))).toBeTruthy();
      expect(
        fetchedAddressAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();
      expect(fetchedAddressAccount.confirmations).toEqual(0);

      expect(decodeAddress(fetchedAddressAccount.address)).toEqual(
        decodeAddress(address.address)
      );

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        addressAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.address);
    });

    it("fail - address can be reported only once", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount, bump] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createAddress(
              [...address.address],
              Category[address.category],
              address.riskScore,
              bump
            )
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        / Error processing Instruction 0: custom program error: 0x0/
      );
    });
  });

  describe("update_address", () => {
    it("fail - validator can't update address", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAddress(Category[address.category], address.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't update address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAddress(Category[address.category], address.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't update address", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAddress(Category[address.category], address.riskScore)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - risk out of range", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateAddress(Category[address.category], 11)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("RiskOutOfRange")
      );
    });

    it("success - publisher updates first case", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .updateAddress(Category["Scam"], 10)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.category).toEqual(Category["Scam"]);
      expect(fetchedAddressAccount.riskScore).toEqual(10);
      expect(fetchedAddressAccount.caseId).toEqual(uuidToBn(cs.id));
    });

    it("success - authority updates second case", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      await program.program.methods
        .updateAddress(Category["Gambling"], 7)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.category).toEqual(Category["Gambling"]);
      expect(fetchedAddressAccount.riskScore).toEqual(7);
      expect(fetchedAddressAccount.caseId).toEqual(uuidToBn(cs.id));
    });
  });

  describe("confirm_address", () => {
    it("fail - reporter can't confirm address reported by himself", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - authority can't confirm address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't confirm address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - tracer can't confirm address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - case closed", async () => {
      const address = ADDRESSES.firstAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
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
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
              confirmation: confirmationAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("CaseClosed")
      );
    });

    it("success - publisher confirms second address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      const confirmationsBefore = (
        await program.program.account.address.fetch(addressAccount)
      ).confirmations;

      await program.program.methods
        .confirmAddress(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedConfirmationAccount =
        await program.program.account.confirmation.fetch(confirmationAccount);

      expect(fetchedConfirmationAccount.bump).toEqual(bump);
      expect(fetchedConfirmationAccount.network).toEqual(networkAccount);
      expect(fetchedConfirmationAccount.account).toEqual(addressAccount);
      expect(
        fetchedConfirmationAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();

      let fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.confirmations).toEqual(
        confirmationsBefore + 1
      );

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        confirmationAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.confirmation);
    });

    it("success - validator confirms second address", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      const confirmationsBefore = (
        await program.program.account.address.fetch(addressAccount)
      ).confirmations;

      await program.program.methods
        .confirmAddress(bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          address: addressAccount,
          confirmation: confirmationAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedConfirmationAccount =
        await program.program.account.confirmation.fetch(confirmationAccount);

      expect(fetchedConfirmationAccount.bump).toEqual(bump);
      expect(fetchedConfirmationAccount.network).toEqual(networkAccount);
      expect(fetchedConfirmationAccount.account).toEqual(addressAccount);
      expect(
        fetchedConfirmationAccount.reporterId.eq(uuidToBn(reporter.id))
      ).toBeTruthy();

      let fetchedAddressAccount = await program.program.account.address.fetch(
        addressAccount
      );

      expect(fetchedAddressAccount.confirmations).toEqual(
        confirmationsBefore + 1
      );

      const addressInfo = await provider.connection.getAccountInfoAndContext(
        confirmationAccount
      );
      expect(addressInfo.value.owner).toEqual(program.programId);
      expect(addressInfo.value.data).toHaveLength(ACCOUNT_SIZE.confirmation);
    });

    it("fail - reporter can confirm address only once", async () => {
      const address = ADDRESSES.secondAddress;
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

      const [addressAccount] = await program.findAddressAddress(
        networkAccount,
        address.address
      );

      const [confirmationAccount, bump] = await program.findConfirmationAddress(
        addressAccount,
        reporter.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .confirmAddress(bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              address: addressAccount,
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
