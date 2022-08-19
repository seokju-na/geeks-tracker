import { createMemoryHistory, ReactLocation } from '@tanstack/react-location';

// NOTE: use memory history because we uses file protocol for serving webview.
export const history = createMemoryHistory();

export const location = new ReactLocation({
  history,
});
