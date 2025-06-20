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
  type WritableSignerAccount,
} from '@solana/kit';
import { ACCOUNT_DATA_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CREATE_DISCRIMINATOR = 0;

export function getCreateDiscriminatorBytes() {
  return getU8Encoder().encode(CREATE_DISCRIMINATOR);
}

export type CreateInstruction<
  TProgram extends string = typeof ACCOUNT_DATA_PROGRAM_ADDRESS,
  TAccountOwner extends string | IAccountMeta<string> = string,
  TAccountAddressInfo extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountOwner extends string
        ? WritableSignerAccount<TAccountOwner> &
            IAccountSignerMeta<TAccountOwner>
        : TAccountOwner,
      TAccountAddressInfo extends string
        ? WritableSignerAccount<TAccountAddressInfo> &
            IAccountSignerMeta<TAccountAddressInfo>
        : TAccountAddressInfo,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CreateInstructionData = {
  discriminator: number;
  name: string;
  houseNumber: number;
  street: string;
  city: string;
};

export type CreateInstructionDataArgs = {
  name: string;
  houseNumber: number;
  street: string;
  city: string;
};

export function getCreateInstructionDataEncoder(): Encoder<CreateInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['name', fixEncoderSize(getUtf8Encoder(), 50)],
      ['houseNumber', getU8Encoder()],
      ['street', fixEncoderSize(getUtf8Encoder(), 50)],
      ['city', fixEncoderSize(getUtf8Encoder(), 50)],
    ]),
    (value) => ({ ...value, discriminator: 0 }),
  );
}

export function getCreateInstructionDataDecoder(): Decoder<CreateInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['name', fixDecoderSize(getUtf8Decoder(), 50)],
    ['houseNumber', getU8Decoder()],
    ['street', fixDecoderSize(getUtf8Decoder(), 50)],
    ['city', fixDecoderSize(getUtf8Decoder(), 50)],
  ]);
}

export function getCreateInstructionDataCodec(): Codec<
  CreateInstructionDataArgs,
  CreateInstructionData
> {
  return combineCodec(
    getCreateInstructionDataEncoder(),
    getCreateInstructionDataDecoder(),
  );
}

export type CreateInput<
  TAccountOwner extends string = string,
  TAccountAddressInfo extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The owner of the counter account. */
  owner: TransactionSigner<TAccountOwner>;
  /** The address info account to create */
  addressInfo: TransactionSigner<TAccountAddressInfo>;
  /** System Program used to open our new class account */
  systemProgram?: Address<TAccountSystemProgram>;
  name: CreateInstructionDataArgs['name'];
  houseNumber: CreateInstructionDataArgs['houseNumber'];
  street: CreateInstructionDataArgs['street'];
  city: CreateInstructionDataArgs['city'];
};

export function getCreateInstruction<
  TAccountOwner extends string,
  TAccountAddressInfo extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof ACCOUNT_DATA_PROGRAM_ADDRESS,
>(
  input: CreateInput<TAccountOwner, TAccountAddressInfo, TAccountSystemProgram>,
  config?: { programAddress?: TProgramAddress },
): CreateInstruction<
  TProgramAddress,
  TAccountOwner,
  TAccountAddressInfo,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? ACCOUNT_DATA_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    owner: { value: input.owner ?? null, isWritable: true },
    addressInfo: { value: input.addressInfo ?? null, isWritable: true },
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
      getAccountMeta(accounts.owner),
      getAccountMeta(accounts.addressInfo),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreateInstructionDataEncoder().encode(
      args as CreateInstructionDataArgs,
    ),
  } as CreateInstruction<
    TProgramAddress,
    TAccountOwner,
    TAccountAddressInfo,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCreateInstruction<
  TProgram extends string = typeof ACCOUNT_DATA_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The owner of the counter account. */
    owner: TAccountMetas[0];
    /** The address info account to create */
    addressInfo: TAccountMetas[1];
    /** System Program used to open our new class account */
    systemProgram: TAccountMetas[2];
  };
  data: CreateInstructionData;
};

export function parseCreateInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>,
): ParsedCreateInstruction<TProgram, TAccountMetas> {
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
      owner: getNextAccount(),
      addressInfo: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreateInstructionDataDecoder().decode(instruction.data),
  };
}
