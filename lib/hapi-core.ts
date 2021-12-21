import { Program, web3, BN, Provider, Coder } from "@project-serum/anchor";

import { IDL } from "../target/types/hapi_core";
import { bufferFromString } from "./buffer";

export function initHapiCore(
  hapiCoreProgramId: string | web3.PublicKey,
  provider?: Provider
) {
  const programId =
    typeof hapiCoreProgramId === "string"
      ? new web3.PublicKey(hapiCoreProgramId)
      : hapiCoreProgramId;

  const program = new Program(IDL, programId, provider);

  const coder = new Coder(IDL);

  async function findCommunityTokenSignerAddress(community: web3.PublicKey) {
    return web3.PublicKey.findProgramAddress(
      [bufferFromString("community_stash"), community.toBytes()],
      programId
    );
  }

  async function findNetworkAddress(community: web3.PublicKey, name: string) {
    return web3.PublicKey.findProgramAddress(
      [
        bufferFromString("network"),
        community.toBytes(),
        bufferFromString(name, 32),
      ],
      programId
    );
  }

  async function findNetworkRewardSignerAddress(network: web3.PublicKey) {
    return web3.PublicKey.findProgramAddress(
      [bufferFromString("network_reward"), network.toBytes()],
      programId
    );
  }

  async function findReporterAddress(
    community: web3.PublicKey,
    pubkey: web3.PublicKey
  ) {
    return web3.PublicKey.findProgramAddress(
      [bufferFromString("reporter"), community.toBytes(), pubkey.toBytes()],
      programId
    );
  }

  async function findReporterRewardAddress(
    network: web3.PublicKey,
    reporter: web3.PublicKey
  ) {
    return web3.PublicKey.findProgramAddress(
      [
        bufferFromString("reporter_reward"),
        network.toBytes(),
        reporter.toBytes(),
      ],
      programId
    );
  }

  async function findCaseAddress(community: web3.PublicKey, caseId: BN) {
    return web3.PublicKey.findProgramAddress(
      [
        bufferFromString("case"),
        community.toBytes(),
        new Uint8Array(caseId.toArray("le", 8)),
      ],
      programId
    );
  }

  async function findAddressAddress(
    network: web3.PublicKey,
    pubkey: web3.PublicKey
  ) {
    return web3.PublicKey.findProgramAddress(
      [bufferFromString("address"), network.toBytes(), pubkey.toBytes()],
      programId
    );
  }

  async function findAssetAddress(
    network: web3.PublicKey,
    mint: web3.PublicKey,
    assetId: Buffer | Uint8Array
  ) {
    return web3.PublicKey.findProgramAddress(
      [bufferFromString("asset"), network.toBytes(), mint.toBuffer(), assetId],
      programId
    );
  }

  return {
    ...program,
    programId,
    coder,
    pda: {
      findNetworkAddress,
      findNetworkRewardSignerAddress,
      findReporterAddress,
      findReporterRewardAddress,
      findCaseAddress,
      findAddressAddress,
      findAssetAddress,
      findCommunityTokenSignerAddress,
    },
  };
}
