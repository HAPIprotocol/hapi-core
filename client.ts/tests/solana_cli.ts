import { spawn } from "child_process";
import chalk from "chalk";

const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);

const PROGRAM_KEYPAIR = "../solana/tests/test_keypair.json";
const WALLET_PATH =
  "/home/olha/Desktop/Solana/hapi-core2/hapi-core/client.ts/tests/keys";

const NETWORK = "solana";
const PROGRAM_ADDRESS = "FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk";

const WALLET1 = "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ";
const WALLET2 = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
const WALLET3 = "5L6h3A2TgUF7DuUky55cCkVdBY9Dvd7rjELVD23reoKk";

const ARGS = `-- --network ${NETWORK} --provider-url "http://localhost:8899" --address ${PROGRAM_ADDRESS}`;

async function execute_command(command: string, ignoreError = false) {
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

    process.kill(pid);
    console.log(chalk.green(`Process with ${pid} pid was killed`));
    await new Promise((resolve) => setTimeout(resolve, 100));
  }

  console.log("==> Initializing solana local validator");
  const validator = spawn("solana-test-validator", ["-r"]);

  validator.on("error", function (err) {
    console.error(chalk.red(`Validator error: ${err}`));
    process.exit(1);
  });

  console.log("==> Waiting for the validator to start");
  await new Promise((resolve) => setTimeout(resolve, 3000));

  console.log("==> Airdropping lamports on wallets");
  await execute_command(
    `solana airdrop 1000  ${WALLET1} && \
     solana airdrop 1000  ${WALLET2} && \
     solana airdrop 1000  ${WALLET3}`
  );

  console.log("==> Building and deploying program");

  process.env.ANCHOR_WALLET = `${WALLET_PATH}/wallet_1.json`;
  await execute_command(
    `cd ../solana && anchor build &&  anchor deploy \
    --program-keypair ${PROGRAM_KEYPAIR} --provider.wallet ${WALLET_PATH}/wallet_1.json`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}

function check_result(output: string, val: string) {
  let result = output
    .split("\n")
    .filter((line) => !line.startsWith(">") && line.length > 0)
    .toString();

  if (result != val) {
    console.error(chalk.red(`Expected: ${val}, got: ${result}`));
    process.exit(1);
  }
}

async function authorityTest() {
  //TODO:
  // 1. Check that initial authority matches the key of contract deployer
  // 2. Assign authority to a new address - Make sure that authority has changed
  // 3. Use the private key of the new authority to change the authority back - Make sure that authority has changed back

  // Check that initial authority matches the key of contract deployer
  check_result(
    await execute_command(`npm run cmd get-authority ${ARGS}`),
    WALLET1
  );
  console.log(chalk.green("Initial authority test passed"));

  check_result(
    await execute_command(`npm run cmd set-authority ${ARGS}`),
    WALLET1
  );
  console.log(chalk.green("New authority test passed"));
}

async function main() {
  await setup();
  await authorityTest();
}

main();
