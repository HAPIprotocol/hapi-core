import { HardhatUserConfig, task } from "hardhat/config";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import "@openzeppelin/hardhat-upgrades";
import "@nomicfoundation/hardhat-toolbox";
import { Contract, ContractTransactionResponse } from "ethers";

import { HapiCore, Token } from "./typechain-types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

const HARDHAT_NETWORK = process.env.HARDHAT_NETWORK || "hardhat";
const HARDHAT_LOCALHOST_URL =
  process.env.HARDHAT_LOCALHOST_URL || "http://127.0.0.1:8545";

const config: HardhatUserConfig = {
  defaultNetwork: HARDHAT_NETWORK,
  typechain: {
    target: "ethers-v6",
  },
  solidity: {
    version: "0.8.22",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  networks: {
    localhost: {
      url: HARDHAT_LOCALHOST_URL,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : undefined,
    },
    mainnet: {
      url: `https://mainnet.infura.io/v3/${process.env.INFURA_KEY}`,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : undefined,
    },
    sepolia: {
      url: `https://sepolia.infura.io/v3/${process.env.INFURA_KEY}`,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : undefined,
    },
    linea: {
      url: `https://linea-mainnet.infura.io/v3/${process.env.INFURA_KEY}`,
      chainId: 59144,
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : undefined,
      gasPrice: 3_000_000_000,
      timeout: 36000,
    },
  },
  gasReporter: {
    currency: "ETH",
    showTimeSpent: true,
    enabled: true,
  },
};

function forceSignerGasPrice(hre: any, signer: any, gwei: string) {
  signer.getFeeData = async () => ({
    lastBaseFeePerGas: null,
    gasPrice: hre.ethers.utils.parseUnits(gwei, "gwei"),
    maxFeePerGas: null,
    maxPriorityFeePerGas: null,
  });
}

task("deploy", "Deploys the HAPI Core contract").setAction(async (_, hre) => {
  try {
    const [signer] = await hre.ethers.getSigners();

    if (!signer) {
      throw new Error("No signer found");
    } else {
      console.log(`Using wallet: ${signer.address}`);
    }

    const network = await hre.ethers.provider.getNetwork();

    console.log(`Deploying to '${network.name}' (${network.chainId})`);

    const HapiCore = await hre.ethers.getContractFactory("HapiCore", signer);

    const gasPrice = process.env.GAS_PRICE_GWEI;
    if (gasPrice) {
      console.log(`Enforcing gas price: ${gasPrice} Gwei`);
      forceSignerGasPrice(hre, await hre.ethers.provider.getSigner(), gasPrice);
    }

    const contract = (await hre.upgrades.deployProxy(HapiCore as any, [], {
      initializer: "initialize",
      pollingInterval: Number(process.env.DEPLOY_POLLING_INTERVAL) || 10000,
      timeout: Number(process.env.DEPLOY_TIMEOUT) || 3600000,
    })) as Contract;

    await contract.waitForDeployment();

    const contractAddress = await contract.getAddress();

    const adminAddress = await hre.upgrades.erc1967.getAdminAddress(
      contractAddress
    );
    const implementationAddress =
      await hre.upgrades.erc1967.getImplementationAddress(contractAddress);

    console.log(`HAPI Core deployed`, {
      contract: contractAddress,
      admin: adminAddress,
      implementation: implementationAddress,
    });
  } catch (error) {
    console.error(`${error}`);
    process.exit(1);
  }
});

task("deploy-test-token", "Deploys the HAPI Test Token contract").setAction(
  async (_, hre) => {
    try {
      const [signer] = await hre.ethers.getSigners();

      if (!signer) {
        throw new Error("No signer found");
      } else {
        console.log(`Using wallet: ${signer.address}`);
      }

      console.log(`Using wallet: ${signer.address}`);

      const network = await hre.ethers.provider.getNetwork();

      console.log(`Deploying to '${network.name}' (${network.chainId})`);

      const TestToken = await hre.ethers.getContractFactory("Token", signer);

      const gasPrice = process.env.GAS_PRICE_GWEI;
      if (gasPrice) {
        console.log(`Enforcing gas price: ${gasPrice} Gwei`);
        forceSignerGasPrice(hre, signer, gasPrice);
      }

      const contract = await TestToken.deploy();

      await contract.waitForDeployment();

      console.log(`HAPI Test Token deployed`, {
        contract: await contract.getAddress(),
      });
    } catch (error) {
      console.error(`${error}`);
      process.exit(1);
    }
  }
);

task("upgrade", "Upgrades the HAPI Core contract")
  .addParam("address", "Contract address")
  .setAction(async (args, hre) => {
    try {
      let [signer] = await hre.ethers.getSigners();

      if (!signer) {
        throw new Error("No signer found");
      } else {
        console.log(`Using wallet: ${signer.address}`);
      }

      let network = await hre.ethers.provider.getNetwork();

      console.log(`Deploying to '${network.name}' (${network.chainId})`);

      const HapiCore = await hre.ethers.getContractFactory("HapiCore");

      const gasPrice = process.env.GAS_PRICE_GWEI;
      if (gasPrice) {
        console.log(`Enforcing gas price: ${gasPrice} Gwei`);
        forceSignerGasPrice(hre, signer, gasPrice);
      }

      if (!!process.env.FORCE_IMPORT) {
        await hre.upgrades.forceImport(args.address, HapiCore as any);
      }

      const contract = await hre.upgrades.upgradeProxy(
        args.address,
        HapiCore as any
      );

      let contractAddress = await contract.getAddress();

      const adminAddress = await hre.upgrades.erc1967.getAdminAddress(
        contractAddress
      );
      const implementationAddress =
        await hre.upgrades.erc1967.getImplementationAddress(contractAddress);

      console.log(`HAPI Core upgraded`, {
        contract: contract.address,
        admin: adminAddress,
        implementation: implementationAddress,
      });
    } catch (error) {
      console.error(`${error}`);
      process.exit(1);
    }
  });

async function setup(
  hre: HardhatRuntimeEnvironment
): Promise<{ signer: HardhatEthersSigner; token: Token; hapiCore: HapiCore }> {
  let [signer] = await hre.ethers.getSigners();

  console.log("==> Environment setup");

  if (!signer) {
    throw new Error("No signer found");
  } else {
    console.log(`Signer: ${signer.address}`);
  }

  const network = await hre.ethers.provider.getNetwork();
  console.log(`Network: '${network.name}' (${network.chainId})`);

  const coinBalance = await hre.ethers.provider.getBalance(signer.address);
  console.log(`Balance: ${hre.ethers.formatEther(coinBalance)} ETH`);

  let token: Token;
  {
    const tokenAddress = process.env.TOKEN_ADDRESS;

    if (!tokenAddress || !hre.ethers.isAddress(tokenAddress)) {
      throw new Error("No token address found (use TOKEN_ADDRESS env var)");
    }

    const tokenFactory = await hre.ethers.getContractFactory("Token");
    token = tokenFactory.attach(tokenAddress) as Token;

    console.log(`Token: ${tokenAddress}`);

    const [tokenBalance, decimals, symbol] = await Promise.all([
      token.balanceOf(signer.address),
      token.decimals(),
      token.symbol(),
    ]);

    console.log(
      `Token balance: ${hre.ethers.formatUnits(
        tokenBalance,
        decimals
      )} ${symbol}`
    );
  }

  let hapiCore: HapiCore;
  {
    const contractAddress = process.env.CONTRACT_ADDRESS;

    if (!contractAddress || !hre.ethers.isAddress(contractAddress)) {
      throw new Error(
        "No contract address found (use CONTRACT_ADDRESS env var)"
      );
    }

    const hapiCoreFactory = await hre.ethers.getContractFactory("HapiCore");
    hapiCore = hapiCoreFactory.attach(contractAddress) as unknown as HapiCore;

    console.log(`HAPI Core: ${contractAddress}`);
  }

  console.log();

  return { signer, token, hapiCore };
}

async function trackTransaction(response: ContractTransactionResponse) {
  console.log("\n==> Executing transaction");
  console.log(`Transaction: ${response.hash}`);
  console.log("Waiting for transaction confirmation...");
  await response.wait();
  console.log(`Finished with ${await response.confirmations()} confirmation(s)`);
}

task("update-stake-configuration", "Updates the stake configuration")
  .addParam("unlockDuration", "Unlock duration")
  .addParam("validatorStake", "Validator stake")
  .addParam("tracerStake", "Tracer stake")
  .addParam("publisherStake", "Publisher stake")
  .addParam("authorityStake", "Authority stake")
  .setAction(async (args, hre) => {
    try {
      const { hapiCore, token } = await setup(hre);

      console.log("==> Updating stake configuration");

      const [name, symbol, decimals] = await Promise.all([
        token.name(),
        token.symbol(),
        token.decimals(),
      ]);

      console.log(`Token: ${name} (${symbol})`);

      console.log(`Unlock duration: ${args.unlockDuration} seconds`);

      const validatorStake = hre.ethers.parseUnits(
        args.validatorStake,
        decimals
      );
      console.log(
        `Validator stake: ${hre.ethers.formatUnits(
          validatorStake,
          decimals
        )} ${symbol}`
      );

      const tracerStake = hre.ethers.parseUnits(args.tracerStake, decimals);
      console.log(
        `Tracer stake: ${hre.ethers.formatUnits(
          tracerStake,
          decimals
        )} ${symbol}`
      );

      const publisherStake = hre.ethers.parseUnits(
        args.publisherStake,
        decimals
      );
      console.log(
        `Publisher stake: ${hre.ethers.formatUnits(
          publisherStake,
          decimals
        )} ${symbol}`
      );

      const authorityStake = hre.ethers.parseUnits(
        args.authorityStake,
        decimals
      );
      console.log(
        `Authority stake: ${hre.ethers.formatUnits(
          authorityStake,
          decimals
        )} ${symbol}`
      );

      const response = await hapiCore.updateStakeConfiguration(
        await token.getAddress(),
        args.unlockDuration,
        validatorStake,
        tracerStake,
        publisherStake,
        authorityStake
      );

      await trackTransaction(response);
    } catch (error) {
      console.error(`${error}`);
      process.exit(1);
    }
  });

export default config;
