"""
Integration tests for exercises API endpoints.
"""

import pytest
from httpx import AsyncClient


@pytest.mark.asyncio
async def test_list_exercises(client: AsyncClient):
    """Test listing exercises endpoint."""
    response = await client.get("/api/v4/exercises")

    assert response.status_code == 200
    data = response.json()

    assert "data" in data
    assert isinstance(data["data"], list)
    assert "pagination" in data

    if len(data["data"]) > 0:
        for exercise in data["data"]:
            assert "id" in exercise
            assert "title" in exercise
            assert "description" in exercise
            assert "difficulty" in exercise


@pytest.mark.asyncio
async def test_get_exercise_by_id(client: AsyncClient):
    """Test getting a specific exercise."""
    response = await client.get("/api/v4/exercises/1")

    if response.status_code == 200:
        data = response.json()
        assert "id" in data
        assert "title" in data
        assert "description" in data
        assert "difficulty" in data
        assert "category" in data
        assert "template_code" in data
    else:
        assert response.status_code == 404


@pytest.mark.asyncio
async def test_get_nonexistent_exercise(client: AsyncClient):
    """Test getting a non-existent exercise."""
    response = await client.get("/api/v4/exercises/99999")
    assert response.status_code == 404


@pytest.mark.asyncio
async def test_run_exercise_unauthorized(client: AsyncClient):
    """Test running exercise code without authentication."""
    response = await client.post(
        "/api/v4/exercises/1/run", json={"code": 'fn main() { println!("Hello"); }'}
    )

    assert response.status_code == 401


@pytest.mark.asyncio
async def test_run_exercise_authorized(authenticated_client):
    """Test running exercise code with authentication."""
    client, auth_headers = authenticated_client

    response = await client.post(
        "/api/v4/exercises/1/run",
        headers=auth_headers,
        json={"code": 'fn main() { println!("Hello, World!"); }'},
    )

    assert response.status_code in [200, 404]

    if response.status_code == 200:
        data = response.json()
        assert "success" in data
        assert "output" in data
        assert "error" in data
        assert "execution_time_ms" in data


@pytest.mark.asyncio
async def test_submit_exercise_unauthorized(client: AsyncClient):
    """Test submitting exercise without authentication."""
    response = await client.post(
        "/api/v4/exercises/1/submit", json={"code": 'fn main() { println!("Hello"); }'}
    )

    assert response.status_code == 401


@pytest.mark.asyncio
async def test_submit_exercise_authorized(authenticated_client):
    """Test submitting exercise with authentication."""
    client, auth_headers = authenticated_client

    response = await client.post(
        "/api/v4/exercises/1/submit",
        headers=auth_headers,
        json={"code": 'fn main() { println!("Hello, World!"); }'},
    )

    assert response.status_code in [200, 404]

    if response.status_code == 200:
        data = response.json()
        assert "success" in data
        assert "xp_earned" in data


@pytest.mark.asyncio
async def test_get_exercise_categories(client: AsyncClient):
    """Test getting exercise categories."""
    response = await client.get("/api/v4/exercises/categories")

    assert response.status_code == 200
    data = response.json()
    assert "categories" in data


@pytest.mark.asyncio
async def test_get_difficulty_levels(client: AsyncClient):
    """Test getting difficulty levels."""
    response = await client.get("/api/v4/exercises/difficulties")

    assert response.status_code == 200
    data = response.json()
    assert "difficulties" in data


@pytest.mark.asyncio
async def test_exercise_pagination(client: AsyncClient):
    """Test exercise list pagination."""
    response = await client.get("/api/v4/exercises?limit=5")

    assert response.status_code == 200
    data = response.json()
    assert "data" in data
    assert "pagination" in data
    assert len(data["data"]) <= 5


@pytest.mark.asyncio
async def test_exercise_filters(client: AsyncClient):
    """Test exercise filtering by difficulty."""
    response = await client.get("/api/v4/exercises?difficulty=1")

    assert response.status_code == 200
    data = response.json()
    assert "filters" in data

    for exercise in data["data"]:
        assert exercise.get("difficulty") == 1


@pytest.mark.asyncio
async def test_run_exercise_invalid_code(authenticated_client):
    """Test running invalid Rust code."""
    client, auth_headers = authenticated_client

    response = await client.post(
        "/api/v4/exercises/1/run",
        headers=auth_headers,
        json={"code": "this is not valid rust code {"},
    )

    assert response.status_code == 200
    data = response.json()
    assert "success" in data
    assert "error" in data


@pytest.mark.asyncio
async def test_run_empty_code(authenticated_client):
    """Test running empty code."""
    client, auth_headers = authenticated_client

    response = await client.post(
        "/api/v4/exercises/1/run",
        headers=auth_headers,
        json={"code": ""},
    )

    assert response.status_code == 400


@pytest.mark.asyncio
async def test_run_code_too_long(authenticated_client):
    """Test running code that exceeds length limit."""
    client, auth_headers = authenticated_client

    response = await client.post(
        "/api/v4/exercises/1/run",
        headers=auth_headers,
        json={"code": "x" * 50001},
    )

    assert response.status_code == 400
