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
  arrayTypeNode,
  fixedCountNode,
} from 'codama';

export const root = rootNode(
  programNode({
    name: 'favorites',
    publicKey: '21cBfH1aaKVeq86icK5pq51FM3ak2QQJ1TWoMLcTTK34',
    version: '1.0.0',
    accounts: [
      accountNode({
        name: 'favorites',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        data: structTypeNode([
          structFieldTypeNode({
            name: 'number',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 8),
          }),
          structFieldTypeNode({
            name: 'color',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          structFieldTypeNode({
            name: 'hobbies',
            type: arrayTypeNode(
              fixedSizeTypeNode(stringTypeNode('utf8'), 50),
              fixedCountNode(5)
            ),
          }),
          structFieldTypeNode({
            name: 'bump',
            type: numberTypeNode('u8'),
          }),
        ]),
      }),
    ],
    instructions: [
      instructionNode({
        name: 'createPda',
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
            name: 'number',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 8),
          }),
          instructionArgumentNode({
            name: 'color',
            type: fixedSizeTypeNode(stringTypeNode('utf8'), 50),
          }),
          instructionArgumentNode({
            name: 'hobbies',
            type: arrayTypeNode(
              fixedSizeTypeNode(stringTypeNode('utf8'), 50),
              fixedCountNode(5)
            ),
          }),
          instructionArgumentNode({
            name: 'bump',
            type: numberTypeNode('u8'),
          }),
        ],
        accounts: [
          instructionAccountNode({
            name: 'user',
            isSigner: true,
            isWritable: true,
            docs: ['The owner of the favorites account'],
          }),
          instructionAccountNode({
            name: 'favorites',
            isSigner: false,
            isWritable: true,
            docs: ['The favorites account to create'],
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
        name: 'getPda',
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
            name: 'user',
            isSigner: true,
            isWritable: true,
            docs: ['The owner of the favorites account'],
          }),
          instructionAccountNode({
            name: 'favorites',
            isSigner: false,
            isWritable: false,
            docs: ['The favorites account to create'],
          }),
        ],
      }),
    ],
  })
);
