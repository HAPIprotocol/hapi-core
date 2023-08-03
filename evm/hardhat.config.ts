import { HardhatUserConfig, task } from "hardhat/config";
import "@openzeppelin/hardhat-upgrades";
import "@nomicfoundation/hardhat-toolbox";
import { Contract } from "ethers";

const HARDHAT_NETWORK = process.env.HARDHAT_NETWORK || "hardhat";
const HARDHAT_LOCALHOST_URL =
  process.env.HARDHAT_LOCALHOST_URL || "http://127.0.0.1:8545";

const config: HardhatUserConfig = {
  defaultNetwork: HARDHAT_NETWORK,
  typechain: {
    target: "ethers-v6",
  },
  solidity: {
    version: "0.8.18",
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

    console.log(`Using wallet: ${signer.address}`);

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
      contract: contract.address,
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

      // await hre.upgrades.forceImport(args.address, HapiCore as any);

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

export default config;
