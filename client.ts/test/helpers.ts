import { v4 as uuidv4 } from "uuid";
import { encodeAddress, CategoryKeys } from "../../solana/lib";

const chai = require("chai");
chai.config.truncateThreshold = 0;
chai.config.showDiff = true;
var expect = chai.expect;

const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);

export const NETWORK = "solana";
const KEYS_PATH = "test/keys";

export const KEYS: Record<string, { pk: string; path: string }> = {
  admin: {
    pk: "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ",
    path: `${KEYS_PATH}/wallet_1.json`,
  },
  authority: {
    pk: "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX",
    path: `${KEYS_PATH}/wallet_2.json`,
  },
  publisher: {
    pk: "5L6h3A2TgUF7DuUky55cCkVdBY9Dvd7rjELVD23reoKk",
    path: `${KEYS_PATH}/wallet_3.json`,
  },
  token: {
    pk: "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC",
    path: `${KEYS_PATH}/token.json`,
  },
  program: {
    pk: "FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk",
    path: `../solana/tests/test_keypair.json`,
  },
};

export const REPORTERS: Record<
  string,
  {
    id: string;
    name: string;
    role: string;
    wallet: { pk: string; path: string };
    url: string;
  }
> = {
  authority: {
    id: uuidv4(),
    name: "authorityReporter",
    role: "Authority",
    wallet: KEYS.authority,
    url: "https://authority.blockchain",
  },
  publisher: {
    id: uuidv4(),
    name: "publisherReporter",
    role: "Publisher",
    wallet: KEYS.publisher,
    url: "https://publisher.blockchain",
  },
};

export const CASES: Record<
  string,
  {
    id: string;
    name: string;
    url: string;
  }
> = {
  firstCase: {
    id: uuidv4(),
    name: "firstCase",
    url: "https://big.hack",
  },
  secondCase: {
    id: uuidv4(),
    name: "secondCase",
    url: "https://big2.hack",
  },
};

export const ADDRESSES: Record<
  string,
  {
    address: string;
    caseId: string;
    category: CategoryKeys;
    riskScore: number;
  }
> = {
  firstAddr: {
    address: "0000000000000000000000000000000000000000000000000000000000000001",
    caseId: CASES.firstCase.id,
    category: "WalletService",
    riskScore: 3,
  },
};

const ARGS = `-- --network ${NETWORK} --provider-url "http://localhost:8899" \
              --contract-address ${KEYS.program.pk} --output json`;

export async function execute_command(command: string, ignoreError = false) {
  try {
    const { stdout, stderr } = await exec(command);

    return { stdout, stderr };
  } catch (error) {
    if (!ignoreError) {
      throw new Error(`Command execution error. Command: ${command}, ${error}`);
      // console.log(
      //   chalk.red(`Command execution error. Command: ${command}, ${error}`)
      // );
      // process.exit(1);
    }
    return { stdout: "", stderr: "" };
  }
}

export async function cli_cmd(command: string, arg = "") {
  const { stdout, stderr } = await execute_command(
    `npm run cmd ${command} ${ARGS} ${arg}`
  );

  if (stderr.length > 0) {
    // console.log(chalk.red(`Error stream: ${stderr}`));
    // process.exit(1);

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
