import { errors } from "../../target/idl/hapi_core.json";

export function errorRegexp(code: number, instruction: number = 0) {
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

export enum AnchorError {
  InstructionMissing = 100,
  InstructionFallbackNotFound,
  InstructionDidNotDeserialize,
  InstructionDidNotSerialize,
  IdlInstructionStub = 1000,
  IdlInstructionInvalidProgram,
  ConstraintMut = 2000,
  ConstraintHasOne,
  ConstraintSigner,
  ConstraintRaw,
  ConstraintOwner,
  ConstraintRentExempt,
  ConstraintSeeds,
  ConstraintExecutable,
  ConstraintState,
  ConstraintAssociated,
  ConstraintAssociatedInit,
  ConstraintClose,
  ConstraintAddress,
  ConstraintZero,
  ConstraintTokenMint,
  ConstraintTokenOwner,
  ConstraintMintMintAuthority,
  ConstraintMintFreezeAuthority,
  ConstraintMintDecimals,
  ConstraintSpace,
  AccountDiscriminatorAlreadySet = 3000,
  AccountDiscriminatorNotFound,
  AccountDiscriminatorMismatch,
  AccountDidNotDeserialize,
  AccountDidNotSerialize,
  AccountNotEnoughKeys,
  AccountNotMutable,
  AccountNotProgramOwned,
  InvalidProgramId,
  InvalidProgramExecutable,
  AccountNotSigner,
  AccountNotSystemOwned,
  AccountNotInitialized,
  AccountNotProgramData,
  StateInvalidAddress = 4000,
  Deprecated = 5000,
}

export function anchorError(code: AnchorError) {
  return errorRegexp(code);
}
