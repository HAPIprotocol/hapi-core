import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import chalk from "chalk";
import * as Token from "@solana/spl-token";
import { Provider, web3 } from "@coral-xyz/anchor";
import { readFileSync } from "fs";

import { execute_command, KEYS, NETWORK } from "./helpers";

let VALIDATOR: ChildProcessWithoutNullStreams;
const VALIDATOR_PORT = 8899;

async function shutDownExistingValidator(display = true) {
  const validatorPid = await execute_command(
    `lsof -t -i :${VALIDATOR_PORT}`,
    true
  );

  if (validatorPid.stdout.length > 0) {
    const pid = parseInt(validatorPid.stdout);

    if (display)
      console.log(
        chalk.yellow(
          `Warning: port ${VALIDATOR_PORT} is already in use. Kill the process with ${pid} pid`
        )
      );

    process.kill(pid);

    while ((await execute_command(`ps -p ${pid}`, true)).stderr.length == 0) {
      await delay(100);
    }
    if (display) console.log(chalk.green(`Process with ${pid} pid was killed`));
  }
}

async function startValidator() {
  console.log("==> Initializing solana local validator");
  VALIDATOR = spawn("solana-test-validator", ["-r"]);

  VALIDATOR.on("error", function (err) {
    console.error(chalk.red(`Validator error: ${err}`));
    process.exit(1);
  });

  console.log("==> Waiting for the validator to start");

  while (
    (await execute_command(`lsof -t -i :${VALIDATOR_PORT}`, true)).stdout
      .length == 0
  ) {
    await delay(100);
  }
}

export function killValidator() {
  VALIDATOR.kill();
}

async function prepareValidator() {
  console.log("==> Building and deploying program");

  const wallet = KEYS.admin.path;
  const programDir = __dirname + "/../../solana";

  process.env.ANCHOR_WALLET = wallet;
  await execute_command(
    `cd ${programDir} && anchor deploy \
    --program-keypair ${KEYS.program.path} --program-name hapi_core_solana --provider.wallet ${wallet}`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}

export async function setupWallets(provider: Provider) {
  for (const key in KEYS) {
    if (key != "token" && key != "program") {
      const wallet = new web3.PublicKey(KEYS[key].pubkey);

      await provider.connection.requestAirdrop(
        wallet,
        10 * web3.LAMPORTS_PER_SOL
      );
    }
  }
  await new Promise((resolve) => setTimeout(resolve, 500));

  const payer = web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(KEYS.admin.path, "utf-8")))
  );

  const tokenKeypair = web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(KEYS.token.path, "utf-8")))
  );

  console.log("==> Creating token");
  const mint = await Token.createMint(
    provider.connection,
    payer,
    payer.publicKey,
    null,
    9,
    tokenKeypair
  );

  console.log("==> Preparing wallets");
  for (const key in KEYS) {
    if (key != "token" && key != "program") {
      const pubkey = new web3.PublicKey(KEYS[key].pubkey);

      const tokenAccount = await Token.getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        pubkey
      );

      await Token.mintTo(
        provider.connection,
        payer as web3.Signer,
        mint,
        tokenAccount.address,
        payer.publicKey,
        100_000
      );
    }
  }
}

async function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function setup(provider: Provider) {
  await shutDownExistingValidator();
  await startValidator();
  await setupWallets(provider);
  await prepareValidator();
}
