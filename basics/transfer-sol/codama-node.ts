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
} from 'codama';

export const root = rootNode(
  programNode({
    name: 'transfer-sol',
    publicKey: 'QBDA4wAjJpX1rmpW7g6eSdize5Dq4mHbnRxkfNQCWya',
    version: '1.0.0',

    instructions: [
      instructionNode({
        name: 'transferSolWithProgram',
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
            name: 'amount',
            type: numberTypeNode('u64'),
            docs: ['The amount of SOL to transfer'],
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
            name: 'recipient',
            isSigner: false,
            isWritable: true,
            docs: ['The account that will receive the SOL'],
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
        name: 'transferSolWithCpi',
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
            docs: ['The amount of SOL to transfer'],
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
            name: 'recipient',
            isSigner: false,
            isWritable: true,
            docs: ['The account that will receive the SOL'],
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
