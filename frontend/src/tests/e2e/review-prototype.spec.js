import { test, expect } from '@playwright/test'

const ITEM_ID = 'prototypes/review-flow'

test.describe('Prototype review', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(`/#/review/${ITEM_ID}`)
    await page.waitForSelector('.split-layout', { timeout: 10000 })
  })

  test('shows prototype content pane', async ({ page }) => {
    await expect(page.locator('.content-pane')).toBeVisible()
  })

  test('shows comment panel', async ({ page }) => {
    await expect(page.locator('.comment-pane')).toBeVisible()
  })

  test('shows comment tabs', async ({ page }) => {
    // CommentPanel always renders two tabs regardless of item type
    const tabs = page.locator('.panel .tab')
    await expect(tabs).toHaveCount(2)
  })

  test('does not show search or index buttons for prototype type', async ({ page }) => {
    // action-btn buttons are only rendered when item.type === 'plan'
    const actionBtns = page.locator('.header-actions .action-btn')
    await expect(actionBtns).toHaveCount(0)
  })

  test('can open add comment form', async ({ page }) => {
    const addBtn = page.locator('.btn-primary').first()
    await addBtn.click()
    await expect(page.locator('.add-form')).toBeVisible({ timeout: 5000 })
  })
})
