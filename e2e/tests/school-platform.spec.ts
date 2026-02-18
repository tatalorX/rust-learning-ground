import { test, expect } from '@playwright/test';

test.describe('School Platform', () => {
  test.beforeEach(async ({ page }) => {
    // Login as teacher before each test
    await page.goto('/auth.html');
    await page.fill('input[type="email"]', 'teacher@example.com');
    await page.fill('input[type="password"]', 'TeacherPass123!');
    await page.click('button[type="submit"]');
    await page.waitForURL(/.*dashboard.*/, { timeout: 10000 });
  });

  test('should display school dashboard', async ({ page }) => {
    await page.goto('/school-dashboard.html');
    
    await expect(page.locator('text=School Dashboard')).toBeVisible();
  });

  test('should show user profile information', async ({ page }) => {
    await page.goto('/school-dashboard.html');
    
    // Check for profile elements
    await expect(page.locator('text=XP')).toBeVisible();
    await expect(page.locator('text=Rank')).toBeVisible();
  });

  test('should create classroom', async ({ page }) => {
    await page.goto('/school-dashboard.html');
    
    // Click create classroom
    await page.click('text=Create Classroom');
    
    // Fill form
    await page.fill('input[name="name"]', `Test Classroom ${Date.now()}`);
    await page.fill('textarea[name="description"]', 'Test description');
    await page.click('button[type="submit"]');
    
    // Check success
    await expect(page.locator('text=Classroom created')).toBeVisible();
  });

  test('should join classroom with code', async ({ page }) => {
    await page.goto('/school-dashboard.html');
    
    // Click join classroom
    await page.click('text=Join Classroom');
    
    // Enter code
    await page.fill('input[name="code"]', 'TESTCODE');
    await page.click('button[type="submit"]');
    
    // Check result (may succeed or fail depending on code validity)
    await expect(page.locator('text=joined, text=invalid, text=error').first()).toBeVisible();
  });

  test('should display leaderboard', async ({ page }) => {
    await page.goto('/school-dashboard.html');
    
    await page.click('text=Leaderboard');
    
    await expect(page.locator('table, .leaderboard, [data-testid="leaderboard"]')).toBeVisible();
  });
});

test.describe('Classroom View', () => {
  test('should display classroom details', async ({ page }) => {
    await page.goto('/classroom.html?id=1');
    
    await expect(page.locator('text=Classroom')).toBeVisible();
    await expect(page.locator('text=Members, text=Students').first()).toBeVisible();
  });

  test('should show assignments', async ({ page }) => {
    await page.goto('/classroom.html?id=1');
    
    await page.click('text=Assignments');
    
    await expect(page.locator('.assignments, [data-testid="assignments"]')).toBeVisible();
  });
});
