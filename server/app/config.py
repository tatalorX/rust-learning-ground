# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Application configuration with security-focused settings.

This module automatically generates and manages all cryptographic secrets
using a master key system for easy backup and rotation.

Sovereign deployment: Supports file-based secrets (e.g., /run/secrets/*)
for use with sops-encrypted secrets decrypted into tmpfs at boot.
"""

import os
import sys
from functools import lru_cache
from typing import Optional, List

from pydantic_settings import BaseSettings
from pydantic import field_validator


def _read_secret_file(env_var: str) -> Optional[str]:
    """
    Read a secret from a file path specified by an environment variable.
    Supports Docker/Podman secrets pattern: FOO_FILE=/run/secrets/foo
    """
    file_path = os.environ.get(f"{env_var}_FILE")
    if file_path and os.path.isfile(file_path):
        try:
            with open(file_path, "r") as f:
                return f.read().strip()
        except (IOError, OSError):
            pass
    return None


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""

    # Application
    APP_NAME: str = "Rust Learning Ground"
    DEBUG: bool = False
    ENVIRONMENT: str = "production"  # development, staging, production

    # Security - Auto-generated if not set
    # In production, you should set these explicitly or backup the master key
    # Supports file-based secrets: set JWT_SECRET_FILE=/run/secrets/jwt_secret
    SECRET_KEY: str = ""  # Auto-generated from master key
    JWT_SECRET_KEY: str = ""  # Auto-generated from master key

    # Master key configuration
    MASTER_KEY_PATH: Optional[str] = None  # Path to store master key

    # Database
    # Supports file-based secrets: set DATABASE_URL_FILE=/run/secrets/db_url
    DATABASE_URL: str = "sqlite+aiosqlite:///./data/app.db"

    @field_validator("DATABASE_URL")
    @classmethod
    def validate_database_url(cls, v: str) -> str:
        """Ensure SQLite uses aiosqlite driver for async support."""
        if v.startswith("sqlite://") and "aiosqlite" not in v:
            v = v.replace("sqlite://", "sqlite+aiosqlite://")
        return v

    # Connection Pooling (PostgreSQL only, ignored for SQLite)
    DB_POOL_SIZE: int = 20
    DB_MAX_OVERFLOW: int = 10
    DB_POOL_RECYCLE: int = 3600  # 1 hour
    DB_POOL_TIMEOUT: int = 30  # seconds

    # JWT Configuration
    JWT_ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 15  # Short-lived access tokens
    REFRESH_TOKEN_EXPIRE_DAYS: int = 7  # Longer-lived refresh tokens

    # Password Hashing (Argon2 settings - OWASP recommended)
    PASSWORD_MIN_LENGTH: int = 8
    PASSWORD_MAX_LENGTH: int = 128

    # GitHub OAuth (optional - for GitHub login)
    GITHUB_CLIENT_ID: Optional[str] = None
    GITHUB_CLIENT_SECRET: Optional[str] = None

    # Rate Limiting
    RATE_LIMIT_LOGIN: str = "5/minute"
    RATE_LIMIT_REGISTER: str = "3/minute"
    RATE_LIMIT_API: str = "100/minute"
    RATE_LIMIT_REFRESH: str = "10/minute"

    # CORS - Restrict in production!
    CORS_ORIGINS: str = (
        "http://localhost:3000,"
        "http://localhost:54321,"
        "http://localhost:8000,"
        "http://127.0.0.1:3000,"
        "http://127.0.0.1:54321,"
        "http://127.0.0.1:8000,"
        "https://rust-learning-ground-v4.pages.dev"
    )

    @property
    def cors_origins_list(self) -> List[str]:
        """Parse CORS origins from comma-separated string."""
        origins = []
        for origin in self.CORS_ORIGINS.split(","):
            origin = origin.strip()
            if origin:
                origins.append(origin)
        return origins

    @property
    def cors_origins_regex(self) -> str:
        """Generate regex pattern for wildcard origins."""
        patterns = []
        for origin in self.cors_origins_list:
            if "*" in origin:
                pattern = origin.replace(".", r"\.").replace("*", ".*")
                patterns.append(pattern)

        if patterns:
            return "|".join(patterns)
        return ""

    # Cookie Security
    COOKIE_SECURE: bool = True
    COOKIE_SAMESITE: str = "lax"
    COOKIE_DOMAIN: Optional[str] = None

    # Trusted Hosts
    ALLOWED_HOSTS: str = "localhost,127.0.0.1"

    @property
    def allowed_hosts_list(self) -> List[str]:
        """Parse allowed hosts."""
        return [host.strip() for host in self.ALLOWED_HOSTS.split(",") if host.strip()]

    # Session/CSRF
    CSRF_TOKEN_EXPIRE_MINUTES: int = 60

    # Proxy Configuration
    TRUSTED_PROXIES: int = 0

    # OAuth Redirect
    OAUTH_REDIRECT_URL: str = "/editor.html?auth=success"

    # Public URL
    API_BASE_URL: Optional[str] = None

    # ============================================================================
    # EMAIL SETTINGS
    # ============================================================================

    # Email backend: smtp, sendgrid, console, null
    EMAIL_BACKEND: str = "console"  # Default to console for development
    EMAIL_FROM: str = "noreply@rustlearning.com"
    EMAIL_FROM_NAME: str = "Rust Learning Ground"

    # SMTP Settings (if EMAIL_BACKEND=smtp)
    SMTP_HOST: Optional[str] = None
    SMTP_PORT: int = 587
    SMTP_USER: Optional[str] = None
    SMTP_PASSWORD: Optional[str] = None
    SMTP_USE_TLS: bool = True
    SMTP_USE_SSL: bool = False

    # SendGrid Settings (if EMAIL_BACKEND=sendgrid)
    SENDGRID_API_KEY: Optional[str] = None

    # ============================================================================
    # REDIS / CACHE SETTINGS
    # ============================================================================

    # Redis URL (for caching, sessions, rate limiting)
    REDIS_URL: Optional[str] = None  # e.g., "redis://localhost:6379/0"

    # Cache TTL settings (in seconds)
    CACHE_TTL_LEADERBOARD: int = 300  # 5 minutes
    CACHE_TTL_USER_PROFILE: int = 60  # 1 minute
    CACHE_TTL_CLASSROOM: int = 120  # 2 minutes
    CACHE_TTL_EXERCISE: int = 3600  # 1 hour

    # Enable/disable caching (with memory fallback when Redis unavailable)
    CACHE_ENABLED: bool = True  # Enabled by default with memory fallback

    # ============================================================================
    # WEBSOCKET SETTINGS
    # ============================================================================

    # WebSocket settings
    WS_HEARTBEAT_INTERVAL: int = 30  # seconds
    WS_MAX_CONNECTIONS: int = 10000

    # ============================================================================
    # MONITORING & LOGGING
    # ============================================================================

    # Log level
    LOG_LEVEL: str = "INFO"

    # Sentry DSN (for error tracking)
    SENTRY_DSN: Optional[str] = None

    # Prometheus metrics endpoint
    METRICS_ENABLED: bool = True

    # ============================================================================
    # FILE UPLOAD SETTINGS
    # ============================================================================

    # Upload directory
    UPLOAD_DIR: str = "./uploads"

    # Max upload size (MB)
    MAX_UPLOAD_SIZE_MB: int = 10

    # Allowed file types
    ALLOWED_UPLOAD_EXTENSIONS: str = ".rs,.txt,.md,.json"

    @property
    def allowed_upload_extensions_list(self) -> List[str]:
        """Parse allowed upload extensions."""
        return [
            ext.strip().lower()
            for ext in self.ALLOWED_UPLOAD_EXTENSIONS.split(",")
            if ext.strip()
        ]

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
        case_sensitive = True


# Global settings cache
_settings: Optional[Settings] = None


def _initialize_secrets(settings: Settings) -> None:
    """
    Initialize secrets from file-based secrets or the key manager.

    Priority order:
    1. Environment variable (e.g., JWT_SECRET_KEY=xxx)
    2. File-based secret (e.g., JWT_SECRET_KEY_FILE=/run/secrets/jwt_secret)
    3. Auto-generated from master key

    This ensures all cryptographic keys are derived from a single master key
    for easy backup and rotation, while supporting sovereign deployment with
    sops-encrypted secrets decrypted into tmpfs.
    """
    # Try file-based secrets first (sovereign deployment pattern)
    if not settings.SECRET_KEY:
        file_secret = _read_secret_file("SECRET_KEY")
        if file_secret:
            settings.SECRET_KEY = file_secret

    if not settings.JWT_SECRET_KEY:
        file_secret = _read_secret_file("JWT_SECRET_KEY")
        if file_secret:
            settings.JWT_SECRET_KEY = file_secret

    # Try file-based DATABASE_URL
    if settings.DATABASE_URL == "sqlite+aiosqlite:///./data/app.db":
        file_db_url = _read_secret_file("DATABASE_URL")
        if file_db_url:
            settings.DATABASE_URL = file_db_url

    # Fall back to key manager for any remaining unset secrets
    if not settings.SECRET_KEY or not settings.JWT_SECRET_KEY:
        # Import here to avoid circular imports
        from app.key_manager import get_key_manager

        km = get_key_manager()

        if not settings.SECRET_KEY:
            settings.SECRET_KEY = km.get_session_secret()
        if not settings.JWT_SECRET_KEY:
            settings.JWT_SECRET_KEY = km.get_jwt_secret()


@lru_cache()
def get_settings() -> Settings:
    """Get cached settings instance with auto-initialized secrets."""
    global _settings
    if _settings is None:
        _settings = Settings()
        _initialize_secrets(_settings)
    return _settings


def reload_settings() -> Settings:
    """Force reload settings (useful for testing)."""
    global _settings
    get_settings.cache_clear()
    _settings = None
    return get_settings()


def get_security_headers(debug: bool = False) -> dict:
    """Generate security headers based on environment."""
    headers = {
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "X-XSS-Protection": "1; mode=block",
        "Content-Security-Policy": (
            "default-src 'self'; "
            "script-src 'self' https://cdn.jsdelivr.net; "
            "style-src 'self' https://fonts.googleapis.com; "
            "font-src 'self' https://fonts.gstatic.com; "
            "img-src 'self' data: https://avatars.githubusercontent.com; "
            "connect-src 'self' http://localhost:8000 http://127.0.0.1:8000 ws: wss:; "
            "frame-ancestors 'none'; "
            "base-uri 'self'; "
            "form-action 'self';"
        ),
        "Referrer-Policy": "strict-origin-when-cross-origin",
        "Permissions-Policy": "geolocation=(), microphone=(), camera=(), payment=()",
        "X-Content-Security-Policy": "default-src 'self'",
    }

    if not debug:
        headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"

    return headers
