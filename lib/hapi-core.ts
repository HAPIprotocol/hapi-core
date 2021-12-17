import { Program, web3, BN } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";

import { HapiCore as HapiCoreType } from "../target/types/hapi_core";
import { bufferFromString } from "./buffer";

const program = anchor["workspace"].HapiCore as Program<HapiCoreType>;

export async function findCommunityTokenSignerAddress(
  community: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("community_stash"), community.toBytes()],
    program.programId
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
    program.programId
  );
}

export async function findNetworkRewardSignerAddress(network: web3.PublicKey) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("network_reward"), network.toBytes()],
    program.programId
  );
}

export async function findReporterAddress(
  community: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("reporter"), community.toBytes(), pubkey.toBytes()],
    program.programId
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
    program.programId
  );
}

export async function findCaseAddress(community: web3.PublicKey, caseId: BN) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("case"), community.toBytes(), caseId.toBuffer("le", 8)],
    program.programId
  );
}

export async function findAddressAddress(
  network: web3.PublicKey,
  pubkey: web3.PublicKey
) {
  return web3.PublicKey.findProgramAddress(
    [bufferFromString("address"), network.toBytes(), pubkey.toBytes()],
    program.programId
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

export const HapiCore = {
  ...program,
  programId: program.programId,
  findNetworkAddress,
  findNetworkRewardSignerAddress,
  findReporterAddress,
  findReporterRewardAddress,
  findCaseAddress,
  findAddressAddress,
  findAssetAddress,
  findCommunityTokenSignerAddress,
};
