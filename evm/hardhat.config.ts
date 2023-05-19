import { HardhatUserConfig, task } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@openzeppelin/hardhat-upgrades";

const config: HardhatUserConfig = {
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
    mainnet: {
      url: `https://mainnet.infura.io/v3/${process.env.INFURA_KEY}`,
    },
    ropsten: {
      url: `https://ropsten.infura.io/v3/${process.env.INFURA_KEY}`,
    }
  },
  gasReporter: {
    currency: "ETH",
    showTimeSpent: true,
    enabled: true,
  },
};

task("deploy", "Deploys the HAPI Core contract").setAction(async (_, hre) => {
  try {
    let network = await hre.ethers.provider.getNetwork();

    console.log(`Deploying to '${network.name}' (${network.chainId})`);

    const HapiCore = await hre.ethers.getContractFactory("HapiCore");

    const contract = await hre.upgrades.deployProxy(HapiCore, [], {
      initializer: "initialize",
    });

    await contract.deployed();

    const adminAddress = await hre.upgrades.erc1967.getAdminAddress(
      contract.address
    );
    const implementationAddress =
      await hre.upgrades.erc1967.getImplementationAddress(contract.address);

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

task("upgrade", "Upgrades the HAPI Core contract")
  .addParam("address", "Contract address")
  .setAction(async (args, hre) => {
    try {
      let network = await hre.ethers.provider.getNetwork();

      console.log(`Deploying to '${network.name}' (${network.chainId})`);

      const HapiCore = await hre.ethers.getContractFactory("HapiCore");

      const contract = await hre.upgrades.upgradeProxy(args.address, HapiCore);

      await contract.deployed();

      const adminAddress = await hre.upgrades.erc1967.getAdminAddress(
        contract.address
      );
      const implementationAddress =
        await hre.upgrades.erc1967.getImplementationAddress(contract.address);

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
