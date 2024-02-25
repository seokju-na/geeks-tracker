import { defineConfig } from 'vite';
import { TanStackRouterVite } from '@tanstack/router-vite-plugin';
import react from '@vitejs/plugin-react';

const { NODE_ENV = 'development' } = process.env;

export default defineConfig({
  plugins: [react(), TanStackRouterVite()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  define: {
    PRODUCTION: (NODE_ENV === 'production').toString(),
  },
  css: {
    transformer: 'lightningcss',
  },
});
