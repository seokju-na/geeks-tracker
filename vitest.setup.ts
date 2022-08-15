import { clearMocks, mockWindows } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';
import { randomFillSync } from 'node:crypto';
import { afterEach, beforeAll, beforeEach } from 'vitest';
import { queryClient } from './src/queryClient';
import { location } from './src/location';

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  (window as any).crypto = {
    getRandomValues: (buffer: Buffer) => {
      return randomFillSync(buffer);
    },
  };
});

beforeEach(() => {
  mockWindows('main');
});

afterEach(() => {
  clearMocks();
  queryClient.clear();
  location.destroy();
});
