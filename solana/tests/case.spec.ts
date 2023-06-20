import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { v1 as uuidv1 } from "uuid";

import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";
import { programError } from "./util/error";
import {
  getReporters,
  getNetwotks,
  setupNetworks,
  setupReporters,
  getCases,
} from "./util/setup";

import {
  ACCOUNT_SIZE,
  HapiCoreProgram,
  CaseStatus,
  bufferFromString,
  uuidToBn,
  bnToUuid,
} from "../lib";

describe("HapiCore Reporter", () => {
  const program = new HapiCoreProgram(
    new web3.PublicKey("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk")
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  let stakeToken: TestToken;
  let rewardToken: TestToken;

  const mainNetwork = "CaseMainNetwork";

  const REPORTERS = getReporters();
  let NETWORKS = getNetwotks([mainNetwork]);
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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher.keypair;
      const [reporterAccount, __] = program.findReporterAddress(
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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.tracer;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];
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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.validator;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];
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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.appraiser;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];
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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

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
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await program.program.methods
        .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
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
      expect(fetchedCaseAccount.network).toEqual(networkAccount);
      expect(fetchedCaseAccount.state).toEqual(CaseStatus.Open);
      expect(fetchedCaseAccount.reporter).toEqual(reporterAccount);

      const caseInfo = await provider.connection.getAccountInfoAndContext(
        caseAccount
      );
      expect(caseInfo.value.owner).toEqual(program.programId);
      expect(caseInfo.value.data.length).toEqual(ACCOUNT_SIZE.case);
    });

    it("success - authority report second case", async () => {
      const cs = CASES.secondCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.authority;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const [caseAccount, bump] = await program.findCaseAddress(
        networkAccount,
        cs.id
      );

      await program.program.methods
        .createCase(uuidToBn(cs.id), cs.name, cs.url, bump)
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
      expect(fetchedCaseAccount.network).toEqual(networkAccount);
      expect(fetchedCaseAccount.state).toEqual(CaseStatus.Open);
      expect(fetchedCaseAccount.reporter).toEqual(reporterAccount);

      const caseInfo = await provider.connection.getAccountInfoAndContext(
        caseAccount
      );
      expect(caseInfo.value.owner).toEqual(program.programId);
      expect(caseInfo.value.data.length).toEqual(ACCOUNT_SIZE.case);
    });

    it("fail - case can be reported only once", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.authority;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

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
    it("tracer can't update case", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.tracer;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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

    it("validator can't update case", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.validator;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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

    it("appraiser can't update case", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.appraiser;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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

    it("reporter can't update another reporter's case", async () => {
      const cs = CASES.secondCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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

    it("publisher updates first case", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.publisher;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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
      expect(fetchedCaseAccount.state).toEqual(CaseStatus.Closed);
    });

    it("authority updates first case", async () => {
      const cs = CASES.firstCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.authority;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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
      expect(fetchedCaseAccount.state).toEqual(CaseStatus.Closed);
    });

    it("authority updates second case", async () => {
      const cs = CASES.secondCase;
      const networkAccount = program.findNetworkAddress(mainNetwork)[0];

      let reporter = REPORTERS.authority;
      const reporterAccount = program.findReporterAddress(
        networkAccount,
        reporter.id
      )[0];

      const caseAccount = await program.findCaseAddress(
        networkAccount,
        cs.id
      )[0];

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
      expect(fetchedCaseAccount.state).toEqual(CaseStatus.Closed);
    });
  });
});
