import { sveltekit } from '@sveltejs/kit/vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

import { defineConfig, searchForWorkspaceRoot } from 'vite';

export default defineConfig({
  plugins: [sveltekit(), wasm(), topLevelAwait()],
  // build: {
  //   rollupOptions: {
  //     external: ['wasm-game-of-life']
  //   }
  // },
  server: {
    fs: {
      allow: [
        // We have to search the workspace root because the Vite config
        // is in a subdirectory of the workspace root:
        searchForWorkspaceRoot(process.cwd()),
        // Our custom path to be allowed by the Vite server:
        '../rust/pkg'
      ]
    }
  }
  // optimizeDeps: {
  //   include: ['wasm-game-of-life']
  // }
  // build: {
  //   commonjsOptions: {
  //     include: [/wasm-game-of-life/, /node_modules/]
  //   }
  // }
});
