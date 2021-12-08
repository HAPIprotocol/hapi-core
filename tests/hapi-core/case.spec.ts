import * as anchor from "@project-serum/anchor";
import { web3, BN } from "@project-serum/anchor";

import { TestToken, u64 } from "../util/token";
import { expectThrowError } from "../util/console";
import { bufferFromString, CaseStatus, program, ReporterRole } from "../../lib";

jest.setTimeout(10_000);

describe("HapiCore Case", () => {
  const provider = anchor.Provider.env();
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

  const CASES: Record<
    string,
    {
      caseId: BN;
      name: string;
      reporter: keyof typeof REPORTERS;
    }
  > = {
    safe: {
      caseId: new BN(1),
      name: "safe network addresses",
      reporter: "alice",
    },
    nftTracking: {
      caseId: new BN(2),
      name: "suspicious nft txes",
      reporter: "carol",
    },
  };

  beforeAll(async () => {
    let wait: Promise<unknown>[] = [];

    stakeToken = new TestToken(provider);
    await stakeToken.mint(new u64(1_000_000_000));
    wait.push(stakeToken.transfer(null, nobody.publicKey, new u64(1_000_000)));

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
  });

  describe("create_case", () => {
    it("fail - non-whitelisted reporter can't creport cases", async () => {
      const cs = CASES.safe;

      const reporter = { keypair: nobody };

      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [reporter.keypair],
          }),
        "167: The given account is not owned by the executing program"
      );
    });

    it("fail - reporter impersonation", async () => {
      const cs = CASES.safe;

      const reporter = { keypair: nobody };

      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        REPORTERS.alice.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [reporter.keypair],
          }),
        "305: Invalid reporter account"
      );
    });

    it("fail - tracers can't report cases", async () => {
      const cs = CASES.safe;

      const reporter = REPORTERS.bob;

      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [reporter.keypair],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("fail - validators can't report cases", async () => {
      const cs = CASES.safe;

      const reporter = REPORTERS.dave;

      const caseName = bufferFromString(cs.name, 32);

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.keypair.publicKey
      );

      await expectThrowError(
        () =>
          program.rpc.createCase(cs.caseId, caseName.toJSON().data, bump, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.keypair.publicKey,
              community: community.publicKey,
              case: caseAccount,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [reporter.keypair],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("success - alice reports case 'safe'", async () => {
      const cs = CASES.safe;

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

      expect(true).toBeTruthy();
    });

    it("success - carol reports case 'nftTracking'", async () => {
      const cs = CASES.nftTracking;

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

      expect(true).toBeTruthy();
    });
  });

  describe("update_case", () => {
    it("success - 'safe'", async () => {
      const cs = CASES.safe;

      const reporter = REPORTERS[cs.reporter].keypair;
      const newCaseName = bufferFromString("new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const tx = await program.rpc.updateCase(
        newCaseName.toJSON().data,
        CaseStatus.Closed,
        {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
            case: caseAccount,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedCaseAccount = await program.account.case.fetch(caseAccount);
      expect(Buffer.from(fetchedCaseAccount.name)).toEqual(newCaseName);
      expect(fetchedCaseAccount.bump).toEqual(bump);
      expect(fetchedCaseAccount.reporter).toEqual(reporterAccount);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Closed);
      expect(fetchedCaseAccount.id.toNumber()).toEqual(cs.caseId.toNumber());
    });

    it("success - 'safe' by authority", async () => {
      const cs = CASES.safe;

      const reporter = REPORTERS.carol.keypair;
      const newCaseName = bufferFromString("super_new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount, bump] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const tx = await program.rpc.updateCase(
        newCaseName.toJSON().data,
        CaseStatus.Open,
        {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
            case: caseAccount,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedCaseAccount = await program.account.case.fetch(caseAccount);
      expect(Buffer.from(fetchedCaseAccount.name)).toEqual(newCaseName);
      expect(fetchedCaseAccount.bump).toEqual(bump);
      expect(fetchedCaseAccount.reporter).not.toEqual(reporterAccount);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Open);
      expect(fetchedCaseAccount.id.toNumber()).toEqual(cs.caseId.toNumber());
    });

    it("success - 'nftTracking'", async () => {
      const cs = CASES.nftTracking;

      const reporter = REPORTERS[cs.reporter].keypair;
      const newCaseName = bufferFromString("new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      const tx = await program.rpc.updateCase(
        newCaseName.toJSON().data,
        CaseStatus.Closed,
        {
          accounts: {
            reporter: reporterAccount,
            sender: reporter.publicKey,
            community: community.publicKey,
            case: caseAccount,
          },
          signers: [reporter],
        }
      );

      expect(tx).toBeTruthy();

      const fetchedCaseAccount = await program.account.case.fetch(caseAccount);
      expect(Buffer.from(fetchedCaseAccount.name)).toEqual(newCaseName);
      expect(fetchedCaseAccount.reporter).toEqual(reporterAccount);
      expect(fetchedCaseAccount.status).toEqual(CaseStatus.Closed);
      expect(fetchedCaseAccount.id.toNumber()).toEqual(cs.caseId.toNumber());
    });

    it("fail - full reporter can't update other reporter's case", async () => {
      const cs = CASES.nftTracking;

      const reporter = REPORTERS.alice.keypair;
      const newCaseName = bufferFromString("new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.updateCase(newCaseName.toJSON().data, CaseStatus.Closed, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.publicKey,
              community: community.publicKey,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("fail - validator can't update a case", async () => {
      const cs = CASES.nftTracking;

      const reporter = REPORTERS.dave.keypair;
      const newCaseName = bufferFromString("new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.updateCase(newCaseName.toJSON().data, CaseStatus.Closed, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.publicKey,
              community: community.publicKey,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        "301: Account is not authorized to perform this action"
      );
    });

    it("fail - tracer can't update a case", async () => {
      const cs = CASES.nftTracking;

      const reporter = REPORTERS.bob.keypair;
      const newCaseName = bufferFromString("new_name", 32);

      const [reporterAccount] = await program.findReporterAddress(
        community.publicKey,
        reporter.publicKey
      );

      const [caseAccount] = await program.findCaseAddress(
        community.publicKey,
        cs.caseId
      );

      await expectThrowError(
        () =>
          program.rpc.updateCase(newCaseName.toJSON().data, CaseStatus.Closed, {
            accounts: {
              reporter: reporterAccount,
              sender: reporter.publicKey,
              community: community.publicKey,
              case: caseAccount,
            },
            signers: [reporter],
          }),
        "301: Account is not authorized to perform this action"
      );
    });
  });
});
