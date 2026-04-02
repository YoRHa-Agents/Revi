import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig } from '@playwright/test'

const frontendRoot = path.dirname(fileURLToPath(import.meta.url))
const repoRoot = path.resolve(frontendRoot, '..')

export default defineConfig({
  testDir: './src/tests/e2e',
  timeout: 30000,
  retries: 1,
  workers: 1,
  use: {
    baseURL: 'http://localhost:5175',
    headless: true,
    screenshot: 'only-on-failure',
    video: 'off',
  },
  webServer: [
    {
      command: 'bash -lc "rm -rf .tmp-e2e && mkdir -p .tmp-e2e && cp -R testdata/e2e/workspace .tmp-e2e/workspace && cp -R testdata/e2e/data .tmp-e2e/data && cargo run --manifest-path server/Cargo.toml --bin revi -- --workspace .tmp-e2e/workspace --data .tmp-e2e/data --port 8000"',
      cwd: repoRoot,
      port: 8000,
      timeout: 30000,
      reuseExistingServer: false,
    },
    {
      command: 'npm run dev -- --port 5175',
      port: 5175,
      timeout: 30000,
      reuseExistingServer: false,
    },
  ],
})
