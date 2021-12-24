import { web3, utils } from "@project-serum/anchor";
import { encode as eip55encode } from "eip55";

export function bufferFromString(str: string, bufferSize?: number) {
  const utf = utils.bytes.utf8.encode(str);

  if (!bufferSize || utf.byteLength === bufferSize) {
    return Buffer.from(utf);
  }

  if (bufferSize && utf.byteLength > bufferSize) {
    throw RangeError("Buffer size too small to fit the string");
  }

  return Buffer.concat(
    [Buffer.from(utf), Buffer.alloc(bufferSize - utf.byteLength)],
    bufferSize
  );
}

export function stringFromArray(array: number[]) {
  return (
    utils.bytes.utf8
      .decode(new Uint8Array(array))
      // eslint-disable-next-line no-control-regex
      .replace(/\x00/g, "")
  );
}

export function pubkeyFromHex(data: string): web3.PublicKey {
  const bytes = utils.bytes.hex.decode(data);

  const paddedBytes = Buffer.concat(
    [bytes, Buffer.alloc(32 - bytes.length)],
    32
  );

  return new web3.PublicKey(paddedBytes);
}

export function pubkeyFromBase58(data: string): web3.PublicKey {
  const bytes = utils.bytes.bs58.decode(data);

  const paddedBytes = Buffer.concat(
    [bytes, Buffer.alloc(32 - bytes.length)],
    32
  );

  return new web3.PublicKey(paddedBytes);
}

export function pubkeyToBitcoinAddress(pubkey: web3.PublicKey): string {
  const bytes = pubkey.toBuffer().subarray(0, 25);

  return utils.bytes.bs58.encode(bytes);
}

export function pubkeyToEthereumAddress(pubkey: web3.PublicKey): string {
  const bytes = pubkey.toBuffer().subarray(0, 20);

  return eip55encode(utils.bytes.hex.encode(bytes));
}
