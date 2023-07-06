import { spawn } from "child_process";
import chalk from "chalk";

const chai = require("chai");
var expect = chai.expect;
chai.config.truncateThreshold = 0;

const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);

const PROGRAM_KEYPAIR = "../solana/tests/test_keypair.json";
export const WALLET_PATH = "test/keys";

export const NETWORK = "solana";
export const PROGRAM_ADDRESS = "FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk";

export const WALLET1 = "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ";
export const WALLET2 = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
export const WALLET3 = "5L6h3A2TgUF7DuUky55cCkVdBY9Dvd7rjELVD23reoKk";

const ARGS = `-- --network ${NETWORK} --provider-url "http://localhost:8899" --contract-address ${PROGRAM_ADDRESS} --output json`;

async function execute_command(command: string, ignoreError = false) {
  try {
    const { stdout, stderr } = await exec(command);

    return { stdout, stderr };
  } catch (error) {
    if (!ignoreError) {
      throw new Error(`Command execution error. Command: ${command}, ${error}`);
    }
    return { stdout: "", stderr: "" };
  }
}

export async function run_cmd(command: string, arg = "") {
  const { stdout, stderr } = await execute_command(
    `npm run cmd ${command} ${ARGS} ${arg}`
  );

  if (stderr.length > 0) {
    throw new Error(`Error stream: ${stderr}`);
  }

  return stdout;
}

export function checkCommandResult<Type>(res: string, val: Type) {
  const parsedObject: Type = JSON.parse(
    res.substring(res.indexOf("{")).replace(/'/g, '"')
  ).data;

  expect(parsedObject).to.deep.equal(val);
}

// TODO: add custom port
export async function shutDownExisting(display = true) {
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

export async function setup() {
  await shutDownExisting();

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

  const wallet = process.cwd() + `/${WALLET_PATH}/wallet_1.json`;
  process.env.ANCHOR_WALLET = wallet;
  await execute_command(
    `cd ../solana && anchor build &&  anchor deploy \
    --program-keypair ${PROGRAM_KEYPAIR} --provider.wallet ${wallet}`
  );

  console.log("==> Creating network for tests");
  await execute_command(`npm --prefix ../solana run create-network ${NETWORK}`);
}
