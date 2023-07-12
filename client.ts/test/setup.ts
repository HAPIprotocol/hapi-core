import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import chalk from "chalk";
import * as Token from "@solana/spl-token";
import { Provider, web3 } from "@coral-xyz/anchor";
import { readFileSync } from "fs";

import { execute_command, KEYS, NETWORK } from "./helpers";

var VALIDATOR: ChildProcessWithoutNullStreams;

// TODO: add custom port
async function shutDownExistingValidator(display = true) {
  const validatorPid = await execute_command("lsof -t -i :8899", true);

  if (validatorPid.stdout.length > 0) {
    const pid = parseInt(validatorPid.stdout);

    if (display)
      console.log(
        chalk.yellow(
          `Warning: port 8899 is already in use. Kill the process with ${pid} pid`
        )
      );

    process.kill(pid);
    if (display) console.log(chalk.green(`Process with ${pid} pid was killed`));
    await new Promise((resolve) => setTimeout(resolve, 100));
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
  await new Promise((resolve) => setTimeout(resolve, 3000));
}

export function killValidator() {
  VALIDATOR.kill();
}

async function prepareValidator() {
  console.log("==> Building and deploying program");

  const wallet = process.cwd() + "/" + KEYS.admin.path;
  process.env.ANCHOR_WALLET = wallet;
  await execute_command(
    `cd ../solana && anchor build &&  anchor deploy \
    --program-keypair ${KEYS.program.path} --provider.wallet ${wallet}`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}

export async function setupWallets(provider: Provider) {
  for (const key in KEYS) {
    if (key != "token" && key != "program") {
      const wallet = new web3.PublicKey(KEYS[key].pk);

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
      const pk = new web3.PublicKey(KEYS[key].pk);

      const tokenAccount = await Token.getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        pk
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

export async function setup(provider: Provider) {
  await shutDownExistingValidator();
  await startValidator();
  await setupWallets(provider);
  await prepareValidator();
}
