import { test, expect } from '@playwright/test'

test.describe('Upload flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/#/')
    await page.waitForSelector('.item-card', { timeout: 10000 })
  })

  test('shows Open File button on dashboard', async ({ page }) => {
    const openFileBtn = page.locator('.upload-btn').filter({ hasText: /Open File|打开文件/ })
    await expect(openFileBtn).toBeVisible()
  })

  test('shows Open Folder button on dashboard', async ({ page }) => {
    const openFolderBtn = page.locator('.upload-btn').filter({ hasText: /Open Folder|打开文件夹/ })
    await expect(openFolderBtn).toBeVisible()
  })

  test('upload modal appears when files are selected', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan\n\nThis is a test plan.'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
  })

  test('upload modal shows file name and type selector', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan\n\nContent here.'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
    await expect(page.locator('.file-name')).toContainText('test-plan.md')
    await expect(page.locator('.type-select')).toBeVisible()
  })

  test('upload modal has cancel and confirm buttons', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
    await expect(page.locator('.cancel-btn')).toBeVisible()
    await expect(page.locator('.confirm-btn')).toBeVisible()
  })

  test('cancel button closes the upload modal', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
    await page.locator('.cancel-btn').click()
    await expect(page.locator('.upload-modal')).not.toBeVisible()
  })

  test('remove button removes file from pending list', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
    await page.locator('.remove-btn').click()
    await expect(page.locator('.upload-modal')).not.toBeVisible()
  })

  test('type select has auto/plan/design/prototype options', async ({ page }) => {
    const fileInput = page.locator('input[type="file"]').first()
    await fileInput.setInputFiles({
      name: 'test-plan.md',
      mimeType: 'text/markdown',
      buffer: Buffer.from('# Test Plan'),
    })
    await expect(page.locator('.upload-modal')).toBeVisible({ timeout: 5000 })
    const options = page.locator('.type-select option')
    await expect(options).toHaveCount(4)
  })
})
