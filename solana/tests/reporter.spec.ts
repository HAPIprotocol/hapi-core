import * as anchor from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";
import { TestToken } from "./util/token";
import { expectThrowError } from "./util/console";

import {
    ACCOUNT_SIZE,
    bufferFromString,
    ReporterRole,
    HapiCoreProgram,
    ReporterStatus,
    stakeConfiguration,
    rewardConfiguration
} from "../lib";
import { programError } from "./util/error";

describe("HapiCore Reporter", () => {
    const program = new HapiCoreProgram(new web3.PublicKey("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk"));

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

    const NETWORKS: Record<
        string,
        { name: string; stakeConfiguration: stakeConfiguration, rewardConfiguration: rewardConfiguration; }
    > = {
        main: {
            name: "ReporterTest",
            stakeConfiguration: {
                unlockDuration: new BN(1),
                validatorStake: new BN(2_000),
                tracerStake: new BN(3_000),
                publisherStake: new BN(4_000),
                authorityStake: new BN(5_000),
                appraiserStake: new BN(6_000),
            },
            rewardConfiguration: {
                addressTracerReward: new BN(1_000),
                addressConfirmationReward: new BN(2_000),
                assetTracerReward: new BN(3_000),
                assetConfirmationReward: new BN(4_000),
            },
        },
        secondary: {
            name: "ReporterTest2",
            stakeConfiguration: {
                unlockDuration: new BN(1_000),
                validatorStake: new BN(2_000),
                tracerStake: new BN(3_000),
                publisherStake: new BN(4_000),
                authorityStake: new BN(5_000),
                appraiserStake: new BN(6_000),
            },
            rewardConfiguration: {
                addressTracerReward: new BN(1_000),
                addressConfirmationReward: new BN(2_000),
                assetTracerReward: new BN(3_000),
                assetConfirmationReward: new BN(4_000),
            },
        },
    };

    const programDataAddress = program.findProgramDataAddress()[0];

    beforeAll(async () => {

        const wait: Promise<unknown>[] = [];

        stakeToken = new TestToken(provider);
        await stakeToken.mint(1_000_000_000);

        rewardToken = new TestToken(provider);
        await rewardToken.mint(1_000_000_000);

        wait.push(provider.connection.requestAirdrop(
            another_authority.publicKey,
            10_000_000
        ));

        for (const key of Object.keys(NETWORKS)) {
            const network = NETWORKS[key];

            const [networkAccount, bump] = program.findNetworkAddress(
                network.name
            );

            const args = [
                bufferFromString(network.name, 32).toJSON().data,
                network.stakeConfiguration,
                network.rewardConfiguration,
                bump,
            ];

            wait.push(program.program.rpc.createNetwork(...args, {
                accounts: {
                    authority: authority.publicKey,
                    network: networkAccount,
                    rewardMint: rewardToken.mintAccount,
                    stakeMint: stakeToken.mintAccount,
                    programAccount: program.programId,
                    programData: programDataAddress,
                    systemProgram: web3.SystemProgram.programId,
                },
            }));
        };

        await Promise.all(wait);

    });

    describe("create_reporter", () => {

        it("fail - authority mismatch", async () => {
            const networkName = NETWORKS.main.name;

            const [networkAccount, _] = program.findNetworkAddress(
                networkName
            );

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = program.findReporterAddress(
                networkAccount, reporter.id
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
                    program.program.rpc.createReporter(...args, {
                        accounts: {
                            authority: another_authority.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            systemProgram: web3.SystemProgram.programId,
                        },
                        signers: [another_authority]
                    }),
                programError("AuthorityMismatch")
            );

        });

        it("success - alice", async () => {
            const networkName = NETWORKS.main.name;

            const [networkAccount, _] = program.findNetworkAddress(
                networkName
            );

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = program.findReporterAddress(
                networkAccount, reporter.id
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

            await program.program.rpc.createReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    network: networkAccount,
                    systemProgram: web3.SystemProgram.programId,
                },
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
            expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
            expect(fetchedReporterAccount.bump).toEqual(bump);

            const reporterInfo = await provider.connection.getAccountInfoAndContext(
                reporterAccount
            );
            expect(reporterInfo.value.owner).toEqual(program.programId);
            expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
        });

        it("success - bob", async () => {
            const networkName = NETWORKS.secondary.name;

            const [networkAccount, _] = program.findNetworkAddress(
                networkName
            );

            const reporter = REPORTERS.bob;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = program.findReporterAddress(
                networkAccount, reporter.id
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

            await program.program.rpc.createReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    network: networkAccount,
                    systemProgram: web3.SystemProgram.programId,
                },
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
            expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
            expect(fetchedReporterAccount.bump).toEqual(bump);

            const reporterInfo = await provider.connection.getAccountInfoAndContext(
                reporterAccount
            );
            expect(reporterInfo.value.owner).toEqual(program.programId);
            expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
        });

        it("success - carol", async () => {
            const networkName = NETWORKS.main.name;

            const [networkAccount, _] = program.findNetworkAddress(
                networkName
            );

            const reporter = REPORTERS.carol;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = program.findReporterAddress(
                networkAccount, reporter.id
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

            await program.program.rpc.createReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    network: networkAccount,
                    systemProgram: web3.SystemProgram.programId,
                },
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.id.eq(reporter.id)).toBeTruthy();
            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
            expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
            expect(fetchedReporterAccount.bump).toEqual(bump);

            const reporterInfo = await provider.connection.getAccountInfoAndContext(
                reporterAccount
            );
            expect(reporterInfo.value.owner).toEqual(program.programId);
            expect(reporterInfo.value.data.length).toEqual(ACCOUNT_SIZE.reporter);
        });

        it("fail - reporter already exists", async () => {
            const networkName = NETWORKS.main.name;

            const [networkAccount, _] = program.findNetworkAddress(
                networkName
            );

            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const [reporterAccount, bump] = program.findReporterAddress(
                networkAccount, reporter.id
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
                    program.program.rpc.createReporter(...args, {
                        accounts: {
                            authority: authority.publicKey,
                            reporter: reporterAccount,
                            network: networkAccount,
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

            const networkName = NETWORKS.main.name;

            const networkAccount = (program.findNetworkAddress(
                networkName
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
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
                    program.program.rpc.updateReporter(...args, {
                        accounts: {
                            authority: another_authority.publicKey,
                            reporter: reporterAccount,
                            network: networkAccount
                        },
                        signers: [another_authority]
                    }),
                programError("AuthorityMismatch")
            );

        });

        it("fail - reporter does not exists", async () => {
            const reporter = REPORTERS.dave;
            let name = bufferFromString(reporter.name, 32);

            const networkName = NETWORKS.main.name;

            const networkAccount = (program.findNetworkAddress(
                networkName
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
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
                    program.program.rpc.updateReporter(...args, {
                        accounts: {
                            authority: authority.publicKey,
                            reporter: reporterAccount,
                            network: networkAccount,
                        },
                    }),
                /The program expected this account to be already initialized/
            );

        });

        it("fail - network mismatch", async () => {
            const reporter = REPORTERS.alice;
            let name = bufferFromString(reporter.name, 32);

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.secondary.name
            ))[0];

            const reporterNetworkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                reporterNetworkAccount, reporter.id
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
                    program.program.rpc.updateReporter(...args, {
                        accounts: {
                            authority: authority.publicKey,
                            reporter: reporterAccount,
                            network: networkAccount,
                        },
                    }),
                /A seeds constraint was violated/
            );

        });

        it("success", async () => {
            const reporter = REPORTERS.alice;
            let name = bufferFromString("new_name", 32);

            const networkName = NETWORKS.main.name;

            const networkAccount = (program.findNetworkAddress(
                networkName
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const reporterRole = ReporterRole[reporter.role];

            const args = [
                reporter.keypair.publicKey,
                name.toJSON().data,
                reporterRole,
                reporter.url,
            ];

            await program.program.rpc.updateReporter(...args, {
                accounts: {
                    authority: authority.publicKey,
                    reporter: reporterAccount,
                    network: networkAccount,
                },
            });
            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.account).toEqual(reporter.keypair.publicKey);
            expect(Buffer.from(fetchedReporterAccount.name)).toEqual(name);
            expect(fetchedReporterAccount.role).toEqual(reporterRole);
            expect(fetchedReporterAccount.url).toEqual(reporter.url);
        });
    });

    describe("activate_reporter", () => {

        it("fail - network mismatch", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.secondary.name
            ))[0];

            const reporterNetworkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                reporterNetworkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                /A seeds constraint was violated/
            );

        });

        it("fail - invalid reporter", async () => {
            const reporter = REPORTERS.alice;
            const anotherReporter = REPORTERS.bob;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: anotherReporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [anotherReporter.keypair]
                    }),
                programError("InvalidReporter")
            );

        });

        it("fail - invalid network ATA mint", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const invalidNetworkStakeTokenAccount = await rewardToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount: invalidNetworkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                programError("InvalidToken")
            );
        });

        it("fail - invalid reporter ATA mint", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const invalidReporterStakeTokenAccount = await rewardToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount: invalidReporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                programError("InvalidToken")
            );

        });

        it("fail - invalid network ATA owner", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount: reporterStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                programError("IllegalOwner")
            );
        });

        it("fail - invalid reporter ATA owner", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount: networkStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                programError("IllegalOwner")
            );

        });

        it("fail - insufficient funds", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                /Error processing Instruction 0: custom program error: 0x1/
            );

        });

        it("success - alice", async () => {
            const reporter = REPORTERS.alice;
            let network = NETWORKS.main;

            const networkAccount = (program.findNetworkAddress(
                network.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await stakeToken.transfer(
                null,
                reporter.keypair.publicKey,
                network.stakeConfiguration.publisherStake.toNumber()
            );

            await program.program.rpc.activateReporter({
                accounts: {
                    signer: reporter.keypair.publicKey,
                    network: networkAccount,
                    reporter: reporterAccount,
                    networkStakeTokenAccount,
                    reporterStakeTokenAccount,
                    tokenProgram: stakeToken.programId
                },
                signers: [reporter.keypair]
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.stake.eq(network.stakeConfiguration.publisherStake)).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

        });

        it("success - bob", async () => {
            const reporter = REPORTERS.bob;
            let network = NETWORKS.secondary;

            const networkAccount = (program.findNetworkAddress(
                network.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await stakeToken.transfer(
                null,
                reporter.keypair.publicKey,
                network.stakeConfiguration.publisherStake.toNumber()
            );

            await program.program.rpc.activateReporter({
                accounts: {
                    signer: reporter.keypair.publicKey,
                    network: networkAccount,
                    reporter: reporterAccount,
                    networkStakeTokenAccount,
                    reporterStakeTokenAccount,
                    tokenProgram: stakeToken.programId
                },
                signers: [reporter.keypair]
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.stake.eq(network.stakeConfiguration.tracerStake)).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Active);

        });

        it("fail - reporter is already activated", async () => {
            const reporter = REPORTERS.alice;
            let network = NETWORKS.main;

            const networkAccount = (program.findNetworkAddress(
                network.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await stakeToken.transfer(
                null,
                reporter.keypair.publicKey,
                network.stakeConfiguration.publisherStake.toNumber()
            );

            await expectThrowError(
                () =>
                    program.program.rpc.activateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId
                        },
                        signers: [reporter.keypair]
                    }),
                programError("InvalidReporterStatus")
            );

        });
    });

    describe("dectivate_reporter", () => {

        it("fail - reporter is not activated", async () => {
            const reporter = REPORTERS.carol;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            await expectThrowError(
                () =>
                    program.program.rpc.deactivateReporter({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                        },
                        signers: [reporter.keypair]
                    }),
                programError("InvalidReporterStatus")
            );

        });

        it("success - alice", async () => {
            const reporter = REPORTERS.alice;
            let network = NETWORKS.main;

            const networkAccount = (program.findNetworkAddress(
                network.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            let { slotIndex } = await provider.connection.getEpochInfo();
            const timestamp = await provider.connection.getBlockTime(slotIndex);

            await program.program.rpc.deactivateReporter({
                accounts: {
                    signer: reporter.keypair.publicKey,
                    network: networkAccount,
                    reporter: reporterAccount,
                },
                signers: [reporter.keypair]
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            let unlockDuration = network.stakeConfiguration.unlockDuration.toNumber();

            expect(fetchedReporterAccount.stake.eq(network.stakeConfiguration.publisherStake)).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
            expect(fetchedReporterAccount.unlockTimestamp.toNumber()).toBeGreaterThanOrEqual(timestamp + unlockDuration);

        });

        it("success - bob", async () => {
            const reporter = REPORTERS.bob;
            let network = NETWORKS.secondary;

            const networkAccount = (program.findNetworkAddress(
                network.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            let { slotIndex } = await provider.connection.getEpochInfo();
            const timestamp = await provider.connection.getBlockTime(slotIndex);

            await program.program.rpc.deactivateReporter({
                accounts: {
                    signer: reporter.keypair.publicKey,
                    network: networkAccount,
                    reporter: reporterAccount,
                },
                signers: [reporter.keypair]
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            let unlockDuration = network.stakeConfiguration.unlockDuration.toNumber();

            expect(fetchedReporterAccount.stake.eq(network.stakeConfiguration.tracerStake)).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Unstaking);
            expect(fetchedReporterAccount.unlockTimestamp.toNumber()).toBeGreaterThanOrEqual(timestamp + unlockDuration);

        });
    });

    describe("unstake", () => {

        it("fail - reporter is not deacactivated", async () => {
            const reporter = REPORTERS.carol;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.unstake({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId

                        },
                        signers: [reporter.keypair]
                    }),
                programError("InvalidReporterStatus")
            );

        });

        it("fail - release epoch in future", async () => {
            const reporter = REPORTERS.bob;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.secondary.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );

            await expectThrowError(
                () =>
                    program.program.rpc.unstake({
                        accounts: {
                            signer: reporter.keypair.publicKey,
                            network: networkAccount,
                            reporter: reporterAccount,
                            networkStakeTokenAccount,
                            reporterStakeTokenAccount,
                            tokenProgram: stakeToken.programId

                        },
                        signers: [reporter.keypair]
                    }),
                programError("ReleaseEpochInFuture")
            );

        });

        it("success", async () => {
            const reporter = REPORTERS.alice;

            const networkAccount = (program.findNetworkAddress(
                NETWORKS.main.name
            ))[0];

            const [reporterAccount, _] = program.findReporterAddress(
                networkAccount, reporter.id
            );

            const networkStakeTokenAccount = await stakeToken.getTokenAccount(
                networkAccount,
                true
            );

            const reporterStakeTokenAccount = await stakeToken.getTokenAccount(
                reporter.keypair.publicKey
            );
            await program.program.rpc.unstake({
                accounts: {
                    signer: reporter.keypair.publicKey,
                    network: networkAccount,
                    reporter: reporterAccount,
                    networkStakeTokenAccount,
                    reporterStakeTokenAccount,
                    tokenProgram: stakeToken.programId
                },
                signers: [reporter.keypair]
            });

            const fetchedReporterAccount = await program.program.account.reporter.fetch(
                reporterAccount
            );

            expect(fetchedReporterAccount.stake.isZero()).toBeTruthy();
            expect(fetchedReporterAccount.status).toEqual(ReporterStatus.Inactive);
            expect(fetchedReporterAccount.unlockTimestamp.isZero()).toBeTruthy();

        });
    });
});
