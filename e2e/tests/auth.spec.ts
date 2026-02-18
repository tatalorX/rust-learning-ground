import { test, expect } from '@playwright/test';

test.describe('Authentication', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/auth.html');
  });

  test('should display login form', async ({ page }) => {
    await expect(page.locator('text=Sign In')).toBeVisible();
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should toggle between login and register', async ({ page }) => {
    await page.click('text=Create Account');
    await expect(page.locator('text=Create Account')).toBeVisible();
    await expect(page.locator('input[name="username"]')).toBeVisible();
    
    await page.click('text=Sign In');
    await expect(page.locator('text=Sign In')).toBeVisible();
  });

  test('should show validation errors for empty fields', async ({ page }) => {
    await page.click('button[type="submit"]');
    await expect(page.locator('text=Email is required')).toBeVisible();
  });

  test('should login with valid credentials', async ({ page }) => {
    // This test requires a test user to exist
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'TestPassword123!');
    await page.click('button[type="submit"]');
    
    // Should redirect to dashboard or editor
    await expect(page).toHaveURL(/.*(dashboard|editor).*/);
  });

  test('should show error for invalid credentials', async ({ page }) => {
    await page.fill('input[type="email"]', 'invalid@example.com');
    await page.fill('input[type="password"]', 'wrongpassword');
    await page.click('button[type="submit"]');
    
    await expect(page.locator('text=Invalid email or password')).toBeVisible();
  });

  test('should be accessible', async ({ page }) => {
    // Check for skip to content link
    await expect(page.locator('text=Skip to main content')).toBeVisible();
    
    // Check form labels
    await expect(page.locator('label[for="email"]')).toBeVisible();
    await expect(page.locator('label[for="password"]')).toBeVisible();
  });
});

test.describe('Registration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/auth.html');
    await page.click('text=Create Account');
  });

  test('should register new user', async ({ page }) => {
    const timestamp = Date.now();
    const email = `test${timestamp}@example.com`;
    
    await page.fill('input[name="username"]', `testuser${timestamp}`);
    await page.fill('input[type="email"]', email);
    await page.fill('input[type="password"]', 'TestPassword123!');
    await page.fill('input[name="display_name"]', 'Test User');
    await page.click('button[type="submit"]');
    
    // Should redirect after successful registration
    await expect(page).toHaveURL(/.*(dashboard|editor).*/, { timeout: 10000 });
  });

  test('should validate password strength', async ({ page }) => {
    await page.fill('input[type="password"]', 'weak');
    await page.click('button[type="submit"]');
    
    await expect(page.locator('text=Password must be at least 8 characters')).toBeVisible();
  });
});
