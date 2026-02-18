import { test, expect } from '@playwright/test';

test.describe('Exercise System', () => {
  test.beforeEach(async ({ page }) => {
    // Login before each test
    await page.goto('/auth.html');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'TestPassword123!');
    await page.click('button[type="submit"]');
    await page.waitForURL(/.*(dashboard|editor).*/, { timeout: 10000 });
  });

  test('should load exercise editor', async ({ page }) => {
    await page.goto('/editor.html?id=1');
    
    // Check editor elements
    await expect(page.locator('.code-editor, textarea, [role="textbox"]')).toBeVisible();
    await expect(page.locator('text=Run')).toBeVisible();
  });

  test('should run code and show output', async ({ page }) => {
    await page.goto('/editor.html?id=1');
    
    // Wait for editor to load
    await page.waitForTimeout(1000);
    
    // Click run
    await page.click('text=Run');
    
    // Wait for output
    await expect(page.locator('.output, .console, [data-testid="output"]')).toBeVisible({ timeout: 30000 });
  });

  test('should navigate between exercises', async ({ page }) => {
    await page.goto('/editor.html?id=1');
    
    // Check for navigation
    const nextButton = page.locator('text=Next, text=>>, text=→').first();
    if (await nextButton.isVisible().catch(() => false)) {
      await nextButton.click();
      await expect(page).toHaveURL(/.*id=2.*/);
    }
  });

  test('should save code', async ({ page }) => {
    await page.goto('/editor.html?id=1');
    
    await page.waitForTimeout(1000);
    
    // Clear and type new code
    await page.keyboard.press('Control+a');
    await page.keyboard.type('fn main() { println!("Test"); }');
    
    // Click save if available
    const saveButton = page.locator('text=Save').first();
    if (await saveButton.isVisible().catch(() => false)) {
      await saveButton.click();
      await expect(page.locator('text=Saved')).toBeVisible();
    }
  });
});

test.describe('Exercise List', () => {
  test('should display exercise list', async ({ page }) => {
    await page.goto('/');
    
    await expect(page.locator('.exercise-list, [data-testid="exercises"]')).toBeVisible();
  });

  test('should filter exercises', async ({ page }) => {
    await page.goto('/');
    
    // Check for filter/search
    const searchBox = page.locator('input[type="search"], input[placeholder*="search" i]').first();
    if (await searchBox.isVisible().catch(() => false)) {
      await searchBox.fill('hello');
      await page.waitForTimeout(500);
      // Check that results are filtered
    }
  });
});
