# Security Documentation

## Overview

This project follows security best practices including:

- **Sandboxed Code Execution**: nsjail-based isolation for running user code
- **Zero-Trust Architecture**: Defense in depth with compartmentalized networks
- **Secret Management**: File-based secrets with sops/age encryption
- **Authentication**: Argon2 password hashing, JWT with HS256
- **Authorization**: Row-level security, role-based access control
- **Input Validation**: SQL injection prevention, XSS protection
- **Infrastructure**: Non-root containers, read-only filesystems, AppArmor

## Security Features

### Code Execution Sandbox
- nsjail with namespace isolation and seccomp
- Resource limits (CPU, memory, time)
- Network isolation
- Disposable execution environment

### Authentication & Authorization
- Argon2id password hashing
- JWT tokens with short expiry
- Rate limiting on all auth endpoints
- CSRF protection
- Account lockout after failed attempts

### Infrastructure Security
- Multi-stage hardened Docker containers
- Podman-compatible deployment
- Systemd sandboxing
- TLS with Caddy reverse proxy
- Security headers (CSP, HSTS, etc.)

## Reporting Security Issues

Please report security vulnerabilities to the project maintainers privately. See [SECURITY.md](../SECURITY.md) for details.

## Compliance

- **GDPR**: Data export, right to erasure, privacy controls
- **OWASP Top 10**: Addressed through defense in depth
- **Zero-Trust**: Every request verified, least privilege principle

---

**Last Updated:** February 2026
