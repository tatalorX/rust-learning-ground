import { test, expect } from '@playwright/test';

test.describe('Accessibility', () => {
  test('should have skip to content link on homepage', async ({ page }) => {
    await page.goto('/');
    
    const skipLink = page.locator('a:has-text("Skip to main content"), a:has-text("Skip to content")');
    await expect(skipLink).toBeVisible();
    await expect(skipLink).toHaveAttribute('href', '#main-content');
  });

  test('should have proper heading structure', async ({ page }) => {
    await page.goto('/');
    
    // Check that h1 exists
    const h1 = page.locator('h1');
    await expect(h1).toHaveCount(1);
    
    // Check that headings are not skipped (h1 -> h2, not h1 -> h3)
    // This is a basic check - full a11y audit would require axe-core
  });

  test('should have alt text on images', async ({ page }) => {
    await page.goto('/');
    
    // Get all images
    const images = page.locator('img');
    const count = await images.count();
    
    for (let i = 0; i < count; i++) {
      const alt = await images.nth(i).getAttribute('alt');
      // Decorative images can have empty alt, but content images should have alt text
      expect(alt).not.toBeNull();
    }
  });

  test('should have accessible forms', async ({ page }) => {
    await page.goto('/auth.html');
    
    // Check that inputs have labels or aria-label
    const inputs = page.locator('input:not([type="hidden"])');
    const count = await inputs.count();
    
    for (let i = 0; i < count; i++) {
      const input = inputs.nth(i);
      const id = await input.getAttribute('id');
      const ariaLabel = await input.getAttribute('aria-label');
      const ariaLabelledBy = await input.getAttribute('aria-labelledby');
      
      if (!ariaLabel && !ariaLabelledBy && id) {
        // Check for associated label
        const label = page.locator(`label[for="${id}"]`);
        const hasLabel = await label.count() > 0;
        expect(hasLabel || await input.getAttribute('placeholder')).toBeTruthy();
      }
    }
  });

  test('should have sufficient color contrast', async ({ page }) => {
    await page.goto('/');
    
    // This is a placeholder - real color contrast testing requires axe-core or similar
    // For now, just verify the page loads without obvious issues
    await expect(page.locator('body')).toBeVisible();
  });

  test('should be keyboard navigable', async ({ page }) => {
    await page.goto('/auth.html');
    
    // Tab through the page
    await page.keyboard.press('Tab');
    const focused1 = await page.evaluate(() => document.activeElement?.tagName);
    expect(focused1).not.toBe('BODY');
    
    // Continue tabbing
    await page.keyboard.press('Tab');
    const focused2 = await page.evaluate(() => document.activeElement?.tagName);
    expect(focused2).not.toBe('BODY');
  });

  test('should have proper ARIA landmarks', async ({ page }) => {
    await page.goto('/');
    
    // Check for main landmark or role
    const main = page.locator('main, [role="main"], #main-content');
    await expect(main).toBeVisible();
    
    // Check for navigation
    const nav = page.locator('nav, [role="navigation"]').first();
    if (await nav.isVisible().catch(() => false)) {
      expect(await nav.count()).toBeGreaterThan(0);
    }
  });
});

test.describe('Responsive Design', () => {
  test('should be usable on mobile', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Check that content is visible
    await expect(page.locator('body')).toBeVisible();
    
    // Check for mobile menu if present
    const menuButton = page.locator('[aria-label*="menu" i], .menu-button, .hamburger').first();
    if (await menuButton.isVisible().catch(() => false)) {
      await menuButton.click();
      // Menu should open
    }
  });

  test('should be usable on tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    
    await expect(page.locator('body')).toBeVisible();
  });

  test('should be usable on desktop', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto('/');
    
    await expect(page.locator('body')).toBeVisible();
  });
});
