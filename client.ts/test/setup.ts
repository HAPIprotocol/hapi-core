import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import chalk from "chalk";

import { execute_command, KEYS, NETWORK } from "./helpers";

var VALIDATOR: ChildProcessWithoutNullStreams;

// TODO: add custom port
async function shutDownExisting(display = true) {
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

export function killValidator() {
  VALIDATOR.kill();
}

export async function setup() {
  await shutDownExisting();

  console.log("==> Initializing solana local validator");
  VALIDATOR = spawn("solana-test-validator", ["-r"]);

  VALIDATOR.on("error", function (err) {
    console.error(chalk.red(`Validator error: ${err}`));
    process.exit(1);
  });

  console.log("==> Waiting for the validator to start");
  await new Promise((resolve) => setTimeout(resolve, 3000));

  console.log("==> Creating token");
  const token = process.cwd() + "/" + KEYS.token.path;
  await execute_command(`spl-token create-token ${token}`);

  console.log("==> Preparing wallets");
  for (const key in KEYS) {
    const wallet = KEYS[key];
    process.env.ANCHOR_WALLET = wallet.path;

    await execute_command(`solana airdrop 1000  ${wallet.pk}`);
    await execute_command(`spl-token create-account ${KEYS.token.pk}`);
    await execute_command(`spl-token mint ${KEYS.token.pk} 10000`);

    console.log(await execute_command(`spl-token balance ${KEYS.token.pk}`));
  }

  console.log("==> Building and deploying program");

  const wallet = process.cwd() + "/" + KEYS.wallet1.path;
  process.env.ANCHOR_WALLET = wallet;
  await execute_command(
    `cd ../solana && anchor build &&  anchor deploy \
    --program-keypair ${KEYS.program.path} --provider.wallet ${wallet}`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}
