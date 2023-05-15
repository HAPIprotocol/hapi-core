import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HapiCore } from "../target/types/hapi_core";

describe("hapi_core", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HapiCore as Program<HapiCore>;

  it("Is initialized!", async () => {
  });
});
