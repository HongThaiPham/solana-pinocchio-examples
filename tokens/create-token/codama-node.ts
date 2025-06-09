import {
  rootNode,
  programNode,
  instructionNode,
  constantDiscriminatorNode,
  constantValueNode,
  numberTypeNode,
  numberValueNode,
  accountNode,
  structTypeNode,
  structFieldTypeNode,
  instructionArgumentNode,
  instructionAccountNode,
  publicKeyValueNode,
  fixedSizeTypeNode,
  stringTypeNode,
  publicKeyTypeNode,
} from 'codama';

export const root = rootNode(
  programNode({
    name: 'create-token',
    publicKey: 'Es5r1Qu1cAFFDFVKh7Usvd2gjQbhoQ58Yo1rkX8PaUr1',
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
            name: 'tokenDecimals',
            type: numberTypeNode('u8'),
            docs: ['The number of decimals for the token.'],
          }),
          instructionArgumentNode({
            name: 'mintAuthority',
            type: publicKeyTypeNode(),
            docs: ['The authority that can mint new tokens.'],
          }),
          instructionArgumentNode({
            name: 'freezeAuthority',
            type: publicKeyTypeNode(),
            docs: ['The authority that can freeze token accounts.'],
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
            docs: ['The address info account to create'],
          }),
          instructionAccountNode({
            name: 'tokenProgram',
            defaultValue: publicKeyValueNode(
              'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
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
