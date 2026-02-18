# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Security utilities: password hashing, JWT tokens, CSRF protection.
"""

import hashlib
import hmac
import secrets
from datetime import datetime, timedelta, timezone
from typing import Optional, Tuple

from jose import JWTError, jwt
from passlib.context import CryptContext
from itsdangerous import URLSafeTimedSerializer, BadSignature, SignatureExpired

from app.config import get_settings
from app.schemas import ErrorResponse

settings = get_settings()

# Password hashing with Argon2 (OWASP recommended)
pwd_context = CryptContext(
    schemes=["argon2"],
    deprecated="auto",
    argon2__time_cost=3,  # Iterations
    argon2__memory_cost=65536,  # 64 MB
    argon2__parallelism=4,  # Parallel threads
    argon2__hash_len=32,
    argon2__salt_len=16,
)

# CSRF token serializer - initialized lazily to allow key manager to generate keys
_csrf_serializer = None


def get_csrf_serializer():
    """Get the CSRF token serializer (lazily initialized)."""
    global _csrf_serializer
    if _csrf_serializer is None:
        from app.key_manager import get_csrf_secret

        _csrf_serializer = URLSafeTimedSerializer(get_csrf_secret(), salt="csrf-token")
    return _csrf_serializer


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """Verify a plain password against a hash."""
    return pwd_context.verify(plain_password, hashed_password)


def hash_password(password: str) -> str:
    """Hash a password using Argon2."""
    return pwd_context.hash(password)


def create_access_token(user_id: int, expires_delta: Optional[timedelta] = None) -> str:
    """Create a JWT access token."""
    if expires_delta:
        expire = datetime.now(timezone.utc) + expires_delta
    else:
        expire = datetime.now(timezone.utc) + timedelta(
            minutes=settings.ACCESS_TOKEN_EXPIRE_MINUTES
        )

    to_encode = {
        "sub": str(user_id),  # Subject: user ID
        "exp": expire,
        "iat": datetime.now(timezone.utc),  # Issued at
        "type": "access",
        "jti": secrets.token_urlsafe(16),  # Unique token ID
    }
    return jwt.encode(
        to_encode, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM
    )


def create_refresh_token(user_id: int) -> Tuple[str, str, datetime]:
    """
    Create a JWT refresh token.
    Returns: (token, token_hash, expires_at)
    """
    expires_at = datetime.now(timezone.utc) + timedelta(
        days=settings.REFRESH_TOKEN_EXPIRE_DAYS
    )

    to_encode = {
        "sub": str(user_id),
        "exp": expires_at,
        "iat": datetime.now(timezone.utc),
        "type": "refresh",
        "jti": secrets.token_urlsafe(16),
    }

    token = jwt.encode(
        to_encode, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM
    )
    # Hash the token for database storage
    token_hash = hashlib.sha256(token.encode()).hexdigest()

    return token, token_hash, expires_at


def decode_token(token: str) -> Optional[dict]:
    """Decode and validate a JWT token."""
    try:
        payload = jwt.decode(
            token, settings.JWT_SECRET_KEY, algorithms=[settings.JWT_ALGORITHM]
        )
        return payload
    except JWTError:
        return None


def verify_refresh_token(token: str) -> Optional[int]:
    """Verify a refresh token and return user_id if valid."""
    payload = decode_token(token)
    if not payload:
        return None

    if payload.get("type") != "refresh":
        return None

    try:
        return int(payload.get("sub"))
    except (ValueError, TypeError):
        return None


def get_token_hash(token: str) -> str:
    """Get SHA256 hash of a token for database lookup."""
    return hashlib.sha256(token.encode()).hexdigest()


# ============== CSRF Protection ==============


def generate_csrf_token() -> str:
    """Generate a CSRF token."""
    return get_csrf_serializer().dumps(secrets.token_urlsafe(32))


def validate_csrf_token(token: str, max_age: int = 3600) -> bool:
    """Validate a CSRF token."""
    try:
        get_csrf_serializer().loads(token, max_age=max_age)
        return True
    except (BadSignature, SignatureExpired):
        return False


# ============== Security Helpers ==============


def generate_secure_token(length: int = 32) -> str:
    """Generate a cryptographically secure random token."""
    return secrets.token_urlsafe(length)


def constant_time_compare(val1: str, val2: str) -> bool:
    """Compare two strings in constant time to prevent timing attacks."""
    return hmac.compare_digest(val1.encode(), val2.encode())


def sanitize_input(text: str, max_length: int = 1000) -> str:
    """Sanitize user input to prevent injection attacks."""
    if not text:
        return ""
    # Remove null bytes and control characters
    text = "".join(char for char in text if char.isprintable() or char in "\n\r\t")
    # Limit length
    return text[:max_length]


def is_safe_redirect_url(url: str) -> bool:
    """
    Check if a redirect URL is safe (prevents open redirect attacks).

    SECURITY FIX: Now properly handles protocol-relative URLs (H-04)
    """
    if not url:
        return False

    # Strip whitespace
    url = url.strip()
    url_lower = url.lower()

    # Block dangerous schemes
    unsafe_schemes = ["javascript:", "data:", "vbscript:", "file:", "about:"]
    for scheme in unsafe_schemes:
        if url_lower.startswith(scheme):
            return False

    # SECURITY FIX: Block protocol-relative URLs (//evil.com) (H-04)
    if url.startswith("//"):
        return False

    # Allow relative URLs (starting with / but not //)
    if url.startswith("/") and not url.startswith("//"):
        return True

    # Reject absolute URLs to external sites
    if "://" in url:
        return False

    return True


# ============== Cookie Settings ==============


def get_cookie_settings() -> dict:
    """Get secure cookie settings."""
    return {
        "httponly": True,
        "secure": settings.COOKIE_SECURE,
        "samesite": settings.COOKIE_SAMESITE,
        "domain": settings.COOKIE_DOMAIN,
    }


# ============== Rate Limiting Keys ==============


def get_rate_limit_key(identifier: str, endpoint: str) -> str:
    """Generate a rate limit key."""
    return f"ratelimit:{endpoint}:{identifier}"
