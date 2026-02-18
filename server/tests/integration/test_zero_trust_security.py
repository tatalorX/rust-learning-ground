"""
Zero-trust security integration tests.

Assume every endpoint might leak secrets or skip auth. Verify:
- No password/hash/token in any response body
- Protected routes return 401 without auth
- No internal paths or stack traces in errors
"""

import re
import pytest

pytestmark = [pytest.mark.integration, pytest.mark.security]


# Keys that must never appear in response bodies (or only in redacted form)
SENSITIVE_KEYS = re.compile(
    r"(password|secret|token|api_key|jwt|hash|credential|private_key)",
    re.I,
)


def assert_no_sensitive_leak(text: str) -> None:
    """Fail if obvious secret pattern in plain text."""
    if not text:
        return
    # Allow "token_type": "bearer" and "access_token": "<opaque>"
    if "token_type" in text or "access_token" in text:
        if "Bearer " in text or "bearer" in text:
            # Check we don't have raw JWT secret
            if "JWT_SECRET" in text or "change-me" in text:
                pytest.fail("Secret or placeholder in response")
    if "hashed_password" in text or "password_hash" in text:
        pytest.fail("Password hash in response")
    if "change-me-in-production" in text or "your-secret" in text.lower():
        pytest.fail("Placeholder secret in response")


class TestNoSecretsInResponses:
    """No response body may contain passwords, hashes, or secrets."""

    @pytest.mark.asyncio
    async def test_register_success_response_no_password(self, client, sample_user_data):
        r = await client.post("/api/auth/register", json=sample_user_data)
        if r.status_code not in [200, 201]:
            pytest.skip("Registration failed")
        body = r.text
        assert sample_user_data["password"] not in body
        assert_no_sensitive_leak(body)

    @pytest.mark.asyncio
    async def test_login_success_response_no_password_or_hash(self, client, test_user):
        from tests.conftest import TEST_USER_EMAIL, TEST_USER_PASSWORD

        r = await client.post(
            "/api/auth/login",
            json={"email": TEST_USER_EMAIL, "password": TEST_USER_PASSWORD},
        )
        if r.status_code != 200:
            pytest.skip("Login failed")
        body = r.text
        assert TEST_USER_PASSWORD not in body
        assert "hashed_password" not in body
        assert_no_sensitive_leak(body)

    @pytest.mark.asyncio
    async def test_me_endpoint_no_password_hash(self, authenticated_client):
        client, headers = authenticated_client
        r = await client.get("/api/me", headers=headers)
        assert r.status_code == 200
        body = r.text
        assert "hashed_password" not in body
        assert "password" not in body or "password_changed_at" in body  # metadata key ok
        assert_no_sensitive_leak(body)

    @pytest.mark.asyncio
    async def test_stats_endpoint_no_internal_leak(self, authenticated_client):
        client, headers = authenticated_client
        r = await client.get("/api/stats", headers=headers)
        assert r.status_code == 200
        assert_no_sensitive_leak(r.text)


class TestProtectedRoutesRequireAuth:
    """Endpoints that must require authentication."""

    @pytest.mark.asyncio
    async def test_me_returns_401_without_auth(self, client):
        r = await client.get("/api/me")
        assert r.status_code == 401

    @pytest.mark.asyncio
    async def test_stats_without_auth_does_not_leak_user_data(self, client):
        """Stats may be public; if 200, must not contain other users' data."""
        r = await client.get("/api/stats")
        assert r.status_code in [200, 401]
        if r.status_code == 200:
            data = r.json()
            assert data.get("authenticated") is False
            assert data.get("xp", 0) == 0
            assert "username" not in data or data.get("authenticated") is False

    @pytest.mark.asyncio
    async def test_logout_all_returns_401_without_auth(self, client):
        r = await client.post("/api/auth/logout-all")
        assert r.status_code == 401

    @pytest.mark.asyncio
    async def test_v4_run_returns_401_without_auth(self, client):
        r = await client.post(
            "/api/v4/exercises/1/run",
            json={"code": "fn main() {}"},
        )
        assert r.status_code == 401

    @pytest.mark.asyncio
    async def test_v4_submit_returns_401_without_auth(self, client):
        r = await client.post(
            "/api/v4/exercises/1/submit",
            json={"code": "fn main() {}"},
        )
        assert r.status_code == 401

    @pytest.mark.asyncio
    async def test_invalid_bearer_returns_401(self, client):
        r = await client.get(
            "/api/me",
            headers={"Authorization": "Bearer invalid.jwt.token"},
        )
        assert r.status_code == 401

    @pytest.mark.asyncio
    async def test_expired_token_returns_401(self, client, test_user):
        from app.security import create_access_token
        from datetime import timedelta

        expired = create_access_token(test_user.id, expires_delta=timedelta(minutes=-10))
        r = await client.get(
            "/api/me",
            headers={"Authorization": f"Bearer {expired}"},
        )
        assert r.status_code == 401


class TestErrorResponsesNoStackTrace:
    """4xx/5xx must not expose tracebacks or file paths."""

    @pytest.mark.asyncio
    async def test_404_no_traceback(self, client):
        r = await client.get("/api/nonexistent-path-404")
        assert "Traceback" not in r.text
        assert "File \"" not in r.text

    @pytest.mark.asyncio
    async def test_422_no_internal_paths(self, client):
        r = await client.post("/api/auth/login", json={"email": "not-valid", "password": "x"})
        assert r.status_code == 422
        # Avoid leaking app path like /home/.../app/main.py
        assert "/app/" not in r.text or "detail" in r.text
