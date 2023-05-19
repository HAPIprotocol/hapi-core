import { Program, web3, BN, Provider, utils } from "@coral-xyz/anchor";

import { IDL } from "../../target/types/hapi_core_solana";
import { padBuffer, pubkeyFromBase58 } from ".";
import { bufferFromString, addrToSeeds } from "./buffer";

export function encodeAddress(
  address: string
): Buffer {
  return padBuffer(Buffer.from(address), 64);
}

export function decodeAddress(
  address: Buffer | Uint8Array | number[],
): string {
  if (!(address instanceof Buffer)) {
    address = Buffer.from(address);
  }

  return address.filter((b) => b).toString();
}

export function initHapiCore(
  hapiCoreProgramId: string | web3.PublicKey,
  provider?: Provider
) {
  const programId =
    typeof hapiCoreProgramId === "string"
      ? new web3.PublicKey(hapiCoreProgramId)
      : hapiCoreProgramId;

  const program = new Program(IDL, programId, provider);

  async function findNetworkAddress(name: string) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("network"),
        bufferFromString(name, 32),
      ],
      programId
    );
  }

  async function findReporterAddress(
    pubkey: web3.PublicKey
  ) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("reporter"), pubkey.toBytes()],
      programId
    );
  }

  async function findCaseAddress(caseId: BN) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("case"),
        new Uint8Array(caseId.toArray("le", 8)),
      ],
      programId
    );
  }

  async function findAddressAddress(network: web3.PublicKey, address: Buffer) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("address"), network.toBytes(), ...addrToSeeds(address)],
      programId
    );
  }

  async function findAssetAddress(
    network: web3.PublicKey,
    mint: Buffer,
    assetId: Buffer | Uint8Array
  ) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("asset"),
        network.toBytes(),
        ...addrToSeeds(mint),
        assetId,
      ],
      programId
    );
  }

  return {
    ...program,
    programId,
    // coder,
    util: {
      encodeAddress,
      decodeAddress,
    },
    idl: IDL,
    pda: {
      findNetworkAddress,
      findReporterAddress,
      findCaseAddress,
      findAddressAddress,
      findAssetAddress,
    },
  };
}
