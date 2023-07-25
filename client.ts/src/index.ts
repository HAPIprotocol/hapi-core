import { HapiCoreEvm, HapiCoreNear, HapiCoreSolana } from "./implementations";
import { HapiCoreConnectionOptions, HapiCoreNetwork } from "./interface";

export function connectHapiCore(options: HapiCoreConnectionOptions) {
  switch (options.network) {
    case HapiCoreNetwork.Ethereum || HapiCoreNetwork.BSC:
      return new HapiCoreEvm(options);
    case HapiCoreNetwork.Solana || HapiCoreNetwork.Bitcoin:
      return new HapiCoreSolana(options);
    case HapiCoreNetwork.NEAR:
      return new HapiCoreNear(options);
    default:
      throw new Error(`Unsupported network: ${options.network}`);
  }
}
