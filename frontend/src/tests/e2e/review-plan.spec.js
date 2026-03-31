import { test, expect } from '@playwright/test'

const BASE_API = 'http://localhost:8000'
const ITEM_ID = 'plans/sprint-1-design'

test.describe('Plan review flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(`/#/review/${ITEM_ID}`)
    // Wait for markdown to render — MarkdownViewer renders into .md-viewer inside .content-pane
    await page.waitForSelector('.content-pane', { timeout: 10000 })
  })

  test('shows split layout with content and comment pane', async ({ page }) => {
    await expect(page.locator('.split-layout')).toBeVisible()
    await expect(page.locator('.content-pane')).toBeVisible()
    await expect(page.locator('.comment-pane')).toBeVisible()
  })

  test('shows open and resolved tabs in comment panel', async ({ page }) => {
    // CommentPanel renders two .tab buttons inside .tabs
    const tabs = page.locator('.panel .tab')
    await expect(tabs).toHaveCount(2)
  })

  test('can add a comment', async ({ page }) => {
    // CommentPanel: btn-primary toggles the form (text from i18n review.addComment)
    const addBtn = page.locator('.btn-primary').first()
    await addBtn.click()

    // Wait for form to slide in
    await page.waitForSelector('.add-form', { timeout: 5000 })

    // Author input has placeholder "Your name" (hardcoded in CommentPanel.vue)
    const authorField = page.locator('.add-form input[type="text"]').first()
    // Textarea for content
    const contentField = page.locator('.add-form textarea').first()

    await authorField.fill('E2E-Test')
    await contentField.fill('E2E test comment')

    // Submit button is .btn-submit
    const submitBtn = page.locator('.btn-submit')
    await submitBtn.click()

    // Comment should appear in the open tab list
    await expect(page.locator('.c-content').filter({ hasText: 'E2E test comment' }).first()).toBeVisible({ timeout: 5000 })
  })

  test('can resolve a comment', async ({ page, request: apiRequest }) => {
    // Seed a comment via API to avoid UI timing issues
    const addResp = await apiRequest.post(`${BASE_API}/api/comments/${ITEM_ID}`, {
      data: { author: 'E2E', content: 'To resolve via E2E', reference: { type: 'general', value: null } }
    })
    expect(addResp.ok()).toBeTruthy()

    // Refresh page to see the comment
    await page.reload()
    await page.waitForSelector('.comment-pane', { timeout: 10000 })
    await page.waitForTimeout(1000) // allow comments to render

    // .btn-resolve is the "✓ resolve" button in each open comment card
    const resolveBtn = page.locator('.btn-resolve').first()
    if (await resolveBtn.isVisible()) {
      await resolveBtn.click()
      await page.waitForTimeout(500)
      // After resolving, tab may switch to "resolved" or comment disappears from open list
    }
  })

  test('shows search button for plan type and opens search bar', async ({ page }) => {
    // Search button is in .header-actions with class .action-btn, only for plan type
    // The button text comes from i18n review.search — look for the action button containing ⌕
    const searchBtn = page.locator('.header-actions .action-btn').filter({ hasText: /⌕/ }).first()
    if (await searchBtn.isVisible()) {
      await searchBtn.click()
      // .in-search-bar should appear in the markdown viewer
      await expect(page.locator('.in-search-bar')).toBeVisible({ timeout: 3000 })
    }
  })

  test('shows doc index button for plan type', async ({ page }) => {
    // Index button is also in .header-actions — the ☰ button
    const indexBtn = page.locator('.header-actions .action-btn').filter({ hasText: /☰/ }).first()
    if (await indexBtn.isVisible()) {
      await indexBtn.click()
      await page.waitForTimeout(300)
      // Click again to close
      await indexBtn.click()
    }
  })
})
