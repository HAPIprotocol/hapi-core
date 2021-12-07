import { Program, web3, workspace, BN } from "@project-serum/anchor";

import { HapiCore } from "../target/types/hapi_core";
import { bufferFromString } from "./buffer";

const HapiCore = workspace.HapiCore as Program<HapiCore>;

export async function findCommunityTokenSignerAddress(
  community: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("community_stash"), community.toBytes()],
    HapiCore.programId
  );
}

export async function findNetworkAddress(
  community: web3.PublicKey,
  name: string
) {
  return web3.PublicKey.findProgramAddress(
    [
      bufferFromString("network"),
      community.toBytes(),
      bufferFromString(name, 32),
    ],
    HapiCore.programId
  );
}

export async function findReporterAddress(
  community: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("reporter"), community.toBytes(), pubkey.toBytes()],
    HapiCore.programId
  );
}

export async function findCaseAddress(community: web3.PublicKey, caseId: BN) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("case"), community.toBytes(), caseId.toBuffer("le", 8)],
    HapiCore.programId
  );
}

export async function findAddressAddress(
  network: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("address"), network.toBytes(), pubkey.toBytes()],
    HapiCore.programId
  );
}

export async function findAssetAddress(
  network: web3.PublicKey,
  mint: web3.PublicKey,
  assetId: Buffer
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("asset"), network.toBytes(), mint.toBuffer(), assetId],
    HapiCore.programId
  );
}

export const program = {
  ...HapiCore,
  programId: HapiCore.programId,
  findNetworkAddress,
  findReporterAddress,
  findCaseAddress,
  findAddressAddress,
  findAssetAddress,
  findCommunityTokenSignerAddress,
};
