import { SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ADDRESS } from '@/clients/shared';
import {
  rootNode,
  programNode,
  instructionNode,
  constantDiscriminatorNode,
  constantValueNode,
  numberTypeNode,
  numberValueNode,
  instructionArgumentNode,
  instructionAccountNode,
  publicKeyValueNode,
  publicKeyTypeNode,
  fixedSizeTypeNode,
  stringTypeNode,
} from 'codama';
import { TOKEN_2022_PROGRAM_ADDRESS } from '@solana-program/token-2022';

export const root = rootNode(
  programNode({
    name: 'token-2022-basic',
    publicKey: 'DfKryFwKhmTHR7qjaUxKDsQR8BT5jdRJJ6tf4CP82eaC',
    version: '1.0.0',
    instructions: [
      instructionNode({
        name: 'createToken',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(0),
            defaultValueStrategy: 'omitted',
          }),
          instructionArgumentNode({
            name: 'tokenName',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 32),
            docs: ['The name of the user to create'],
          }),
          instructionArgumentNode({
            name: 'tokenDecimals',
            type: numberTypeNode('u8'),
            docs: ['The number of decimals for the token.'],
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'payer',
            isSigner: true,
            isWritable: true,
            docs: ['The payer of the transaction'],
          }),
          instructionAccountNode({
            name: 'mint',
            isSigner: true,
            isWritable: true,
            docs: ['The mint account that will be created'],
          }),
          instructionAccountNode({
            name: 'tokenProgram',
            defaultValue: publicKeyValueNode(
              TOKEN_2022_PROGRAM_ADDRESS,
              'tokenProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The token program that will handle the mint'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new account'],
          }),
        ],
      }),
      instructionNode({
        name: 'mintToken',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(1))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(1),
            defaultValueStrategy: 'omitted',
          }),
          instructionArgumentNode({
            name: 'amount',
            type: numberTypeNode('u64'),
            docs: ['The amount of tokens to mint.'],
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'mint_authority',
            isSigner: true,
            isWritable: true,
            docs: ['The authority that can mint new tokens.'],
          }),
          instructionAccountNode({
            name: 'mint',
            isSigner: false,
            isWritable: true,
            docs: ['The mint account to mint tokens to'],
          }),
          instructionAccountNode({
            name: 'to',
            isSigner: false,
            isWritable: false,
            docs: ['The wallet to mint tokens to'],
          }),
          instructionAccountNode({
            name: 'tokenAccount',
            isSigner: false,
            isWritable: true,
            docs: [
              'The token account to mint tokens to. If not provided, the associated token account will be used.',
            ],
          }),
          instructionAccountNode({
            name: 'associatedTokenProgram',
            defaultValue: publicKeyValueNode(
              SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ADDRESS,
              'associatedTokenProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The associated token program that will handle the mint'],
          }),
          instructionAccountNode({
            name: 'tokenProgram',
            defaultValue: publicKeyValueNode(
              TOKEN_2022_PROGRAM_ADDRESS,
              'tokenProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The token program that will handle the mint'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new account'],
          }),
        ],
      }),
      instructionNode({
        name: 'transfer',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(2))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(2),
            defaultValueStrategy: 'omitted',
          }),
          instructionArgumentNode({
            name: 'amount',
            type: numberTypeNode('u64'),
            docs: ['The amount of tokens to transfer.'],
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'from',
            isSigner: true,
            isWritable: true,
            docs: ['The wallet to transfer tokens from'],
          }),
          instructionAccountNode({
            name: 'mint',
            isSigner: false,
            isWritable: true,
            docs: ['The mint account to mint tokens to'],
          }),
          instructionAccountNode({
            name: 'to',
            isSigner: false,
            isWritable: false,
            docs: ['The wallet to mint tokens to'],
          }),
          instructionAccountNode({
            name: 'fromTokenAccount',
            isSigner: false,
            isWritable: true,
            docs: [
              'The token account to transfer tokens from. If not provided, the associated token account will be used.',
            ],
          }),
          instructionAccountNode({
            name: 'toTokenAccount',
            isSigner: false,
            isWritable: true,
            docs: [
              'The token account to transfer tokens to. If not provided, the associated token account will be used.',
            ],
          }),
          instructionAccountNode({
            name: 'associatedTokenProgram',
            defaultValue: publicKeyValueNode(
              SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ADDRESS,
              'associatedTokenProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The associated token program that will handle the mint'],
          }),
          instructionAccountNode({
            name: 'tokenProgram',
            defaultValue: publicKeyValueNode(
              TOKEN_2022_PROGRAM_ADDRESS,
              'tokenProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The token program that will handle the mint'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new account'],
          }),
        ],
      }),
    ],
  })
);
