# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.0.x   | :white_check_mark: |
| < 3.0   | :x:                |

## Reporting a Vulnerability

We take the security of The Book of Rust learning platform seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Reporting Process

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@rustlearning.local**

For especially sensitive issues, you may encrypt your report using our PGP key (available upon request).

### What to Include

When reporting a vulnerability, please include:

- **Description**: A clear description of the vulnerability
- **Impact**: What could an attacker achieve with this vulnerability?
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Affected Components**: Which parts of the application are affected?
- **Proof of Concept**: If possible, include a minimal proof of concept
- **Suggested Fix**: If you have one, suggest how to fix the issue

### Response Timeline

We will respond to security reports within the following timeframes:

| Severity | Acknowledgment | Fix Timeline |
|----------|---------------|--------------|
| Critical | 24 hours | 7 days |
| High | 48 hours | 14 days |
| Medium | 72 hours | 30 days |
| Low | 1 week | 90 days |

### Security Measures

The following security measures are implemented:

#### Authentication & Authorization
- Argon2id password hashing (OWASP recommended)
- HS256 JWT with explicit algorithm validation
- Rate limiting on all auth endpoints
- CSRF protection via double-submit cookies
- Row-Level Security for classroom access

#### Code Execution
- Sandboxed compilation using Firejail or Docker
- Seccomp BPF filtering for system calls
- Network isolation (`--net=none`)
- Resource limits (CPU, memory, file descriptors, processes)
- Dangerous pattern detection in submitted code

#### Data Protection
- SQL injection prevention via SQLAlchemy ORM
- XSS prevention via escapeHtml function
- Path traversal protection
- Input validation on all endpoints

#### Infrastructure
- Non-root container execution
- Security headers (CSP, HSTS, X-Frame-Options, etc.)
- CORS restrictions
- HTTPS enforcement in production

### Bug Bounty

We do not currently have a bug bounty program, but we publicly acknowledge security researchers who responsibly disclose vulnerabilities (with their permission).

### Security Checklist

Before each release, the following security checks are performed:

- [ ] All dependencies scanned with `pip-audit`
- [ ] No hardcoded secrets in codebase
- [ ] All input validated and sanitized
- [ ] Authentication checks on protected endpoints
- [ ] Rate limiting configured
- [ ] Security headers in place
- [ ] CORS properly restricted
- [ ] SQL injection tests pass
- [ ] XSS tests pass

### Contact

- **Security Team**: security@rustlearning.local
- **Response Time**: 24-48 hours for critical issues
- **GPG Key**: Available upon request

---

Thank you for helping keep The Book of Rust learning platform secure!
