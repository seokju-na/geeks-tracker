import { clearMocks } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';
import { randomFillSync } from 'node:crypto';
import * as React from 'react';

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  (window as any).crypto = {
    getRandomValues: (buffer: Buffer) => {
      return randomFillSync(buffer);
    },
  };
});

afterEach(() => {
  clearMocks();
});

// shims for jsx
// @ts-ignore
global.React = React;
