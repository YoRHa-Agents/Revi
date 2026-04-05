import { test, expect } from '@playwright/test'

const BASE_API = 'http://localhost:8000'

test.describe('Workspace setup', () => {
  test.describe('configured workspace', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/#/')
      await page.waitForSelector('.item-card', { timeout: 10000 })
    })

    test('shows workspace indicator bar with configured path', async ({ page }) => {
      const indicator = page.locator('.workspace-indicator')
      await expect(indicator).toBeVisible()
      await expect(indicator.locator('.ws-path')).not.toBeEmpty()
    })

    test('shows Change button in workspace indicator', async ({ page }) => {
      const changeBtn = page.locator('.ws-change')
      await expect(changeBtn).toBeVisible()
    })

    test('clicking Change shows workspace setup form', async ({ page }) => {
      await page.locator('.ws-change').click()
      await expect(page.locator('.workspace-setup')).toBeVisible()
      await expect(page.locator('.setup-card')).toBeVisible()
    })

    test('setup form has local path input', async ({ page }) => {
      await page.locator('.ws-change').click()
      await expect(page.locator('.setup-input').first()).toBeVisible()
    })

    test('setup form has Open button for local path', async ({ page }) => {
      await page.locator('.ws-change').click()
      const openBtn = page.locator('.setup-btn.primary')
      await expect(openBtn).toBeVisible()
    })

    test('setup form has remote server input and Connect button', async ({ page }) => {
      await page.locator('.ws-change').click()
      const inputs = page.locator('.setup-input')
      await expect(inputs).toHaveCount(2)
      const connectBtn = page.locator('.setup-btn').last()
      await expect(connectBtn).toBeVisible()
    })

    test('Open button is disabled when path input is empty', async ({ page }) => {
      await page.locator('.ws-change').click()
      const openBtn = page.locator('.setup-btn.primary')
      await expect(openBtn).toBeDisabled()
    })
  })

  test.describe('workspace config API', () => {
    test('GET /api/config returns workspace configuration', async ({ request: apiRequest }) => {
      const resp = await apiRequest.get(`${BASE_API}/api/config`)
      expect(resp.ok()).toBeTruthy()
      const cfg = await resp.json()
      expect(cfg).toHaveProperty('workspacePath')
    })
  })
})
