"""
Pytest configuration and fixtures for Rust Learning Ground backend tests.

This module provides:
- Database fixtures with transaction rollback
- Authentication fixtures
- Test client with dependency overrides
- Sample data fixtures
"""

import asyncio
import os
import pytest
import pytest_asyncio
from datetime import datetime, timedelta
from typing import AsyncGenerator, Generator
from httpx import AsyncClient

# Relax rate limits in tests so auth tests don't get 429 (zero-trust: we still assert behavior)
os.environ.setdefault("RATE_LIMIT_LOGIN", "1000/minute")
os.environ.setdefault("RATE_LIMIT_REGISTER", "1000/minute")
os.environ.setdefault("RATE_LIMIT_API", "10000/minute")
os.environ.setdefault("RATE_LIMIT_REFRESH", "1000/minute")

# Add parent directory to path for imports
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession, async_sessionmaker
from sqlalchemy.pool import StaticPool

# Import database and models FIRST (before app.main to ensure tables are registered)
from app.database import Base, get_db

# Import all models to ensure tables are created
from app.models import User, RefreshToken, UserProgress, AuditLog
from app.models_school import (
    UserSchoolProfile,
    Classroom,
    ClassroomMember,
    Assignment,
    AssignmentSubmission,
    RankDefinition,
    TitleDefinition,
    UserTitle,
    TreatTransaction,
    LeaderboardEntry,
    GradeContentMapping,
    Notification,
    GradeLevel,
    UserRole,
    RankTier,
)
from app.auth_service import hash_password

# Import app LAST (after models are registered)
from app.main import app

# =============================================================================
# Configuration
# =============================================================================

# Use in-memory SQLite for tests
TEST_DATABASE_URL = "sqlite+aiosqlite:///:memory:"

# Test user data
TEST_USER_EMAIL = "test@example.com"
TEST_USER_PASSWORD = "testpassword123"
TEST_USER_USERNAME = "testuser"


# =============================================================================
# Database Fixtures
# =============================================================================


@pytest_asyncio.fixture
async def db_engine():
    """
    Create a test database engine with in-memory SQLite.

    This fixture runs once per test for isolation.
    """
    # Import all models to register them with metadata
    from app.models import User, RefreshToken, UserProgress, AuditLog  # noqa: F401
    from app.models_school import (  # noqa: F401
        UserSchoolProfile,
        Classroom,
        ClassroomMember,
        Assignment,
        AssignmentSubmission,
        RankDefinition,
        TitleDefinition,
        UserTitle,
        TreatTransaction,
        LeaderboardEntry,
        GradeContentMapping,
        Notification,
    )

    engine = create_async_engine(
        TEST_DATABASE_URL,
        echo=False,  # Set to True for SQL debugging
        future=True,
        poolclass=StaticPool,  # Use static pool to share connection
        connect_args={"check_same_thread": False},
    )

    # Create all tables
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    yield engine

    # Cleanup
    await engine.dispose()


@pytest_asyncio.fixture
async def db_session(db_engine) -> AsyncGenerator[AsyncSession, None]:
    """
    Create a fresh database session for each test.

    Automatically rolls back after each test to ensure isolation.
    """
    async_session = async_sessionmaker(
        db_engine, class_=AsyncSession, expire_on_commit=False
    )

    async with async_session() as session:
        yield session
        # Rollback any changes
        await session.rollback()


# =============================================================================
# Application Client Fixtures
# =============================================================================


@pytest_asyncio.fixture
async def client(db_session) -> AsyncGenerator[AsyncClient, None]:
    """
    Create an HTTP test client with database override.

    This allows tests to use the test database instead of the real one.
    """

    async def override_get_db():
        try:
            yield db_session
        finally:
            pass

    # Override the dependency
    app.dependency_overrides[get_db] = override_get_db

    async with AsyncClient(app=app, base_url="http://test") as ac:
        yield ac

    # Clear overrides after test
    app.dependency_overrides.clear()


@pytest_asyncio.fixture
async def authenticated_client(
    client, test_user
) -> AsyncGenerator[tuple[AsyncClient, dict], None]:
    """
    Create an authenticated test client.

    Returns a tuple of (client, auth_headers).
    """
    # Login to get tokens
    response = await client.post(
        "/api/auth/login",
        json={"email": TEST_USER_EMAIL, "password": TEST_USER_PASSWORD},
    )

    # If rate limited, skip this test
    if response.status_code == 429:
        pytest.skip("Rate limited")

    assert response.status_code == 200, f"Login failed: {response.text}"
    data = response.json()

    auth_headers = {"Authorization": f"Bearer {data['access_token']}"}

    yield client, auth_headers


# =============================================================================
# User Fixtures
# =============================================================================


@pytest_asyncio.fixture
async def test_user(db_session) -> User:
    """Create a basic test user."""
    user = User(
        email=TEST_USER_EMAIL,
        username=TEST_USER_USERNAME,
        hashed_password=hash_password(TEST_USER_PASSWORD),
        is_active=True,
        is_verified=True,
        display_name="Test User",
        created_at=datetime.utcnow(),
    )

    db_session.add(user)
    await db_session.commit()
    await db_session.refresh(user)

    return user


@pytest_asyncio.fixture
async def test_user_with_profile(db_session) -> User:
    """Create a test user with a school profile in a single session."""
    # Create user first
    user = User(
        email=TEST_USER_EMAIL,
        username=TEST_USER_USERNAME,
        hashed_password=hash_password(TEST_USER_PASSWORD),
        is_active=True,
        is_verified=True,
        display_name="Test User",
        created_at=datetime.utcnow(),
    )
    db_session.add(user)
    await db_session.commit()
    await db_session.refresh(user)

    # Create profile in the same session
    profile = UserSchoolProfile(
        user_id=user.id,
        grade_level=GradeLevel.PRIMARY_5,
        total_xp=1000,
        current_streak=7,
        last_activity_date=datetime.utcnow(),
    )
    db_session.add(profile)
    await db_session.commit()

    return user


