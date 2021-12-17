import { errors } from "../../target/idl/hapi_core.json";

export function errorRegexp(code: number, instruction = 0) {
  return new RegExp(
    `failed to send transaction: Transaction simulation failed: Error processing Instruction ${instruction}: custom program error: 0x${code.toString(
      16
    )}`
  );
}

export function programError(name: string): RegExp {
  const error = errors.find((error) => error.name === name);
  if (!error) {
    throw new Error(`Error "${name}" is not found`);
  }

  return errorRegexp(5700 + error.code);
}
