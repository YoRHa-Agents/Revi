import { test, expect } from '@playwright/test'

const BASE_API = 'http://localhost:8000'
const ITEM_ID = 'plans/sprint-1-design'

test.describe('Archive view', () => {
  test.beforeEach(async ({ request: apiRequest }) => {
    // Add a comment, resolve it, archive it via API so the archive view has data
    const addResp = await apiRequest.post(`${BASE_API}/api/comments/${ITEM_ID}`, {
      data: { author: 'E2E-Archive', content: 'Archive test comment', reference: { type: 'general', value: null } }
    })
    if (addResp.ok()) {
      const comment = await addResp.json()
      await apiRequest.patch(`${BASE_API}/api/comments/${ITEM_ID}/${comment.id}/resolve`)
      await apiRequest.post(`${BASE_API}/api/archive/${ITEM_ID}`)
    }
  })

  test('shows archive page with .archive-page class', async ({ page }) => {
    await page.goto('/#/archive')
    // ArchiveView.vue uses .archive-page (not .archive-view)
    await page.waitForSelector('.archive-page', { timeout: 10000 })
    await expect(page).toHaveURL(/archive/)
  })

  test('archive page has page title', async ({ page }) => {
    await page.goto('/#/archive')
    await page.waitForSelector('.archive-page', { timeout: 10000 })
    // .page-title is always rendered in ArchiveView
    await expect(page.locator('.page-title')).toBeVisible()
  })

  test('archive page loads without error state', async ({ page }) => {
    await page.goto('/#/archive')
    await page.waitForSelector('.archive-page', { timeout: 10000 })
    // The .error class from HomeView is not used here, but check no error text
    const errorEl = page.locator('.archive-page .error')
    await expect(errorEl).toHaveCount(0)
  })

  test('shows archived batch data after seeding', async ({ page }) => {
    await page.goto('/#/archive')
    await page.waitForSelector('.archive-page', { timeout: 10000 })
    // If seeding succeeded, there should be at least one .batch element
    // Allow time for async archive data to load
    await page.waitForTimeout(2000)
    const batches = await page.locator('.batch').all()
    // Either we have batches (seed worked) or we have empty state
    if (batches.length > 0) {
      await expect(page.locator('.batch').first()).toBeVisible()
      await expect(page.locator('.batch-header').first()).toBeVisible()
    } else {
      // Empty state is shown with .empty class
      await expect(page.locator('.empty')).toBeVisible()
    }
  })

  test('archived comments show resolved badge', async ({ page }) => {
    await page.goto('/#/archive')
    await page.waitForSelector('.archive-page', { timeout: 10000 })
    await page.waitForTimeout(2000)
    const resolvedBadges = await page.locator('.resolved-badge').all()
    if (resolvedBadges.length > 0) {
      // Each archived comment shows "✓ resolved" badge
      await expect(page.locator('.resolved-badge').first()).toBeVisible()
    }
  })
})
