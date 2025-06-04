import {
  constantDiscriminatorNode,
  constantValueNode,
  createFromRoot,
  instructionNode,
  numberTypeNode,
  numberValueNode,
  programNode,
  rootNode,
} from 'codama';
import { renderVisitor, type RenderOptions } from '@codama/renderers-js';
import * as path from 'path';
import * as fs from 'fs';

const root = rootNode(
  programNode({
    name: 'hello-solana',
    publicKey: 'FpFC3vEsjXKTrLweeD9PaG4HpTqMNJNoMvSVcZVJ8JCT',
    version: '1.0.0',
    accounts: [],
    instructions: [
      instructionNode({
        name: 'hello',
        discriminators: [
          constantDiscriminatorNode(
            constantValueNode(numberTypeNode('u8'), numberValueNode(0))
          ),
        ],
        arguments: [],
      }),
    ],
  })
);

const pathToGeneratedFolder = path.join(
  __dirname,
  '..',
  '..',
  'clients',
  'hello-solana'
);
const options: RenderOptions = {
  deleteFolderBeforeRendering: true,
  formatCode: true,
  prettierOptions: {
    parser: 'typescript',
    singleQuote: true,
    trailingComma: 'all',
    printWidth: 80,
  },
};

function preserveConfigFiles() {
  const filesToPreserve = [
    'package.json',
    'tsconfig.json',
    '.npmignore',
    'bun.lock',
  ];
  const preservedFiles = new Map();

  filesToPreserve.forEach((filename) => {
    const filePath = path.join(pathToGeneratedFolder, filename);
    const tempPath = path.join(pathToGeneratedFolder, `${filename}.temp`);

    if (fs.existsSync(filePath)) {
      fs.copyFileSync(filePath, tempPath);
      preservedFiles.set(filename, tempPath);
    }
  });

  return {
    restore: () => {
      preservedFiles.forEach((tempPath, filename) => {
        const filePath = path.join(pathToGeneratedFolder, filename);
        if (fs.existsSync(tempPath)) {
          fs.copyFileSync(tempPath, filePath);
          fs.unlinkSync(tempPath);
        }
      });
    },
  };
}

const codama = createFromRoot(root);
const configPreserver = preserveConfigFiles();
codama.accept(renderVisitor(pathToGeneratedFolder, options));

console.log(
  `Generated code for hello-solana client at ${pathToGeneratedFolder}`
);
