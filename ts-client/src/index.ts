import { HapiCoreEvm } from "./implementations/evm";
import { HapiCoreNear } from "./implementations/near";
import { HapiCoreSolana } from "./implementations/solana";

export enum HapiCoreNetwork {
  Ethereum = "ethereum",
  BSC = "bsc",
  Solana = "solana",
  Bitcoin = "bitcoin",
  NEAR = "near",
}

export const HapiCoreAddresses: {
  [key in HapiCoreNetwork]: string;
} = {
  [HapiCoreNetwork.Ethereum]: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
  [HapiCoreNetwork.BSC]: "",
  [HapiCoreNetwork.Solana]: "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6",
  [HapiCoreNetwork.Bitcoin]: "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6",
  [HapiCoreNetwork.NEAR]: "core.hapiprotocol.near",
};

export function connectHapiCore(network: HapiCoreNetwork, provider: any) {
  switch (network) {
    case HapiCoreNetwork.Ethereum || HapiCoreNetwork.BSC:
      return new HapiCoreEvm(HapiCoreAddresses[network], provider);
    case HapiCoreNetwork.Solana || HapiCoreNetwork.Bitcoin:
      return new HapiCoreSolana(HapiCoreAddresses[network], network, provider);
    case HapiCoreNetwork.NEAR:
      return new HapiCoreNear(HapiCoreAddresses[network], provider);
    default:
      throw new Error(`Unsupported network: ${network}`);
  }
}
