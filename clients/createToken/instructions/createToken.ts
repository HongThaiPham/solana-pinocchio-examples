/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
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
  type WritableSignerAccount,
} from '@solana/kit';
import { CREATE_TOKEN_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CREATE_TOKEN_DISCRIMINATOR = 0;

export function getCreateTokenDiscriminatorBytes() {
  return getU8Encoder().encode(CREATE_TOKEN_DISCRIMINATOR);
}

export type CreateTokenInstruction<
  TProgram extends string = typeof CREATE_TOKEN_PROGRAM_ADDRESS,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountMint extends string | IAccountMeta<string> = string,
  TAccountTokenProgram extends
    | string
    | IAccountMeta<string> = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
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
      TAccountMint extends string
        ? WritableSignerAccount<TAccountMint> & IAccountSignerMeta<TAccountMint>
        : TAccountMint,
      TAccountTokenProgram extends string
        ? ReadonlyAccount<TAccountTokenProgram>
        : TAccountTokenProgram,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type CreateTokenInstructionData = {
  discriminator: number;
  /** The number of decimals for the token. */
  tokenDecimals: number;
  /** The authority that can mint new tokens. */
  mintAuthority: Address;
  /** The authority that can freeze token accounts. */
  freezeAuthority: Address;
};

export type CreateTokenInstructionDataArgs = {
  /** The number of decimals for the token. */
  tokenDecimals: number;
  /** The authority that can mint new tokens. */
  mintAuthority: Address;
  /** The authority that can freeze token accounts. */
  freezeAuthority: Address;
};

export function getCreateTokenInstructionDataEncoder(): Encoder<CreateTokenInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['tokenDecimals', getU8Encoder()],
      ['mintAuthority', getAddressEncoder()],
      ['freezeAuthority', getAddressEncoder()],
    ]),
    (value) => ({ ...value, discriminator: 0 }),
  );
}

export function getCreateTokenInstructionDataDecoder(): Decoder<CreateTokenInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['tokenDecimals', getU8Decoder()],
    ['mintAuthority', getAddressDecoder()],
    ['freezeAuthority', getAddressDecoder()],
  ]);
}

export function getCreateTokenInstructionDataCodec(): Codec<
  CreateTokenInstructionDataArgs,
  CreateTokenInstructionData
> {
  return combineCodec(
    getCreateTokenInstructionDataEncoder(),
    getCreateTokenInstructionDataDecoder(),
  );
}

export type CreateTokenInput<
  TAccountPayer extends string = string,
  TAccountMint extends string = string,
  TAccountTokenProgram extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** The payer of the transaction */
  payer: TransactionSigner<TAccountPayer>;
  /** The address info account to create */
  mint: TransactionSigner<TAccountMint>;
  /** The token program that will handle the mint */
  tokenProgram?: Address<TAccountTokenProgram>;
  /** System Program used to open our new account */
  systemProgram?: Address<TAccountSystemProgram>;
  tokenDecimals: CreateTokenInstructionDataArgs['tokenDecimals'];
  mintAuthority: CreateTokenInstructionDataArgs['mintAuthority'];
  freezeAuthority: CreateTokenInstructionDataArgs['freezeAuthority'];
};

export function getCreateTokenInstruction<
  TAccountPayer extends string,
  TAccountMint extends string,
  TAccountTokenProgram extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof CREATE_TOKEN_PROGRAM_ADDRESS,
>(
  input: CreateTokenInput<
    TAccountPayer,
    TAccountMint,
    TAccountTokenProgram,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress },
): CreateTokenInstruction<
  TProgramAddress,
  TAccountPayer,
  TAccountMint,
  TAccountTokenProgram,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? CREATE_TOKEN_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    mint: { value: input.mint ?? null, isWritable: true },
    tokenProgram: { value: input.tokenProgram ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.tokenProgram.value) {
    accounts.tokenProgram.value =
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address<'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'>;
  }
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.mint),
      getAccountMeta(accounts.tokenProgram),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getCreateTokenInstructionDataEncoder().encode(
      args as CreateTokenInstructionDataArgs,
    ),
  } as CreateTokenInstruction<
    TProgramAddress,
    TAccountPayer,
    TAccountMint,
    TAccountTokenProgram,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedCreateTokenInstruction<
  TProgram extends string = typeof CREATE_TOKEN_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The payer of the transaction */
    payer: TAccountMetas[0];
    /** The address info account to create */
    mint: TAccountMetas[1];
    /** The token program that will handle the mint */
    tokenProgram: TAccountMetas[2];
    /** System Program used to open our new account */
    systemProgram: TAccountMetas[3];
  };
  data: CreateTokenInstructionData;
};

export function parseCreateTokenInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>,
): ParsedCreateTokenInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
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
      mint: getNextAccount(),
      tokenProgram: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getCreateTokenInstructionDataDecoder().decode(instruction.data),
  };
}
