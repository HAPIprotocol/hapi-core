import { spawn, ExecException } from "child_process";
import chalk from "chalk";

const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);

const KEYPAIR = "../solana/tests/test_keypair.json";
const NETWORK = "solana";

async function execute_command(
  command: string,
  ignoreError = false
): Promise<string | undefined> {
  try {
    const { stdout, stderr } = await exec(command);

    // if (stderr.length != 0) {
    //   console.error(chalk.red(`Error stream: ${stderr}`));
    // }

    return stdout;
  } catch (error) {
    if (!ignoreError) {
      console.error(
        chalk.red(`Command execution error. Command: ${command}, ${error}`)
      );

      process.exit(1);
    }
  }
}

// TODO: add custom port
async function setup() {
  const validatorPid = await execute_command("lsof -t -i :8899", true);

  if (validatorPid) {
    const pid = parseInt(validatorPid);

    console.log(
      chalk.yellow(
        `Warning: port 8899 is already in use. Kill the process with ${pid} pid`
      )
    );

    await execute_command(`kill ${pid}`);
    await new Promise((resolve) => setTimeout(resolve, 100));

    console.log(chalk.green(`Process with ${pid} pid was killed`));
  }

  console.log("==> Initializing solana local validator");
  const validator = spawn("solana-test-validator", ["-r"]);

  validator.on("error", function (err) {
    console.error(chalk.red(`Validator error: ${err}`));
    process.exit(1);
  });

  console.log("==> Waiting for the validator to start");
  await new Promise((resolve) => setTimeout(resolve, 2000));

  console.log("==> Building and deploying program");
  await execute_command(
    `cd ../solana && anchor build &&  anchor deploy --program-keypair ${KEYPAIR}`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}

async function authorityTest() {
  //TODO:
  // 1. Check that initial authority matches the key of contract deployer
  // 2. Assign authority to a new address - Make sure that authority has changed
  // 3. Use the private key of the new authority to change the authority back - Make sure that authority has changed back

  await execute_command(`npm run cmd`);
}

async function main() {
  setup();

  // ls();
}

main();
