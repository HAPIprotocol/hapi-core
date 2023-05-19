import * as uuid from "uuid";

export function uuidToBigInt(input: string): BigInt {
  uuid.parse(input);
  return BigInt("0x" + input.replace(/-/g, ""));
}
