import { web3 } from "@project-serum/anchor";

export function pubkeyFromHex(hex: string): web3.PublicKey {
  return web3.PublicKey.decodeUnchecked(Buffer.from(hex, "hex"));
}
