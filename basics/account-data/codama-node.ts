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
} from 'codama';

export const root = rootNode(
  programNode({
    name: 'account-data',
    publicKey: 'EAUvJAw61MTaJbyV4tqFB4dEZuYHdYrtpGQ35hDsQ6Dw',
    version: '1.0.0',
    accounts: [
      accountNode({
        name: 'addressInfo',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        data: structTypeNode([
          structFieldTypeNode({
            name: 'name',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          structFieldTypeNode({
            name: 'houseNumber',
            type: numberTypeNode('u8'),
          }),
          structFieldTypeNode({
            name: 'street',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          structFieldTypeNode({
            name: 'city',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
        ]),
      }),
    ],
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
            name: 'name',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          instructionArgumentNode({
            name: 'houseNumber',
            type: numberTypeNode('u8'),
          }),
          instructionArgumentNode({
            name: 'street',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          instructionArgumentNode({
            name: 'city',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
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
            name: 'addressInfo',
            isSigner: true,
            isWritable: true,
            docs: ['The address info account to create'],
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
