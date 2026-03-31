import { test, expect } from '@playwright/test'

const ITEM_ID = 'designs/ui-mockup-v1'

test.describe('Design review', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(`/#/review/${ITEM_ID}`)
    await page.waitForSelector('.split-layout', { timeout: 10000 })
  })

  test('shows design content pane', async ({ page }) => {
    await expect(page.locator('.content-pane')).toBeVisible()
    await expect(page.locator('.comment-pane')).toBeVisible()
  })

  test('shows comment panel with tabs', async ({ page }) => {
    // The comment pane should be visible with open/resolved tabs
    const commentPane = page.locator('.comment-pane')
    await expect(commentPane).toBeVisible()
    // CommentPanel always renders two tabs
    const tabs = page.locator('.panel .tab')
    await expect(tabs).toHaveCount(2)
  })

  test('does not show search or index buttons for design type', async ({ page }) => {
    // Search and index action buttons are only shown for plan type (v-if="item.type === 'plan'")
    const actionBtns = page.locator('.header-actions .action-btn')
    await expect(actionBtns).toHaveCount(0)
  })

  test('can open add comment form', async ({ page }) => {
    // btn-primary toggles the comment form
    const addBtn = page.locator('.btn-primary').first()
    await addBtn.click()
    await expect(page.locator('.add-form')).toBeVisible({ timeout: 5000 })
    // Close the form
    await addBtn.click()
    await expect(page.locator('.add-form')).not.toBeVisible()
  })
})
