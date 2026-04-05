import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/tests/setup.js'],
    exclude: ['**/node_modules/**', '**/e2e/**', '**/*.spec.js'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'text-summary'],
      include: ['src/**/*.{js,vue}'],
      exclude: ['src/tests/**', 'src/mock/**'],
    },
  },
})
