import { test, expect } from '@playwright/test'

const VIEWPORTS = [
  { name: 'mobile', width: 375, height: 667 },
  { name: 'tablet', width: 768, height: 1024 },
  { name: 'desktop', width: 1440, height: 900 },
]

test.describe('Responsive viewports', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`${vp.name} (${vp.width}x${vp.height})`, () => {
      test.beforeEach(async ({ page }) => {
        await page.setViewportSize({ width: vp.width, height: vp.height })
        await page.goto('/#/')
        await page.waitForSelector('.item-card', { timeout: 10000 })
      })

      test('home page loads without errors', async ({ page }) => {
        await expect(page.locator('.home')).toBeVisible()
        const errorEl = page.locator('.error')
        await expect(errorEl).toHaveCount(0)
      })

      test('stat cards are visible', async ({ page }) => {
        const statCards = await page.locator('.stat-card').all()
        expect(statCards.length).toBe(4)
      })

      test('item cards are visible', async ({ page }) => {
        const cards = await page.locator('.item-card').all()
        expect(cards.length).toBeGreaterThanOrEqual(1)
      })

      test('no horizontal overflow on body', async ({ page }) => {
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth)
        expect(bodyWidth).toBeLessThanOrEqual(vp.width)
      })

      test('review page loads at this viewport', async ({ page }) => {
        await page.goto('/#/review/plans/sprint-1-design')
        await page.waitForSelector('.content-pane', { timeout: 10000 })
        await expect(page.locator('.split-layout')).toBeVisible()
      })
    })
  }
})
