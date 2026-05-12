import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    lib: {
      entry: 'injection/main.ts',
      formats: ['iife'],
      name: '__GoogleChatTauriInjection',
      fileName: () => 'injection.js',
    },
    outDir: 'src-tauri',
    emptyOutDir: false,
    minify: 'oxc',
    sourcemap: false,
    target: 'es2022',
    rollupOptions: {
      output: {
        extend: false,
      },
    },
  },
});
