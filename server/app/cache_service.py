# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Caching service using Redis with fallback to in-memory.
Provides caching for frequently accessed data like leaderboards and user profiles.

SECURITY FIXES APPLIED:
- Removed pickle import (H-06)
- Changed MD5 to SHA-256 for cache keys (H-07)
"""

import json
from typing import Optional, Any, Union
from datetime import timedelta
import hashlib

try:
    import redis.asyncio as redis

    REDIS_AVAILABLE = True
except ImportError:
    REDIS_AVAILABLE = False

from app.config import Settings


class CacheService:
    """Caching service with Redis backend and in-memory fallback."""

    def __init__(self, settings: Settings):
        self.settings = settings
        self._redis: Optional[Any] = None
        self._memory_cache: dict = {}
        self._enabled = settings.CACHE_ENABLED

        if self._enabled and REDIS_AVAILABLE and settings.REDIS_URL:
            self._redis = redis.from_url(settings.REDIS_URL, decode_responses=True)

    async def connect(self):
        """Connect to Redis if configured."""
        if self._redis:
            try:
                await self._redis.ping()
            except Exception as e:
                print(f"Redis connection failed: {e}. Using memory cache.")
                self._redis = None

    def _make_key(self, key: str, namespace: str = "default") -> str:
        """Create a namespaced cache key."""
        return f"rust_learning:{namespace}:{key}"

    async def get(self, key: str, namespace: str = "default") -> Optional[Any]:
        """Get value from cache."""
        if not self._enabled:
            return None

        full_key = self._make_key(key, namespace)

        # Try Redis first
        if self._redis:
            try:
                value = await self._redis.get(full_key)
                if value:
                    return json.loads(value)
            except Exception:
                pass

        # Fallback to memory cache
        if full_key in self._memory_cache:
            return self._memory_cache[full_key]["value"]

        return None

    async def set(
        self,
        key: str,
        value: Any,
        namespace: str = "default",
        ttl: Optional[int] = None,
    ) -> bool:
        """Set value in cache with optional TTL."""
        if not self._enabled:
            return False

        full_key = self._make_key(key, namespace)

        # Try Redis first
        if self._redis:
            try:
                serialized = json.dumps(value, default=str)
                if ttl:
                    await self._redis.setex(full_key, ttl, serialized)
                else:
                    await self._redis.set(full_key, serialized)
                return True
            except Exception:
                pass

        # Fallback to memory cache
        import time

        self._memory_cache[full_key] = {
            "value": value,
            "expires": time.time() + ttl if ttl else None,
        }
        return True

    async def delete(self, key: str, namespace: str = "default") -> bool:
        """Delete value from cache."""
        if not self._enabled:
            return False

        full_key = self._make_key(key, namespace)

        # Try Redis first
        if self._redis:
            try:
                await self._redis.delete(full_key)
                return True
            except Exception:
                pass

        # Fallback to memory cache
        if full_key in self._memory_cache:
            del self._memory_cache[full_key]
            return True

        return False

    async def delete_pattern(self, pattern: str, namespace: str = "default") -> int:
        """Delete all keys matching pattern."""
        if not self._enabled:
            return 0

        full_pattern = self._make_key(pattern, namespace)
        count = 0

        # Try Redis first
        if self._redis:
            try:
                keys = await self._redis.keys(f"{full_pattern}*")
                if keys:
                    await self._redis.delete(*keys)
                    count = len(keys)
            except Exception:
                pass

        # Fallback to memory cache
        import time

        keys_to_delete = [
            k for k in self._memory_cache.keys() if k.startswith(full_pattern)
        ]
        for k in keys_to_delete:
            del self._memory_cache[k]
            count += 1

        return count

    async def increment(
        self, key: str, amount: int = 1, namespace: str = "default"
    ) -> int:
        """Increment a counter in cache."""
        if not self._enabled:
            return 0

        full_key = self._make_key(key, namespace)

        # Try Redis first
        if self._redis:
            try:
                return await self._redis.incrby(full_key, amount)
            except Exception:
                pass

        # Fallback to memory cache
        current = self._memory_cache.get(full_key, {}).get("value", 0)
        new_value = current + amount
        self._memory_cache[full_key] = {"value": new_value, "expires": None}
        return new_value

    async def exists(self, key: str, namespace: str = "default") -> bool:
        """Check if key exists in cache."""
        if not self._enabled:
            return False

        full_key = self._make_key(key, namespace)

        # Try Redis first
        if self._redis:
            try:
                return await self._redis.exists(full_key) > 0
            except Exception:
                pass

        # Fallback to memory cache
        return full_key in self._memory_cache

    async def clear_namespace(self, namespace: str = "default") -> int:
        """Clear all keys in a namespace."""
        return await self.delete_pattern("", namespace)

    async def close(self):
        """Close Redis connection."""
        if self._redis:
            await self._redis.close()


# Decorator for caching function results
from functools import wraps


def cached(
    namespace: str = "default", ttl: int = 300, key_builder: Optional[callable] = None
):
    """Decorator to cache function results."""

    def decorator(func):
        @wraps(func)
        async def async_wrapper(*args, **kwargs):
            # Build cache key
            if key_builder:
                cache_key = key_builder(*args, **kwargs)
            else:
                # Default key from function name and arguments
                key_parts = [func.__name__]
                for arg in args[1:]:  # Skip self/db
                    key_parts.append(str(arg))
                for k, v in sorted(kwargs.items()):
                    key_parts.append(f"{k}={v}")
                # SECURITY FIX: Use SHA-256 instead of MD5 (H-07)
                cache_key = hashlib.sha256("|".join(key_parts).encode()).hexdigest()

            # Try to get from cache
            from app.config import get_settings

            cache = CacheService(get_settings())

            cached_value = await cache.get(cache_key, namespace)
            if cached_value is not None:
                return cached_value

            # Call function
            result = await func(*args, **kwargs)

            # Store in cache
            await cache.set(cache_key, result, namespace, ttl)

            return result

        return async_wrapper

    return decorator


def invalidate_cache(namespace: str = "default", pattern: str = ""):
    """Decorator to invalidate cache after function execution."""

    def decorator(func):
        @wraps(func)
        async def async_wrapper(*args, **kwargs):
            result = await func(*args, **kwargs)

            # Invalidate cache
            from app.config import get_settings

            cache = CacheService(get_settings())
            await cache.delete_pattern(pattern, namespace)

            return result

        return async_wrapper

    return decorator
