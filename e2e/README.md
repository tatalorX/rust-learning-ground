# End-to-End Tests

This directory contains end-to-end tests for The Book of Rust using Playwright.

## Prerequisites

```bash
# Install dependencies
cd e2e
npm install

# Install Playwright browsers
npx playwright install
```

## Running Tests

```bash
# Run all tests
npm test

# Run with UI mode (for development)
npm run test:ui

# Run in debug mode
npm run test:debug

# Run specific test file
npx playwright test auth.spec.ts

# Run in specific browser
npx playwright test --project=chromium

# Run tests against specific URL
BASE_URL=http://localhost:3000 npx playwright test
```

## Test Structure

```
e2e/
├── tests/
│   ├── auth.spec.ts          # Authentication tests
│   ├── exercises.spec.ts      # Exercise system tests
│   ├── school-platform.spec.ts # School platform tests
│   └── accessibility.spec.ts  # Accessibility tests
├── playwright.config.ts       # Playwright configuration
└── package.json
```

## Writing Tests

Example test:

```typescript
import { test, expect } from '@playwright/test';

test('should login', async ({ page }) => {
  await page.goto('/auth.html');
  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password');
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/.*dashboard.*/);
});
```

## Environment Variables

- `BASE_URL` - Base URL for tests (default: http://localhost:54321)
- `SKIP_WEBSERVER` - Don't start local servers (for testing against deployed instance)
- `CI` - Running in CI mode (enables retries, disables web server reuse)

## Best Practices

1. **Use data-testid attributes** for selecting elements when possible
2. **Avoid hardcoded waits** - use `waitFor` or assertions instead
3. **Test user workflows** not just individual features
4. **Clean up state** after tests when necessary
5. **Use page objects** for repeated operations (see below)

## Page Objects

Create reusable page objects for common operations:

```typescript
// pages/AuthPage.ts
export class AuthPage {
  constructor(private page: Page) {}
  
  async login(email: string, password: string) {
    await this.page.goto('/auth.html');
    await this.page.fill('input[type="email"]', email);
    await this.page.fill('input[type="password"]', password);
    await this.page.click('button[type="submit"]');
  }
}
```

## Accessibility Testing

Tests include basic accessibility checks:
- Skip to content links
- Heading structure
- Alt text on images
- Form labels
- ARIA landmarks
- Keyboard navigation
- Responsive design

For comprehensive accessibility testing, consider integrating [axe-core](https://github.com/dequelabs/axe-core).

## CI/CD Integration

Tests run automatically in CI on:
- Pull requests
- Pushes to main branch
- Daily scheduled runs

## Troubleshooting

### Tests fail due to timing issues

Add explicit waits or use retry logic:
```typescript
await expect(locator).toBeVisible({ timeout: 10000 });
```

### Tests fail in CI but pass locally

- Check that the web server started correctly
- Verify environment variables are set
- Check screenshots in the HTML report

### Generate new tests

Use codegen to record interactions:
```bash
npx playwright codegen http://localhost:54321
```
