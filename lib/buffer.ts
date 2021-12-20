import * as anchor from "@project-serum/anchor";

export function bufferFromString(str: string, bufferSize?: number) {
  const utf = anchor.utils.bytes.utf8.encode(str);

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
  return anchor.utils.bytes.utf8.decode(new Uint8Array(array));
}
