import '@testing-library/jest-dom/vitest';
import { cleanup } from '@testing-library/react';
import { randomFillSync } from 'node:crypto';
import { afterEach, beforeAll } from 'vitest';

beforeAll(() => {
  Object.defineProperty(window, 'crypto', {
    value: {
      // @ts-ignore
      getRandomValues: (buffer) => {
        return randomFillSync(buffer);
      },
    },
  });
});

afterEach(() => {
  cleanup();
});