@pytest_asyncio.fixture
async def test_teacher(db_session) -> User:
    """Create a test teacher user."""
    teacher = User(
        email="teacher@example.com",
        username="testteacher",
        hashed_password=hash_password("teacherpass123"),
        is_active=True,
        is_verified=True,
        display_name="Test Teacher",
        created_at=datetime.utcnow(),
    )

    db_session.add(teacher)
    await db_session.commit()
    await db_session.refresh(teacher)

    # Create teacher profile
    profile = UserSchoolProfile(
        user_id=teacher.id,
        grade_level=GradeLevel.HIGH_12,
        total_xp=5000,
        role=UserRole.TEACHER,
    )

    db_session.add(profile)
    await db_session.commit()

    return teacher


@pytest_asyncio.fixture
async def test_admin(db_session) -> User:
    """Create a test admin user."""
    admin = User(
        email="admin@example.com",
        username="testadmin",
        hashed_password=hash_password("adminpass123"),
        is_active=True,
        is_verified=True,
        display_name="Test Admin",
        is_admin=True,
        created_at=datetime.utcnow(),
    )

    db_session.add(admin)
    await db_session.commit()
    await db_session.refresh(admin)

    return admin


# =============================================================================
# School Platform Fixtures
# =============================================================================


@pytest_asyncio.fixture
async def test_classroom(db_session, test_teacher) -> Classroom:
    """Create a test classroom."""
    classroom = Classroom(
        name="Test Classroom",
        description="A classroom for testing",
        teacher_id=test_teacher.id,
        grade_level=GradeLevel.PRIMARY_5,
        max_students=30,
        code="TEST123",
        is_archived=False,
        created_at=datetime.utcnow(),
    )

    db_session.add(classroom)
    await db_session.commit()
    await db_session.refresh(classroom)

    return classroom


@pytest_asyncio.fixture
async def test_classroom_with_student(
    db_session, test_classroom, test_user
) -> Classroom:
    """Create a test classroom with a student member."""
    member = ClassroomMember(
        classroom_id=test_classroom.id,
        user_id=test_user.id,
        joined_at=datetime.utcnow(),
    )

    db_session.add(member)
    await db_session.commit()

    return test_classroom


@pytest_asyncio.fixture
async def test_assignment(db_session, test_classroom, test_teacher) -> Assignment:
    """Create a test assignment."""
    assignment = Assignment(
        title="Test Assignment",
        description="Complete exercises 1-5",
        classroom_id=test_classroom.id,
        created_by=test_teacher.id,
        exercise_ids=[1, 2, 3],
        due_date=datetime.utcnow() + timedelta(days=7),
        created_at=datetime.utcnow(),
    )

    db_session.add(assignment)
    await db_session.commit()
    await db_session.refresh(assignment)

    return assignment


@pytest_asyncio.fixture
async def test_rank_definitions(db_session):
    """Create rank definitions for testing."""
    ranks = [
        RankDefinition(rank_tier=RankTier.NOVICE, name="Novice", min_xp=0, icon="🌱"),
        RankDefinition(
            rank_tier=RankTier.APPRENTICE, name="Apprentice", min_xp=1000, icon="📚"
        ),
        RankDefinition(
            rank_tier=RankTier.EXPERT, name="Expert", min_xp=10000, icon="🎯"
        ),
    ]

    for rank in ranks:
        db_session.add(rank)

    await db_session.commit()


@pytest_asyncio.fixture
async def test_title_definitions(db_session):
    """Create title definitions for testing."""
    titles = [
        TitleDefinition(
            key="first_steps",
            name="First Steps",
            description="Solve your first problem",
            category="achievement",
            requirement_type="problems_solved",
            requirement_value=1,
            icon="👣",
        ),
        TitleDefinition(
            key="dedicated",
            name="Dedicated",
            description="Solve 50 problems",
            category="achievement",
            requirement_type="problems_solved",
            requirement_value=50,
            icon="💪",
        ),
    ]

    for title in titles:
        db_session.add(title)

    await db_session.commit()


# =============================================================================
# Sample Data Fixtures
# =============================================================================


@pytest.fixture
def sample_user_data() -> dict:
    """Return sample user registration data."""
    return {
        "email": "newuser@example.com",
        "password": "NewPass123!",
        "username": "newuser",
        "display_name": "New User",
    }


@pytest.fixture
def sample_classroom_data() -> dict:
    """Return sample classroom creation data."""
    return {
        "name": "New Classroom",
        "description": "A new test classroom",
        "grade_level": 5,
        "max_students": 25,
    }


@pytest.fixture
def sample_assignment_data() -> dict:
    """Return sample assignment creation data."""
    return {
        "title": "New Assignment",
        "description": "Complete these exercises",
        "exercise_ids": [1, 2, 3],
        "due_date": (datetime.utcnow() + timedelta(days=7)).isoformat(),
    }


@pytest.fixture
def sample_rust_code() -> str:
    """Return a valid Rust code snippet."""
    return """fn main() {
    println!("Hello, World!");
}"""


@pytest.fixture
def sample_rust_code_with_error() -> str:
    """Return a Rust code snippet with syntax error."""
    return """fn main() {
    println!("Hello, World!"
}"""


# Configure pytest-asyncio
pytest_plugins = ("pytest_asyncio",)
