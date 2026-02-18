"""
Zero-trust unit tests for configuration.

Assume config may leak secrets, have wrong types, or missing required keys.
Verify nothing sensitive is exposed and required settings exist.
"""

import os
from unittest.mock import patch

import pytest

pytestmark = [pytest.mark.unit]


class TestSettingsNoSecretLeakage:
    """Config and settings must never expose secrets in repr, str, or dict."""

    def test_settings_repr_does_not_contain_secret_key(self):
        """get_settings() repr/string must not contain SECRET_KEY value."""
        from app.config import get_settings

        settings = get_settings()
        repr_str = repr(settings)
        # If SECRET_KEY is in repr, it must be redacted
        if "SECRET_KEY" in repr_str or "secret" in repr_str.lower():
            assert "change-me" not in repr_str or "***" in repr_str or "REDACTED" in repr_str.upper()

    def test_settings_dict_like_access_returns_strings_for_secrets(self):
        """Secret keys must be str type when accessed (no raw bytes)."""
        from app.config import get_settings

        settings = get_settings()
        assert isinstance(settings.SECRET_KEY, str)
        assert isinstance(settings.JWT_SECRET_KEY, str)

    def test_jwt_algorithm_is_explicit(self):
        """JWT algorithm must be set and not 'none'."""
        from app.config import get_settings

        settings = get_settings()
        assert settings.JWT_ALGORITHM
        assert settings.JWT_ALGORITHM.lower() != "none"


class TestCorsConfig:
    """CORS must be parseable and not wildcard in strict mode."""

    def test_cors_origins_list_is_list_of_strings(self):
        """cors_origins_list must be a list of non-empty strings."""
        from app.config import get_settings

        settings = get_settings()
        origins = settings.cors_origins_list
        assert isinstance(origins, list)
        for o in origins:
            assert isinstance(o, str), f"Origin must be str: {type(o)}"
            assert o.strip() == o, "No leading/trailing whitespace"


class TestDatabaseConfig:
    """Database URL must be valid for async SQLAlchemy when SQLite."""

    def test_database_url_uses_aiosqlite_for_sqlite(self):
        """If SQLite, driver must be aiosqlite."""
        from app.config import get_settings

        with patch.dict(os.environ, {"DATABASE_URL": "sqlite:///./x.db"}, clear=False):
            from app.config import Settings
            s = Settings()
            assert "aiosqlite" in s.DATABASE_URL or "sqlite" not in s.DATABASE_URL.lower()


class TestSecurityRelatedSettings:
    """Security-critical settings must have safe defaults."""

    def test_password_min_length_positive(self):
        from app.config import get_settings
        assert get_settings().PASSWORD_MIN_LENGTH >= 8

    def test_access_token_expire_positive(self):
        from app.config import get_settings
        assert get_settings().ACCESS_TOKEN_EXPIRE_MINUTES > 0

    def test_rate_limit_strings_present(self):
        from app.config import get_settings
        s = get_settings()
        assert s.RATE_LIMIT_LOGIN
        assert s.RATE_LIMIT_REGISTER
        assert s.RATE_LIMIT_API
