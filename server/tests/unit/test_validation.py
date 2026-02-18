"""
Zero-trust validation and boundary tests.

Assume all inputs are hostile: wrong types, huge lengths, injection payloads.
Verify the app rejects or sanitizes every bad case.
"""

import json
import pytest

pytestmark = [pytest.mark.unit, pytest.mark.security]


class TestRegistrationValidation:
    """Registration must reject invalid input - do not trust client."""

    @pytest.mark.asyncio
    async def test_register_rejects_empty_json(self, client):
        response = await client.post("/api/auth/register", json={})
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_register_rejects_null_email(self, client):
        response = await client.post(
            "/api/auth/register",
            json={
                "email": None,
                "password": "ValidPass123!",
                "username": "u",
            },
        )
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_register_rejects_invalid_email_format(self, client):
        for bad in ["", "a", "@", "a@", "@b.com", "a@.com", "a space@x.com"]:
            response = await client.post(
                "/api/auth/register",
                json={
                    "email": bad,
                    "password": "ValidPass123!",
                    "username": "user",
                },
            )
            assert response.status_code == 422, f"Should reject email: {bad!r}"

    @pytest.mark.asyncio
    async def test_register_rejects_short_password(self, client):
        response = await client.post(
            "/api/auth/register",
            json={
                "email": "v@x.com",
                "password": "short",
                "username": "u",
            },
        )
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_register_rejects_non_string_types(self, client):
        """Send wrong types; must get 422 or 400."""
        response = await client.post(
            "/api/auth/register",
            json={
                "email": 12345,
                "password": "ValidPass123!",
                "username": "u",
            },
        )
        assert response.status_code in [400, 422]


class TestLoginValidation:
    """Login must not accept malformed or dangerous input."""

    @pytest.mark.asyncio
    async def test_login_rejects_empty_body(self, client):
        response = await client.post("/api/auth/login", json={})
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_login_rejects_sql_like_input(self, client):
        payloads = [
            {"email": "'; DROP TABLE users; --", "password": "x"},
            {"email": "admin'--", "password": "x"},
            {"email": "1 OR 1=1", "password": "x"},
        ]
        for body in payloads:
            response = await client.post("/api/auth/login", json=body)
            assert response.status_code in [401, 422], f"Must not crash on: {body}"

    @pytest.mark.asyncio
    async def test_login_rejects_content_type_without_json(self, client):
        response = await client.post(
            "/api/auth/login",
            content="email=test@x.com&password=secret",
            headers={"Content-Type": "application/x-www-form-urlencoded"},
        )
        # FastAPI may accept form; if so we get 401. We require no 500.
        assert response.status_code != 500


class TestV4ExerciseRunValidation:
    """Run/submit code endpoints must validate strictly."""

    @pytest.mark.asyncio
    async def test_run_rejects_missing_code_key(self, authenticated_client):
        client, headers = authenticated_client
        response = await client.post(
            "/api/v4/exercises/1/run",
            headers=headers,
            json={},
        )
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_run_rejects_non_string_code(self, authenticated_client):
        client, headers = authenticated_client
        response = await client.post(
            "/api/v4/exercises/1/run",
            headers=headers,
            json={"code": 12345},
        )
        assert response.status_code == 422

    @pytest.mark.asyncio
    async def test_run_negative_exercise_id_does_not_leak_or_crash(self, authenticated_client):
        """Negative exercise id must not crash; ideally 404/422."""
        client, headers = authenticated_client
        response = await client.post(
            "/api/v4/exercises/-1/run",
            headers=headers,
            json={"code": "fn main() {}"},
        )
        assert response.status_code != 500
        if response.status_code == 200:
            data = response.json()
            assert "success" in data and "error" in data

    @pytest.mark.asyncio
    async def test_run_rejects_non_integer_exercise_id(self, authenticated_client):
        client, headers = authenticated_client
        response = await client.get("/api/v4/exercises/not_a_number")
        assert response.status_code == 422


class TestResponseNoSensitiveData:
    """No response must echo secrets or internal paths."""

    @pytest.mark.asyncio
    async def test_login_failure_does_not_reveal_whether_email_exists(self, client):
        """Wrong password vs unknown email: same status, no user enumeration."""
        r1 = await client.post(
            "/api/auth/login",
            json={"email": "nonexistent@example.com", "password": "wrong"},
        )
        r2 = await client.post(
            "/api/auth/login",
            json={"email": "test@example.com", "password": "wrong"},
        )
        assert r1.status_code == r2.status_code, "Avoid user enumeration"
        assert r1.status_code == 401

    @pytest.mark.asyncio
    async def test_error_response_no_stack_trace(self, client):
        """4xx/5xx must not include traceback in body."""
        response = await client.get("/api/nonexistent-route-xyz")
        body = response.text
        assert "Traceback" not in body
        assert "File \"" not in body
        assert ".py\"" not in body or "detail" in body.lower()
