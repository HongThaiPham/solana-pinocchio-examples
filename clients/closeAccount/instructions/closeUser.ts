/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/kit';
import { CLOSE_ACCOUNT_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CLOSE_USER_DISCRIMINATOR = 1;

export function getCloseUserDiscriminatorBytes() {
  return getU8Encoder().encode(CLOSE_USER_DISCRIMINATOR);
}

export type CloseUserInstruction<
  TProgram extends string = typeof CLOSE_ACCOUNT_PROGRAM_ADDRESS,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountTargetAccount extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountTargetAccount extends string
        ? WritableAccount<TAccountTargetAccount>
        : TAccountTargetAccount,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CloseUserInstructionData = { discriminator: number };

export type CloseUserInstructionDataArgs = {};

export function getCloseUserInstructionDataEncoder(): Encoder<CloseUserInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 1 }),
  );
}

export function getCloseUserInstructionDataDecoder(): Decoder<CloseUserInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getCloseUserInstructionDataCodec(): Codec<
  CloseUserInstructionDataArgs,
  CloseUserInstructionData
> {
  return combineCodec(
    getCloseUserInstructionDataEncoder(),
    getCloseUserInstructionDataDecoder(),
  );
}

export type CloseUserInput<
  TAccountPayer extends string = string,
  TAccountTargetAccount extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The account that will pay for the transaction */
  payer: TransactionSigner<TAccountPayer>;
  /** The account to close */
  targetAccount: Address<TAccountTargetAccount>;
  /** System Program used to open our new account */
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getCloseUserInstruction<
  TAccountPayer extends string,
  TAccountTargetAccount extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof CLOSE_ACCOUNT_PROGRAM_ADDRESS,
>(
  input: CloseUserInput<
    TAccountPayer,
    TAccountTargetAccount,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress },
): CloseUserInstruction<
  TProgramAddress,
  TAccountPayer,
  TAccountTargetAccount,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? CLOSE_ACCOUNT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    targetAccount: { value: input.targetAccount ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.targetAccount),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCloseUserInstructionDataEncoder().encode({}),
  } as CloseUserInstruction<
    TProgramAddress,
    TAccountPayer,
    TAccountTargetAccount,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCloseUserInstruction<
  TProgram extends string = typeof CLOSE_ACCOUNT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The account that will pay for the transaction */
    payer: TAccountMetas[0];
    /** The account to close */
    targetAccount: TAccountMetas[1];
    /** System Program used to open our new account */
    systemProgram: TAccountMetas[2];
  };
  data: CloseUserInstructionData;
};

export function parseCloseUserInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>,
): ParsedCloseUserInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      payer: getNextAccount(),
      targetAccount: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCloseUserInstructionDataDecoder().decode(instruction.data),
  };
}
