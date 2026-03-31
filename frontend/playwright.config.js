import { defineConfig } from '@playwright/test'

export default defineConfig({
  testDir: './src/tests/e2e',
  timeout: 30000,
  retries: 1,
  use: {
    baseURL: 'http://localhost:5175',
    headless: true,
    screenshot: 'only-on-failure',
    video: 'off',
  },
  webServer: [
    {
      command: 'python3 -m uvicorn backend.main:app --port 8000',
      cwd: '/Users/jingyu/Workspace/Projects/Revi',
      port: 8000,
      timeout: 30000,
      reuseExistingServer: true,
    },
    {
      command: 'npm run dev -- --port 5175',
      port: 5175,
      timeout: 30000,
      reuseExistingServer: false,
    },
  ],
})
