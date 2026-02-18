# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Database configuration and session management.
"""

import asyncio
from typing import AsyncGenerator

from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine, async_sessionmaker
from sqlalchemy.orm import declarative_base
from sqlalchemy.pool import NullPool

from app.config import get_settings

settings = get_settings()


# Create async engine with connection pooling for PostgreSQL
def create_engine_with_pooling():
    """Create database engine with appropriate pooling strategy."""
    is_sqlite = settings.DATABASE_URL.startswith("sqlite")

    if is_sqlite:
        # SQLite: Use NullPool to avoid threading issues
        return create_async_engine(
            settings.DATABASE_URL,
            echo=settings.DEBUG,
            future=True,
            poolclass=NullPool,
        )
    else:
        # PostgreSQL: Use connection pooling for production
        return create_async_engine(
            settings.DATABASE_URL,
            echo=settings.DEBUG,
            future=True,
            # Connection pooling settings
            pool_size=settings.DB_POOL_SIZE,  # Default: 20
            max_overflow=settings.DB_MAX_OVERFLOW,  # Default: 10
            pool_pre_ping=True,  # Verify connections before use
            pool_recycle=settings.DB_POOL_RECYCLE,  # Default: 3600s (1 hour)
            pool_timeout=settings.DB_POOL_TIMEOUT,  # Default: 30s
        )


engine = create_engine_with_pooling()

# Create session factory
AsyncSessionLocal = async_sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
    autocommit=False,
    autoflush=False,
)

# Base class for models
Base = declarative_base()


async def get_db() -> AsyncGenerator[AsyncSession, None]:
    """Dependency to get database session."""
    async with AsyncSessionLocal() as session:
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise
        finally:
            await session.close()


async def init_db(max_retries: int = 5, base_delay: float = 1.0) -> None:
    """Initialize database tables with retry logic.

    Args:
        max_retries: Maximum number of connection attempts
        base_delay: Initial delay between retries in seconds (doubles each retry)
    """
    last_exception: Exception | None = None

    for attempt in range(max_retries):
        try:
            async with engine.begin() as conn:
                await conn.run_sync(Base.metadata.create_all)
            return  # Success
        except Exception as e:
            last_exception = e
            if attempt < max_retries - 1:
                delay = base_delay * (2**attempt)  # Exponential backoff
                if settings.DEBUG:
                    print(
                        f"Database connection failed (attempt {attempt + 1}/{max_retries}), retrying in {delay}s..."
                    )
                await asyncio.sleep(delay)

    # All retries exhausted
    if last_exception is not None:
        raise last_exception
    raise RuntimeError("Database initialization failed")


async def close_db() -> None:
    """Close database connections."""
    await engine.dispose()
