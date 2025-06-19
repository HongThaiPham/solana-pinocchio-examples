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
  accountNode,
  structTypeNode,
  structFieldTypeNode,
  fixedSizeTypeNode,
  stringTypeNode,
} from 'codama';

export const root = rootNode(
  programNode({
    name: 'close-account',
    publicKey: 'H9ZpziEUkrhakmLKaFXeokJFhTFm69jJ8aVSso43PopB',
    version: '1.0.0',
    accounts: [
      accountNode({
        name: 'user',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        data: structTypeNode([
          structFieldTypeNode({
            name: 'name',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 64),
          }),
        ]),
      }),
    ],
    instructions: [
      instructionNode({
        name: 'createUser',
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
            name: 'name',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 64),
            docs: ['The name of the user to create'],
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'payer',
            isSigner: true,
            isWritable: true,
            docs: ['The account that will pay for the transaction'],
          }),
          instructionAccountNode({
            name: 'targetAccount',
            isSigner: false,
            isWritable: true,
            docs: ['The account to create'],
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
        name: 'closeUser',
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
        ],
        accounts: [
          instructionAccountNode({
            name: 'payer',
            isSigner: true,
            isWritable: true,
            docs: ['The account that will pay for the transaction'],
          }),
          instructionAccountNode({
            name: 'targetAccount',
            isSigner: false,
            isWritable: true,
            docs: ['The account to close'],
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
