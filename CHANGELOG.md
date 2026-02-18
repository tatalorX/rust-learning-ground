# Changelog

All notable changes to the Rust Learning Ground project.

## [3.0.0] - 2026-02-18 - Sovereign Deployment Hardening

### 🔒 Security
- Implemented nsjail-based code execution sandbox
- Added file-based secrets management with sops/age
- Created compartmentalized Docker networks
- Gated subprocess fallback (fail-closed in production)
- Enhanced security configuration
- Changed license from MIT to AGPL-3.0
- Password reset tokens hashed with SHA-256
- SQL injection prevention measures
- XSS prevention with escapeHtml()
- Open redirect protection
- CSRF protection added to frontend API client

### 🏗️ Infrastructure
- Added hardened Dockerfile (multi-stage, minimal attack surface)
- Created production compose configuration (Podman-compatible)
- Added Caddyfile with TLS and security headers
- Created systemd service unit with sandboxing
- Added NixOS configuration reference
- Implemented .woodpecker.yml CI/CD pipeline
- Added .sops.yaml for age-encrypted secrets

### 📁 Project Structure
- Consolidated audit reports into `docs/AUDITS.md`
- Archived legacy code to `docs/archived/`
- Consolidated environment files into comprehensive `.env.example`
- Organized assets into `docs/assets/`
- Cleaned temporary files

### 📚 Documentation
- Created comprehensive security documentation
- Added verification and rotation scripts
- Updated deployment guides

## [2.0.0] - 2026-02-09 - Production Readiness

### 🔒 Security
- GDPR compliance implemented
- N+1 query issues resolved with batch loading
- Dependency scanning configured
- CSP headers hardened
- XSS prevention measures
- SECURITY.md created

### ✨ Features
- School platform with classrooms and assignments
- Grade levels (12), ranks (8 tiers), titles (20+)
- Leaderboards and progress tracking
- Real-time notifications via WebSocket

## [1.0.0] - 2026-02-09 - Initial Release

### 📊 Initial Release
- Security audit completed
- Core platform features implemented
- Authentication and authorization system
- Sandboxed code execution
- 320+ Rust exercises

---

**Note**: This changelog documents major releases and security improvements.
