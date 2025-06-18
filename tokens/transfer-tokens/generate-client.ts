import { createFromRoot } from 'codama';
import { renderVisitor, type RenderOptions } from '@codama/renderers-js';
import * as path from 'path';
import { root } from './codama-node';

const pathToGeneratedFolder = path.join(
  __dirname,
  '..',
  '..',
  'clients',
  root.program.name
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

const codama = createFromRoot(root);

codama.accept(renderVisitor(pathToGeneratedFolder, options));

console.log(
  `Generated code for program "${root.program.name.toUpperCase()}" at ${pathToGeneratedFolder}`
);
