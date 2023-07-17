import { v4 as uuidv4 } from "uuid";
import { CategoryKeys } from "../../solana/lib";

const chai = require("chai");
export var expect = chai.expect;

export function setupChai() {
  chai.config.truncateThreshold = 0;
  chai.config.showDiff = true;
}

const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);

export const NETWORK = "solana";
const KEYS_PATH = "test/keys";

export const KEYS: Record<string, { pubkey: string; path: string }> = {
  admin: {
    pubkey: "QDWdYo5JWQ96cCEgdBXpL6TVs5whScFSzVbZgobHLrQ",
    path: `${KEYS_PATH}/wallet_1.json`,
  },
  authority: {
    pubkey: "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX",
    path: `${KEYS_PATH}/wallet_2.json`,
  },
  publisher: {
    pubkey: "5L6h3A2TgUF7DuUky55cCkVdBY9Dvd7rjELVD23reoKk",
    path: `${KEYS_PATH}/wallet_3.json`,
  },
  token: {
    pubkey: "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC",
    path: `${KEYS_PATH}/token.json`,
  },
  program: {
    pubkey: "FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk",
    path: `../solana/tests/test_keypair.json`,
  },
};

export const REPORTERS: Record<
  string,
  {
    id: string;
    name: string;
    role: string;
    wallet: { pubkey: string; path: string };
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

export const ASSETS: Record<
  string,
  {
    address: string;
    assetId: string;
    caseId: string;
    category: CategoryKeys;
    riskScore: number;
  }
> = {
  firstAsset: {
    address: "0000000000000000000000000000000000000000000000000000000000000001",
    assetId: uuidv4(),
    caseId: CASES.firstCase.id,
    category: "WalletService",
    riskScore: 3,
  },
};

const BASE_ARGS = `-- --network ${NETWORK} --provider-url "http://localhost:8899" \
              --contract-address ${KEYS.program.pubkey} --output json`;

export async function execute_command(command: string, ignoreError = false) {
  try {
    const { stdout, stderr } = await exec(command);

    return { stdout, stderr };
  } catch (error) {
    const msg = `Command execution error. Command: ${command}, ${error}`;

    if (!ignoreError) {
      throw new Error(msg);
    }
    return {
      stdout: "",
      stderr: msg,
    };
  }
}

export async function cli_cmd(command: string, args = "") {
  const { stdout, stderr } = await execute_command(
    `npm run cmd ${command} ${BASE_ARGS} ${args}`
  );

  if (stderr.length > 0) {
    throw new Error(`Error stream: ${stderr}`);
  }

  return stdout;
}

export enum CommandCheck {
  ToBeEqual,
  ToContain,
}

export function checkCommandResult<Type>(
  res: string,
  val: Type,
  check = CommandCheck.ToBeEqual
) {
  const parsedObject: Type = JSON.parse(
    res.substring(res.indexOf("{")).replace(/'/g, '"')
  ).data;

  if (check == CommandCheck.ToBeEqual) {
    expect(parsedObject).to.deep.equal(val);
  } else {
    expect(parsedObject).to.deep.contain(val);
  }
}
