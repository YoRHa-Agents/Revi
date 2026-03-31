import { test, expect } from '@playwright/test'

test.describe('Home page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/#/')
    // Wait for items to load from API
    await page.waitForSelector('.item-card', { timeout: 10000 })
  })

  test('shows review items in grid', async ({ page }) => {
    const cards = await page.locator('.item-card').all()
    expect(cards.length).toBeGreaterThanOrEqual(1)
  })

  test('shows type badges', async ({ page }) => {
    // At least one type badge should be visible (plan/design/prototype)
    const badges = await page.locator('.type-badge').all()
    expect(badges.length).toBeGreaterThan(0)
  })

  test('has open button that navigates to review', async ({ page }) => {
    // .open-btn is a router-link rendered as an <a> tag
    await page.locator('.open-btn').first().click()
    await expect(page).toHaveURL(/\/review\//)
  })

  test('shows stats row with counts', async ({ page }) => {
    // Stats row with 4 stat cards should always be present
    const statCards = await page.locator('.stat-card').all()
    expect(statCards.length).toBe(4)
  })

  test('language toggle switches display language', async ({ page }) => {
    // App.vue uses .lang-btn with text "EN" and "中文"
    const zhBtn = page.locator('button.lang-btn').filter({ hasText: '中文' })
    await expect(zhBtn).toBeVisible()
    await zhBtn.click()
    // After switching to Chinese, wait briefly for reactivity
    await page.waitForTimeout(500)

    // Switch back to EN
    const enBtn = page.locator('button.lang-btn').filter({ hasText: 'EN' })
    await expect(enBtn).toBeVisible()
    await enBtn.click()
    await page.waitForTimeout(300)

    // Verify EN is now active
    await expect(enBtn).toHaveClass(/active/)
  })
})
