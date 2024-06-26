import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  root: '.',
  esbuild: {
    tsconfigRaw: '{}',
  },
  test: {
    clearMocks: true,
    globals: true,
    testTimeout: 450000,
    setupFiles: ['dotenv/config', './tests/testSetup.ts'],
  },
  resolve: {
    alias: [{ 
      find: '~', 
      replacement: resolve(__dirname, 'tests'),
    }],
  },
});