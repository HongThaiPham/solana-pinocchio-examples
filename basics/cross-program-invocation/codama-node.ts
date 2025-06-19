import { COUNTER_PROGRAM_ADDRESS } from '@/clients/counter';
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
    name: 'cross-program-invocation',
    publicKey: 'HgeJhsevaynVUxZdwD5RJxuubfMagRmfkj9dHDHidwVY',
    version: '1.0.0',

    instructions: [
      instructionNode({
        name: 'create',
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
            name: 'initialValue',
            type: numberTypeNode('u64'),
          }),
          instructionArgumentNode({
            name: 'bump',
            type: numberTypeNode('u8'),
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'owner',
            isSigner: true,
            isWritable: true,
            docs: ['The owner of the counter account.'],
          }),
          instructionAccountNode({
            name: 'counter',
            isSigner: false,
            isWritable: true,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'counterProgram',
            defaultValue: publicKeyValueNode(
              COUNTER_PROGRAM_ADDRESS,
              'counterProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new class account'],
          }),
        ],
      }),
      instructionNode({
        name: 'increase',
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
            name: 'owner',
            isSigner: true,
            isWritable: true,
            docs: ['The owner of the counter account.'],
          }),
          instructionAccountNode({
            name: 'counter',
            isSigner: false,
            isWritable: true,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'counterProgram',
            defaultValue: publicKeyValueNode(
              COUNTER_PROGRAM_ADDRESS,
              'counterProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new class account'],
          }),
        ],
      }),
      instructionNode({
        name: 'decrease',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(1))
          ),
        ],
        arguments: [
          instructionArgumentNode({
            name: 'discriminator',
            type: numberTypeNode('u8'),
            defaultValue: numberValueNode(2),
            defaultValueStrategy: 'omitted',
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'owner',
            isSigner: true,
            isWritable: true,
            docs: ['The owner of the counter account.'],
          }),
          instructionAccountNode({
            name: 'counter',
            isSigner: false,
            isWritable: true,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'counterProgram',
            defaultValue: publicKeyValueNode(
              COUNTER_PROGRAM_ADDRESS,
              'counterProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['The counter account to be created.'],
          }),
          instructionAccountNode({
            name: 'systemProgram',
            defaultValue: publicKeyValueNode(
              '11111111111111111111111111111111',
              'systemProgram'
            ),
            isSigner: false,
            isWritable: false,
            docs: ['System Program used to open our new class account'],
          }),
        ],
      }),
    ],
  })
);
