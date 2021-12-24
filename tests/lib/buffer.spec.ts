import { web3 } from "@project-serum/anchor";

import {
  bufferFromString,
  stringFromArray,
  pubkeyFromHex,
  pubkeyFromBase58,
  pubkeyToBitcoinAddress,
  pubkeyToEthereumAddress,
} from "../../lib/buffer";

describe("bufferFromString", () => {
  it("should work without size", () => {
    const str = "ethereum";

    const buffer = bufferFromString(str);

    expect(buffer.toString("hex")).toMatchInlineSnapshot(`"657468657265756d"`);
  });

  it("should work with size", () => {
    const str = "nera";
    const length = 32;

    const buffer = bufferFromString(str, length);

    expect(buffer.toString("hex")).toMatchInlineSnapshot(
      `"6e65726100000000000000000000000000000000000000000000000000000000"`
    );
    expect(buffer).toHaveLength(length);
  });

  it("should throw if the string size is larger than target length", () => {
    const str = "a long long time ago...";
    const length = 8;

    expect(() =>
      bufferFromString(str, length)
    ).toThrowErrorMatchingInlineSnapshot(
      `"Buffer size too small to fit the string"`
    );
  });
});

describe("stringFromArray", () => {
  it("should work with empty array", () => {
    const array = [];
    const str = stringFromArray(array);
    expect(str).toMatchInlineSnapshot(`""`);
  });

  it("should work with a normal ascii array", () => {
    const array = [107, 105, 108, 108, 106, 111, 121];
    const str = stringFromArray(array);
    expect(str).toMatchInlineSnapshot(`"killjoy"`);
  });

  it("should strip zero-code characters", () => {
    const array = [107, 105, 108, 108, 106, 111, 121, 0, 0, 0, 0, 0];
    const str = stringFromArray(array);
    expect(str).toMatchInlineSnapshot(`"killjoy"`);
  });
});

describe("pubkeyFromHex", () => {
  it("should work with a normal pubkey", () => {
    const og = new web3.PublicKey(
      "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6"
    );

    const pubkey = pubkeyFromHex(og.toBuffer().toString("hex"));

    expect(pubkey.toString()).toEqual(og.toString());
  });

  it("should work with padding", () => {
    const pubkey = pubkeyFromHex("a6579c62dba7dc");

    expect(pubkey.toString()).toMatchInlineSnapshot(
      `"CCLAMjwsSPLivLPfxPHesPqZqesGEPkJpwYMjSKfyJEB"`
    );
  });

  it("should work with an ethereum address", () => {
    const input = "0x8F03f1a3f10c05E7CCcF75C1Fd10168e06659Be7";
    const pubkey = pubkeyFromHex(input);

    expect(pubkey.toString()).toMatchInlineSnapshot(
      `"AdGnkVmVjxi3EcUNGDLkZwiRJvFrxMuBw78ydsnzmP99"`
    );

    expect(pubkeyToEthereumAddress(pubkey)).toEqual(input);
  });

  it("should work with zero address", () => {
    const pubkey = pubkeyFromHex(
      "0000000000000000000000000000000000000000000000000000000000000000"
    );

    expect(pubkey.toString()).toMatchInlineSnapshot(
      `"11111111111111111111111111111111"`
    );
  });
});

describe("pubkeyFromBase58", () => {
  it("should work with a normal pubkey", () => {
    const og = new web3.PublicKey(
      "hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6"
    );

    const pubkey = pubkeyFromBase58(og.toBase58());

    expect(pubkey.toString()).toEqual(og.toString());
  });

  it("should work with a bitcoin address", () => {
    const input = "3DPNFXGoe8QGiEXEApQ3QtHb8wM15VCQU3";
    const pubkey = pubkeyFromBase58(input);

    expect(pubkey.toString()).toMatchInlineSnapshot(
      `"NUW5jdxo9zmhmT1Wns2as2BeuJ6aYAPApzwtBKNFLW3"`
    );

    expect(pubkeyToBitcoinAddress(pubkey)).toEqual(input);
  });

  it("should work with zero address", () => {
    const pubkey = pubkeyFromBase58("1111111111111111111111111");

    expect(pubkey.toString()).toMatchInlineSnapshot(
      `"11111111111111111111111111111111"`
    );
  });
});
