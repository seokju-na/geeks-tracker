import { parser } from './parser.js';
import { fileTests } from '@lezer/generator/test';
import { readdirSync, readFileSync } from 'node:fs';
import { describe, it } from 'vitest';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const cases = path.join(path.dirname(fileURLToPath(import.meta.url)), '__tests__');

for (const file of readdirSync(cases)) {
  const name = /^[^.]*/.exec(file)![0];
  describe(name, () => {
    for (const { name, run } of fileTests(readFileSync(path.join(cases, file), 'utf8'), file)) {
      it(name, () => run(parser));
    }
  });
}
