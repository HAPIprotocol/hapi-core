import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { v1 as uuidv1 } from "uuid";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import {
  getReporters,
  getNetworks,
  setupNetworks,
  setupReporters,
  getCases,
  HAPI_CORE_TEST_ID,
} from "./util/setup";

import {
  ACCOUNT_SIZE,
  HapiCoreProgram,
  CaseStatus,
  bufferFromString,
  uuidToBn,
} from "../lib";

describe("HapiCore Case", () => {
  const program = new HapiCoreProgram(new web3.PublicKey(HAPI_CORE_TEST_ID));

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "CaseMainNetwork";

  const REPORTERS = getReporters();
  const NETWORKS = getNetworks([mainNetwork]);
  const CASES = getCases();

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
  });

  describe("create_case", () => {
    it("fail - unknown reporter", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher.keypair;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        REPORTERS.authority.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
            .accounts({
              sender: reporter.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter])
            .rpc(),
        programError("InvalidReporter")
      );
    });

    it("fail - tracers can't report cases", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.tracer;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - validator can't report cases", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't report cases", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );
      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - invalid case id", async () => {
      const caseId = bufferFromString("invalid-id", 16);
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = web3.PublicKey.findProgramAddressSync(
        [bufferFromString("case"), networkAccount.toBytes(), caseId],
        program.programId
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(new BN(caseId, "be"), "name", "url", bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("InvalidUUID")
      );
    });

    it("fail - invalid case id version", async () => {
      const caseId = uuidv1();
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        caseId
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(caseId), "name", "url", bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("InvalidUUID")
      );
    });

    it("success - publisher report first case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const id = uuidToBn(cs.id);

      await program.program.methods
        .createCase(id, cs.name, cs.url, bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedCaseAccount = await program.program.account.case.fetch(
        caseAccount
      );

      expect((fetchedCaseAccount.id as BN).eq(id)).toBeTruthy();
      expect(fetchedCaseAccount.name).toEqual(cs.name);
      expect(fetchedCaseAccount.url).toEqual(cs.url);
      expect(fetchedCaseAccount.network).toEqual(networkAccount);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Open);
      expect(fetchedCaseAccount.reporterId).toEqual(uuidToBn(reporter.id));

      const caseInfo = await provider.connection.getAccountInfoAndContext(
        caseAccount
      );
      expect(caseInfo.value.owner).toEqual(program.programId);
      expect(caseInfo.value.data.length).toEqual(ACCOUNT_SIZE.case);
    });

    it("success - authority report second case", async () => {
      const cs = CASES.secondCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      const id = uuidToBn(cs.id);

      await program.program.methods
        .createCase(id, cs.name, cs.url, bump)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedCaseAccount = await program.program.account.case.fetch(
        caseAccount
      );

      expect((fetchedCaseAccount.id as BN).eq(id)).toBeTruthy();
      expect(fetchedCaseAccount.name).toEqual(cs.name);
      expect(fetchedCaseAccount.url).toEqual(cs.url);
      expect(fetchedCaseAccount.network).toEqual(networkAccount);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Open);
      expect(fetchedCaseAccount.reporterId).toEqual(uuidToBn(reporter.id));

      const caseInfo = await provider.connection.getAccountInfoAndContext(
        caseAccount
      );
      expect(caseInfo.value.owner).toEqual(program.programId);
      expect(caseInfo.value.data.length).toEqual(ACCOUNT_SIZE.case);
    });

    it("fail - case can be reported only once", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        / Error processing Instruction 0: custom program error: 0x0/
      );
    });
  });

  describe("update_case", () => {
    it("fail - tracer can't update case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.tracer;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateCase(cs.name, cs.url, CaseStatus.Closed)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - validator can't update case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.validator;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateCase(cs.name, cs.url, CaseStatus.Closed)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - appraiser can't update case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.appraiser;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateCase(cs.name, cs.url, CaseStatus.Closed)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("fail - reporter can't update another reporter's case", async () => {
      const cs = CASES.secondCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await expectThrowError(
        () =>
          program.program.methods
            .updateCase(cs.name, cs.url, CaseStatus.Closed)
            .accounts({
              sender: reporter.keypair.publicKey,
              network: networkAccount,
              reporter: reporterAccount,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            })
            .signers([reporter.keypair])
            .rpc(),
        programError("Unauthorized")
      );
    });

    it("success - publisher updates first case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.publisher;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
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

      const fetchedCaseAccount = await program.program.account.case.fetch(
        caseAccount
      );

      expect(fetchedCaseAccount.name).toEqual(cs.name);
      expect(fetchedCaseAccount.url).toEqual(cs.url);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Closed);
    });

    it("success - authority updates first case", async () => {
      const cs = CASES.firstCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      let newName = "new name";

      await program.program.methods
        .updateCase(newName, cs.url, CaseStatus.Closed)
        .accounts({
          sender: reporter.keypair.publicKey,
          network: networkAccount,
          reporter: reporterAccount,
          case: caseAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([reporter.keypair])
        .rpc();

      const fetchedCaseAccount = await program.program.account.case.fetch(
        caseAccount
      );

      expect(fetchedCaseAccount.name).toEqual(newName);
      expect(fetchedCaseAccount.url).toEqual(cs.url);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Closed);
    });

    it("success - authority updates second case", async () => {
      const cs = CASES.secondCase;
      const [networkAccount] = program.findNetworkAddress(mainNetwork);

      let reporter = REPORTERS.authority;
      const [reporterAccount] = program.findReporterAddress(
        networkAccount,
        reporter.id
      );

      const [caseAccount] = await program.findCaseAddress(
        networkAccount,
        cs.id
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

      const fetchedCaseAccount = await program.program.account.case.fetch(
        caseAccount
      );

      expect(fetchedCaseAccount.name).toEqual(cs.name);
      expect(fetchedCaseAccount.url).toEqual(cs.url);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Closed);
    });
  });
});
