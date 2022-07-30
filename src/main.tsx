import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css';
import { injectTheme } from './styles';

injectTheme();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const root = document.getElementById('root')!;
createRoot(root).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
