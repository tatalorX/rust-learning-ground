"""
API Versioning Router for v4 Preparation.

Provides /api/v4/ endpoints alongside /api/ for backward compatibility.
New v4 features: cursor pagination, advanced filtering, GraphQL preparation.
"""

from fastapi import APIRouter, Depends, HTTPException, Request, status
from pydantic import BaseModel
from sqlalchemy.ext.asyncio import AsyncSession
from typing import List, Optional

from app.database import get_db
from app.auth_deps import get_current_user, optional_user
from app.models import User
from app.exercise_loader import get_exercise_loader
from app.code_runner import get_code_runner
from slowapi import Limiter
from slowapi.util import get_remote_address

# Initialize limiter for this router
limiter = Limiter(key_func=get_remote_address)

router = APIRouter(prefix="/v4", tags=["v4"])


class RunCodeRequest(BaseModel):
    code: str


class RunCodeResponse(BaseModel):
    success: bool
    output: str
    error: Optional[str] = None
    execution_time_ms: int


class SubmitCodeRequest(BaseModel):
    code: str


class SubmitCodeResponse(BaseModel):
    success: bool
    xp_earned: int
    output: str
    expected_output: Optional[str] = None
    error: Optional[str] = None
    execution_time_ms: int


# =============================================================================
# Health & Info
# =============================================================================


@router.get("/health")
async def health_check_v4():
    """v4 health check endpoint."""
    return {
        "status": "healthy",
        "version": "4.0.0-alpha",
        "api": "v4",
        "features": ["cursor_pagination", "advanced_filtering", "graphql_prep"],
    }


@router.get("/info")
async def api_info():
    """API information and versioning."""
    return {
        "name": "Rust Learning Ground API",
        "version": "4.0.0-alpha",
        "documentation": "/api/docs",
        "graphql": "/api/v4/graphql",
    }


# =============================================================================
# Exercises (v4 with cursor pagination)
# =============================================================================


@router.get("/exercises")
async def list_exercises_v4(
    cursor: Optional[str] = None,
    limit: int = 20,
    difficulty: Optional[int] = None,
    category: Optional[str] = None,
    search: Optional[str] = None,
    current_user: Optional[User] = Depends(optional_user),
    db: AsyncSession = Depends(get_db),
):
    """
    List exercises with cursor pagination and advanced filtering.

    v4 Improvements:
    - Cursor-based pagination (better for large datasets)
    - Full-text search
    - Multiple filter combinations
    """
    loader = get_exercise_loader()
    cursor_int = int(cursor) if cursor else None

    result = loader.list_exercises(
        cursor=cursor_int,
        limit=limit,
        difficulty=difficulty,
        category=category,
        search=search,
    )

    return result


@router.get("/exercises/categories")
async def get_exercise_categories():
    """Get all exercise categories with counts."""
    loader = get_exercise_loader()
    return loader.get_categories()


@router.get("/exercises/difficulties")
async def get_difficulty_levels():
    """Get available difficulty levels with counts."""
    loader = get_exercise_loader()
    return {"difficulties": loader.get_difficulties()}


@router.get("/exercises/{exercise_id}")
async def get_exercise_v4(
    exercise_id: int,
    current_user: Optional[User] = Depends(optional_user),
    db: AsyncSession = Depends(get_db),
):
    """Get single exercise with v4 enriched data."""
    loader = get_exercise_loader()
    exercise = loader.get_exercise(exercise_id)

    if not exercise:
        raise HTTPException(status_code=404, detail=f"Exercise {exercise_id} not found")

    return {
        "id": exercise.id,
        "title": exercise.title,
        "description": exercise.description,
        "category": exercise.category,
        "difficulty": exercise.difficulty,
        "concepts": exercise.concepts,
        "bonus": exercise.bonus,
        "template_code": exercise.template_code,
    }


@router.post("/exercises/{exercise_id}/run", response_model=RunCodeResponse)
@limiter.limit("10/minute")  # Rate limit: 10 runs per minute per user
async def run_exercise_code(
    exercise_id: int,
    request: RunCodeRequest,
    request_obj: Request,  # Required for rate limiting
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db),
):
    """Run exercise code in sandboxed environment."""
    if not request.code or len(request.code.strip()) == 0:
        raise HTTPException(status_code=400, detail="Code is required")

    if len(request.code) > 50000:
        raise HTTPException(
            status_code=400, detail="Code too long (max 50000 characters)"
        )

    runner = get_code_runner()
    result = await runner.run_code(request.code, exercise_id)

    return RunCodeResponse(
        success=result.success,
        output=result.output,
        error=result.error,
        execution_time_ms=result.execution_time_ms,
    )


@router.post("/exercises/{exercise_id}/submit", response_model=SubmitCodeResponse)
@limiter.limit("30/minute")  # Rate limit: 30 submissions per minute per user
async def submit_exercise(
    exercise_id: int,
    request: SubmitCodeRequest,
    request_obj: Request,  # Required for rate limiting
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db),
):
    """Submit exercise solution for grading."""
    if not request.code or len(request.code.strip()) == 0:
        raise HTTPException(status_code=400, detail="Code is required")

    loader = get_exercise_loader()
    exercise = loader.get_exercise(exercise_id)

    if not exercise:
        raise HTTPException(status_code=404, detail=f"Exercise {exercise_id} not found")

    runner = get_code_runner()

    difficulty = exercise.difficulty
    base_xp = {1: 10, 2: 20, 3: 30, 4: 50, 5: 100}
    xp_earned = base_xp.get(difficulty, 10)

    result = await runner.run_code(request.code, exercise_id)

    success = result.success

    return SubmitCodeResponse(
        success=success,
        xp_earned=xp_earned if success else 0,
        output=result.output,
        expected_output=None,
        error=result.error,
        execution_time_ms=result.execution_time_ms,
    )


# =============================================================================
# User Profile (v4)
# =============================================================================


@router.get("/me")
async def get_current_user_v4(
    current_user: User = Depends(get_current_user), db: AsyncSession = Depends(get_db)
):
    """Get current user with v4 enriched profile."""
    return {
        "id": current_user.id,
        "email": current_user.email,
        "username": current_user.username,
        "version": "v4",
        "enriched_profile": True,
    }


# =============================================================================
# GraphQL Preparation
# =============================================================================


@router.get("/graphql/schema")
async def graphql_schema():
    """
    GraphQL schema endpoint (preparation for v4).

    Returns GraphQL schema definition for client-side preparation.
    Actual GraphQL endpoint will be at /api/v4/graphql
    """
    return {
        "status": "coming_soon",
        "message": "GraphQL API will be available in v4",
        "schema_preview": {
            "queries": ["exercises", "user", "classrooms", "progress"],
            "mutations": ["submitExercise", "joinClassroom", "updateProfile"],
        },
    }


# =============================================================================
# Migration Guide
# =============================================================================


@router.get("/migration")
async def migration_guide():
    """API migration guide from v3 to v4."""
    return {
        "current_version": "v3",
        "target_version": "v4",
        "breaking_changes": [],
        "deprecations": [],
        "new_features": [
            "Cursor-based pagination",
            "Advanced filtering",
            "GraphQL API",
            "Real-time subscriptions",
            "Bulk operations",
        ],
        "migration_timeline": {
            "v3_api": "Stable - no breaking changes",
            "v4_api": "Alpha - in development",
            "v3_deprecation": "Planned for Q3 2026",
            "v4_stable": "Target Q2 2026",
        },
    }
