import { Program, web3, BN } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";

import { HapiCore as HapiCoreType } from "../target/types/hapi_core";
import { metadata } from "../target/idl/hapi_core.json";
import { bufferFromString } from "./buffer";

export const HAPI_CORE_PROGRAM_ID = new web3.PublicKey(metadata.address);

export async function findCommunityTokenSignerAddress(
  community: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("community_stash"), community.toBytes()],
    HAPI_CORE_PROGRAM_ID
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
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findNetworkRewardSignerAddress(network: web3.PublicKey) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("network_reward"), network.toBytes()],
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findReporterAddress(
  community: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("reporter"), community.toBytes(), pubkey.toBytes()],
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findReporterRewardAddress(
  network: web3.PublicKey,
  reporter: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [
      bufferFromString("reporter_reward"),
      network.toBytes(),
      reporter.toBytes(),
    ],
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findCaseAddress(community: web3.PublicKey, caseId: BN) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("case"), community.toBytes(), caseId.toBuffer("le", 8)],
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findAddressAddress(
  network: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("address"), network.toBytes(), pubkey.toBytes()],
    HAPI_CORE_PROGRAM_ID
  );
}

export async function findAssetAddress(
  network: web3.PublicKey,
  mint: web3.PublicKey,
  assetId: Buffer
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("asset"), network.toBytes(), mint.toBuffer(), assetId],
    HAPI_CORE_PROGRAM_ID
  );
}

const program = anchor["workspace"].HapiCore as Program<HapiCoreType>;

export const HapiCore = {
  ...program,
  programId: HAPI_CORE_PROGRAM_ID,
  findNetworkAddress,
  findNetworkRewardSignerAddress,
  findReporterAddress,
  findReporterRewardAddress,
  findCaseAddress,
  findAddressAddress,
  findAssetAddress,
  findCommunityTokenSignerAddress,
};
