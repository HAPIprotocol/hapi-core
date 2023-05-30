import { Program, web3, BN, Provider } from "@coral-xyz/anchor";
import { IDL, HapiCoreSolana } from "../target/types/hapi_core_solana";
import { padBuffer } from ".";
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

export class HapiCoreProgram {
  program: Program<HapiCoreSolana>;
  programId: web3.PublicKey;

  constructor (
    hapiCoreProgramId: string | web3.PublicKey,
    provider?: Provider) {
    this.programId =
      typeof hapiCoreProgramId === "string"
        ? new web3.PublicKey(hapiCoreProgramId)
        : hapiCoreProgramId;

    this.program = new Program(IDL, this.programId, provider);
  }

  public findProgramDataAddress() {
    return web3.PublicKey.findProgramAddressSync(
      [this.programId.toBytes()],
      new web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    );
  }

  public findNetworkAddress(name: string) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("network"),
        bufferFromString(name, 32),
      ],
      this.programId
    );
  }

  public findReporterAddress(
    network: web3.PublicKey,
    reporterId: BN
  ) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("reporter"), network.toBytes(), new Uint8Array(reporterId.toArray("le", 8))],
      this.programId
    );
  }

  public findCaseAddress(caseId: BN) {
    return web3.PublicKey.findProgramAddressSync(
      [
        bufferFromString("case"),
        new Uint8Array(caseId.toArray("le", 8)),
      ],
      this.programId
    );
  }

  public findAddressAddress(network: web3.PublicKey, address: Buffer) {
    return web3.PublicKey.findProgramAddressSync(
      [bufferFromString("address"), network.toBytes(), ...addrToSeeds(address)],
      this.programId
    );
  }

  public findAssetAddress(
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
      this.programId
    );
  }
}
