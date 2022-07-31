import type { TauriCommand } from '@tauri-apps/api/helpers/tauri';
import { mockIPC as mockTauriIPC } from '@tauri-apps/api/mocks';
import { afterEach, beforeEach } from 'vitest';

type Handler = (args: Record<string, unknown>) => any;
const tauriEventHandlers = new Map<string, Handler>(); // handlers for tauri event
const handlers = new Map<string, Handler>();

beforeEach(() => {
  mockTauriIPC((command, args) => {
    if (command === 'tauri') {
      const module = (args as TauriCommand).__tauriModule;
      switch (module) {
        case 'Event': {
          const handler = tauriEventHandlers.get((args as any).message.event);
          return handler?.(args);
        }
        default:
          throw new Error('TODO');
      }
    }

    return handlers.get(command)?.(args);
  });
});

afterEach(() => {
  tauriEventHandlers.clear();
  handlers.clear();
});

export function mockIPC(command: string, handler: Handler) {
  if (command === 'tauri') {
    throw new Error(`Cannot mock "tauri" command. Use other mock instead.`);
  }
  handlers.set(command, handler);
}

export function mockTauriEvent(event: string, handler: Handler) {
  tauriEventHandlers.set(event, handler);
}
