/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  getUtf8Decoder,
  getUtf8Encoder,
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
import { FAVORITES_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CREATE_PDA_DISCRIMINATOR = 0;

export function getCreatePdaDiscriminatorBytes() {
  return getU8Encoder().encode(CREATE_PDA_DISCRIMINATOR);
}

export type CreatePdaInstruction<
  TProgram extends string = typeof FAVORITES_PROGRAM_ADDRESS,
  TAccountUser extends string | IAccountMeta<string> = string,
  TAccountFavorites extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountUser extends string
        ? WritableSignerAccount<TAccountUser> & IAccountSignerMeta<TAccountUser>
        : TAccountUser,
      TAccountFavorites extends string
        ? WritableAccount<TAccountFavorites>
        : TAccountFavorites,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CreatePdaInstructionData = {
  discriminator: number;
  number: string;
  color: string;
  hobbies: Array<string>;
  bump: number;
};

export type CreatePdaInstructionDataArgs = {
  number: string;
  color: string;
  hobbies: Array<string>;
  bump: number;
};

export function getCreatePdaInstructionDataEncoder(): Encoder<CreatePdaInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['number', fixEncoderSize(getUtf8Encoder(), 8)],
      ['color', fixEncoderSize(getUtf8Encoder(), 50)],
      [
        'hobbies',
        getArrayEncoder(fixEncoderSize(getUtf8Encoder(), 50), { size: 5 }),
      ],
      ['bump', getU8Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 0 }),
  );
}

export function getCreatePdaInstructionDataDecoder(): Decoder<CreatePdaInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['number', fixDecoderSize(getUtf8Decoder(), 8)],
    ['color', fixDecoderSize(getUtf8Decoder(), 50)],
    [
      'hobbies',
      getArrayDecoder(fixDecoderSize(getUtf8Decoder(), 50), { size: 5 }),
    ],
    ['bump', getU8Decoder()],
  ]);
}

export function getCreatePdaInstructionDataCodec(): Codec<
  CreatePdaInstructionDataArgs,
  CreatePdaInstructionData
> {
  return combineCodec(
    getCreatePdaInstructionDataEncoder(),
    getCreatePdaInstructionDataDecoder(),
  );
}

export type CreatePdaInput<
  TAccountUser extends string = string,
  TAccountFavorites extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The owner of the favorites account */
  user: TransactionSigner<TAccountUser>;
  /** The favorites account to create */
  favorites: Address<TAccountFavorites>;
  /** System Program used to open our new class account */
  systemProgram?: Address<TAccountSystemProgram>;
  number: CreatePdaInstructionDataArgs['number'];
  color: CreatePdaInstructionDataArgs['color'];
  hobbies: CreatePdaInstructionDataArgs['hobbies'];
  bump: CreatePdaInstructionDataArgs['bump'];
};

export function getCreatePdaInstruction<
  TAccountUser extends string,
  TAccountFavorites extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof FAVORITES_PROGRAM_ADDRESS,
>(
  input: CreatePdaInput<TAccountUser, TAccountFavorites, TAccountSystemProgram>,
  config?: { programAddress?: TProgramAddress },
): CreatePdaInstruction<
  TProgramAddress,
  TAccountUser,
  TAccountFavorites,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? FAVORITES_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    user: { value: input.user ?? null, isWritable: true },
    favorites: { value: input.favorites ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.user),
      getAccountMeta(accounts.favorites),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreatePdaInstructionDataEncoder().encode(
      args as CreatePdaInstructionDataArgs,
    ),
  } as CreatePdaInstruction<
    TProgramAddress,
    TAccountUser,
    TAccountFavorites,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCreatePdaInstruction<
  TProgram extends string = typeof FAVORITES_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The owner of the favorites account */
    user: TAccountMetas[0];
    /** The favorites account to create */
    favorites: TAccountMetas[1];
    /** System Program used to open our new class account */
    systemProgram: TAccountMetas[2];
  };
  data: CreatePdaInstructionData;
};

export function parseCreatePdaInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>,
): ParsedCreatePdaInstruction<TProgram, TAccountMetas> {
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
      user: getNextAccount(),
      favorites: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreatePdaInstructionDataDecoder().decode(instruction.data),
  };
}
