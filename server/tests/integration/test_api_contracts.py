"""
Zero-trust API contract tests.

Assume nothing about response shape. Assert every key and type required
by clients. No trust in docstrings or OpenAPI alone.
"""

import pytest


pytestmark = [pytest.mark.integration]


class TestHealthContract:
    """Health and info endpoints must return exact shapes."""

    @pytest.mark.asyncio
    async def test_api_health_response_shape(self, client):
        r = await client.get("/api/health")
        assert r.status_code == 200
        data = r.json()
        assert isinstance(data, dict)
        assert "status" in data

    @pytest.mark.asyncio
    async def test_v4_health_response_shape(self, client):
        r = await client.get("/api/v4/health")
        assert r.status_code == 200
        data = r.json()
        assert data.get("status") == "healthy"
        assert "version" in data
        assert "api" in data
        assert data.get("api") == "v4"

    @pytest.mark.asyncio
    async def test_v4_info_response_shape(self, client):
        r = await client.get("/api/v4/info")
        assert r.status_code == 200
        data = r.json()
        assert "name" in data
        assert "version" in data


class TestExercisesListContract:
    """GET /api/v4/exercises must have data + pagination + filters."""

    @pytest.mark.asyncio
    async def test_list_exercises_top_level_keys(self, client):
        r = await client.get("/api/v4/exercises?limit=2")
        assert r.status_code == 200
        data = r.json()
        assert "data" in data
        assert "pagination" in data
        assert "filters" in data
        assert isinstance(data["data"], list)
        assert isinstance(data["pagination"], dict)
        assert isinstance(data["filters"], dict)

    @pytest.mark.asyncio
    async def test_pagination_keys(self, client):
        r = await client.get("/api/v4/exercises?limit=1")
        assert r.status_code == 200
        pag = r.json()["pagination"]
        assert "cursor" in pag
        assert "next_cursor" in pag
        assert "limit" in pag
        assert "has_more" in pag
        assert "total" in pag
        assert isinstance(pag["limit"], int)
        assert isinstance(pag["total"], int)
        assert isinstance(pag["has_more"], bool)

    @pytest.mark.asyncio
    async def test_each_exercise_item_shape(self, client):
        r = await client.get("/api/v4/exercises?limit=3")
        assert r.status_code == 200
        for item in r.json()["data"]:
            assert "id" in item
            assert "title" in item
            assert "description" in item
            assert "difficulty" in item
            assert "category" in item
            assert isinstance(item["id"], int)
            assert isinstance(item["title"], str)
            assert isinstance(item["difficulty"], int)


class TestSingleExerciseContract:
    """GET /api/v4/exercises/{id} must return full exercise or 404."""

    @pytest.mark.asyncio
    async def test_get_exercise_200_shape(self, client):
        r = await client.get("/api/v4/exercises/1")
        if r.status_code != 200:
            pytest.skip("Exercise 1 may not exist in this environment")
        data = r.json()
        assert "id" in data
        assert "title" in data
        assert "description" in data
        assert "category" in data
        assert "difficulty" in data
        assert "template_code" in data
        assert "concepts" in data
        assert isinstance(data["template_code"], str)
        assert isinstance(data["concepts"], list)

    @pytest.mark.asyncio
    async def test_get_nonexistent_returns_404(self, client):
        r = await client.get("/api/v4/exercises/999999")
        assert r.status_code == 404
        data = r.json()
        assert "detail" in data or "message" in data


class TestCategoriesAndDifficultiesContract:
    """Categories and difficulties endpoints shape."""

    @pytest.mark.asyncio
    async def test_categories_response_shape(self, client):
        r = await client.get("/api/v4/exercises/categories")
        assert r.status_code == 200
        data = r.json()
        assert "categories" in data
        assert isinstance(data["categories"], list)

    @pytest.mark.asyncio
    async def test_difficulties_response_shape(self, client):
        r = await client.get("/api/v4/exercises/difficulties")
        assert r.status_code == 200
        data = r.json()
        assert "difficulties" in data
        assert isinstance(data["difficulties"], list)


class TestRunResponseContract:
    """POST run must return success, output, error, execution_time_ms."""

    @pytest.mark.asyncio
    async def test_run_response_shape_when_authenticated(self, authenticated_client):
        client, headers = authenticated_client
        r = await client.post(
            "/api/v4/exercises/1/run",
            headers=headers,
            json={"code": 'fn main() { println!("x"); }'},
        )
        if r.status_code == 404:
            pytest.skip("Exercise 1 not available")
        assert r.status_code == 200
        data = r.json()
        assert "success" in data
        assert "output" in data
        assert "error" in data
        assert "execution_time_ms" in data
        assert isinstance(data["success"], bool)
        assert isinstance(data["output"], str)
        assert isinstance(data["execution_time_ms"], int)


class TestSubmitResponseContract:
    """POST submit must return xp_earned and run fields."""

    @pytest.mark.asyncio
    async def test_submit_response_shape_when_authenticated(self, authenticated_client):
        client, headers = authenticated_client
        r = await client.post(
            "/api/v4/exercises/1/submit",
            headers=headers,
            json={"code": 'fn main() { println!("x"); }'},
        )
        if r.status_code == 404:
            pytest.skip("Exercise 1 not available")
        assert r.status_code == 200
        data = r.json()
        assert "success" in data
        assert "xp_earned" in data
        assert "output" in data
        assert "execution_time_ms" in data
        assert isinstance(data["xp_earned"], int)
        assert data["xp_earned"] >= 0
