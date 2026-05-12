import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [svelte({ hot: false })],
  resolve: {
    conditions: ['browser'],
  },
  test: {
    environment: 'jsdom',
    include: ['injection/**/__tests__/**/*.test.ts', 'src/**/__tests__/**/*.test.ts'],
    globals: false,
    setupFiles: ['./src/__tests__/setup.ts'],
  },
});
