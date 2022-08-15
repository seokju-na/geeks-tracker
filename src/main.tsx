import { QueryClientProvider } from '@tanstack/react-query';
import { listen } from '@tauri-apps/api/event';
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css';
import { location } from './location';
import { queryClient } from './queryClient';
import { injectTheme } from './styles';

// inject theme css
injectTheme();

// listen backend events
listen<{ to: string }>('geeks-tracker://navigate', e => {
  const next = location.buildNext(location.current.pathname, {
    to: e.payload.to,
  });

  location.navigate(next);
});

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const root = document.getElementById('root')!;
createRoot(root).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </React.StrictMode>
);
