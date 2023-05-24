import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";

import {
    ACCOUNT_SIZE,
    bufferFromString,
    ReporterRole,
    initHapiCore,
} from "./lib";
import { programError } from "./util/error";

describe("HapiCore Reporter", () => {
    const program = initHapiCore(new web3.PublicKey("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk"));

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const authority = provider.wallet;
    const another_authority = web3.Keypair.generate();

    let stakeToken: TestToken;
    let rewardToken: TestToken;

    const REPORTERS: Record<
        string,
        { name: string; id: BN, keypair: web3.Keypair; role: keyof typeof ReporterRole; url: string }
    > = {
        alice: {
            id: new BN(1),
            name: "alice",
            keypair: web3.Keypair.generate(),
            role: "Publisher",
            url: "alice url"
        },
        bob: { id: new BN(2), name: "bob", keypair: web3.Keypair.generate(), role: "Tracer", url: "bob url" },
        carol: {
            id: new BN(3),
            name: "carol",
            keypair: web3.Keypair.generate(),
            role: "Authority",
            url: "carol url"
        },
        dave: { id: new BN(4), name: "dave", keypair: web3.Keypair.generate(), role: "Publisher", url: "dave url" },
        erin: { id: new BN(5), name: "erin", keypair: web3.Keypair.generate(), role: "Appraiser", url: "erin url" },
    };

    const programDataAddress = web3.PublicKey.findProgramAddressSync(
        [program.programId.toBytes()],
        new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    )[0];

    beforeAll(async () => {

        stakeToken = new TestToken(provider);
        await stakeToken.mint(1_000_000_000);

        rewardToken = new TestToken(provider);
        await rewardToken.mint(1_000_000_000);

        await provider.connection.requestAirdrop(
            another_authority.publicKey,
            10_000_000
        );

        const networkName = "ReporterTest";

        const stakeConfiguration = {
            unlockDuration: new BN(1_000),
            validatorStake: new BN(2_000),
            tracerStake: new BN(3_000),
            publisherStake: new BN(4_000),
            authorityStake: new BN(5_000),
        };

        const rewardConfiguration = {
            addressTracerReward: new BN(1_000),
            addressConfirmationReward: new BN(2_000),
            assetTracerReward: new BN(3_000),
            assetConfirmationReward: new BN(4_000),
        };

        const [networkAccount, bump] = await program.pda.findNetworkAddress(
            networkName
        );

        const stakeTokenAccount = await stakeToken.getTokenAccount(
            networkAccount,
            true
        );

        const args = [
            bufferFromString(networkName, 32).toJSON().data,
            stakeConfiguration,
            rewardConfiguration,
            bump,
        ];

        await program.rpc.createNetwork(...args, {
            accounts: {
                authority: authority.publicKey,
                network: networkAccount,
                rewardMint: rewardToken.mintAccount,
                stakeMint: stakeToken.mintAccount,
                stakeTokenAccount,
                programAccount: program.programId,
                programData: programDataAddress,
                systemProgram: web3.SystemProgram.programId,
            },
        });

    });

    describe("create_reporter", () => {

        it("fail - authority mismatch", async () => {

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.id,
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
                bump,
            ];

            await expectThrowError(
                () =>
                    program.rpc.createReporter(...args, {
                        accounts: {
                            authority: another_authority.publicKey,
                            reporter: reporterAccount,
                            programAccount: program.programId,
                            programData: programDataAddress,
                            systemProgram: web3.SystemProgram.programId,
                        },
                        signers: [another_authority]
                    }),
                programError("AuthorityMismatch")
            );

        });

        it("success", async () => {
            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.id,
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
                bump,
            ];

            await program.rpc.createReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    programAccount: program.programId,
                    programData: programDataAddress,
                    systemProgram: web3.SystemProgram.programId,
                },
            });

            const fetchedReporterAccount = await program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
            expect(fetchedReporterAccount.bump).toEqual(bump);

            const reporterInfo = await provider.connection.getAccountInfoAndContext(
                reporterAccount
            );
            expect(reporterInfo.value.owner).toEqual(program.programId);
            expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
        });

        it("fail - network already exists", async () => {

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.id,
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
                bump,
            ];

            await expectThrowError(
                () =>
                    program.rpc.createReporter(...args, {
                        accounts: {
                            authority: authority.publicKey,
                            reporter: reporterAccount,
                            programAccount: program.programId,
                            programData: programDataAddress,
                            systemProgram: web3.SystemProgram.programId,
                        },
                    }),
                /custom program error: 0x0/
            );

        });
    });

    describe("update_reporter", () => {

        it("fail - authority mismatch", async () => {

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, _] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
            ];

            await expectThrowError(
                () =>
                    program.rpc.updateReporter(...args, {
                        accounts: {
                            authority: another_authority.publicKey,
                            reporter: reporterAccount,
                            programAccount: program.programId,
                            programData: programDataAddress,
                        },
                        signers: [another_authority]
                    }),
                programError("AuthorityMismatch")
            );

        });

        it("fail - reporter does not exists", async () => {
            const reporter = REPORTERS.bob;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, _] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
            ];

            await expectThrowError(
                () =>
                    program.rpc.updateReporter(...args, {
                        accounts: {
                            authority: authority.publicKey,
                            reporter: reporterAccount,
                            programAccount: program.programId,
                            programData: programDataAddress,
                        },
                    }),
                /The program expected this account to be already initialized/
            );

        });

        it("success", async () => {
            const reporter = REPORTERS.alice;
            let name = bufferFromString("new_name", 32);

            const [reporterAccount, _] = await program.pda.findReporterAddress(
                reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
            ];

            await program.rpc.updateReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    programAccount: program.programId,
                    programData: programDataAddress,
                },
            });
            const fetchedReporterAccount = await program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
        });
    });

});
