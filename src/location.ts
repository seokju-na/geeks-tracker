import { createMemoryHistory, ReactLocation } from '@tanstack/react-location';

export const location = new ReactLocation({
  // NOTE: use memory history because we uses file protocol for serving webview.
  history: createMemoryHistory(),
});
