import { defineConfig } from 'vite'
import { resolve } from 'path'
import dts from 'vite-plugin-dts'

export default defineConfig({
  root: __dirname,
  build: {
    lib: {
      // 使用 resolve 确保路径正确
      entry: resolve(__dirname, './src/index.ts'),
      name: 'SuiTxParse',
      fileName: 'index',
      formats: ['es', 'umd']
    },
    rollupOptions: {
      external: ['@mysten/sui'],
      output: {
        globals: {
          '@mysten/sui': 'Sui'
        }
      }
    }
  },
  plugins: [
    dts({
      insertTypesEntry: true,
      rollupTypes: false,
      tsconfigPath: resolve(__dirname, './tsconfig.json')
    })
  ],
  test: {
    globals: true,
    environment: "node",
    include: ["tests/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
    },
  },
});
