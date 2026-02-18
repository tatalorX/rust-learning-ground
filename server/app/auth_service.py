# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Authentication service - business logic for user management.
"""
import hashlib
from datetime import datetime, timezone, timedelta
from typing import Optional, Tuple

from sqlalchemy import select, and_
from sqlalchemy.ext.asyncio import AsyncSession

from app.models import User, UserProgress, RefreshToken, AuditLog
from app.schemas import UserCreate, UserUpdate, ProgressResponse
from app.security import (
    hash_password, verify_password, create_access_token, 
    create_refresh_token, get_token_hash, generate_secure_token
)
from app.config import get_settings

settings = get_settings()


class AuthError(Exception):
    """Authentication error with message."""
    def __init__(self, message: str, code: str = "auth_error"):
        self.message = message
        self.code = code
        super().__init__(message)


def sanitize_for_log(value: Optional[str], max_len: int = 100) -> str:
    """Sanitize value for safe logging."""
    if value is None:
        return ""
    if not isinstance(value, str):
        value = str(value)
    # Remove control characters, limit length
    sanitized = ''.join(c for c in value if c.isprintable())[:max_len]
    return sanitized


def hash_email_for_log(email: str) -> str:
    """Hash email for safe logging (privacy protection)."""
    return hashlib.sha256(email.lower().encode()).hexdigest()[:16]


async def create_user(db: AsyncSession, user_data: UserCreate) -> User:
    """Create a new user account."""
    # Check if email already exists (case-insensitive)
    result = await db.execute(
        select(User).where(User.email.ilike(user_data.email))
    )
    if result.scalar_one_or_none():
        raise AuthError("An account with this email already exists", "email_exists")
    
    # Check if username already exists (case-insensitive)
    result = await db.execute(
        select(User).where(User.username.ilike(user_data.username))
    )
    if result.scalar_one_or_none():
        raise AuthError("This username is already taken", "username_exists")
    
    # Create user
    user = User(
        email=user_data.email.lower().strip(),
        username=user_data.username.lower().strip(),
        display_name=sanitize_for_log(user_data.display_name or user_data.username, 100),
        hashed_password=hash_password(user_data.password),
        password_changed_at=datetime.now(timezone.utc),
    )
    
    db.add(user)
    await db.flush()  # Get user.id
    
    # Create progress record
    progress = UserProgress(user_id=user.id)
    db.add(progress)
    
    await db.commit()
    return user


async def authenticate_user(db: AsyncSession, email: str, password: str) -> User:
    """
    Authenticate user with email and password.
    Uses constant-time comparison to prevent timing attacks.
    """
    # Normalize email
    email = email.lower().strip()
    
    # Get user (including inactive - we check later for consistent timing)
    result = await db.execute(
        select(User).where(User.email == email)
    )
    user = result.scalar_one_or_none()
    
    # TIMING ATTACK PREVENTION:
    # Always perform password verification, even if user not found
    # Use a dummy hash that takes similar time to real hash
    dummy_hash = (
        "$argon2id$v=19$m=65536,t=3,p=4$"
        "c29tZXNhbHRzb21lc2FsdHNvbWVzYWx0$"
        "c29tZWhhc2hzb21laGFzaHNvbWVoYXNoc29tZWhhc2g"
    )
    
    # Get hash to verify (real or dummy)
    hash_to_verify = user.hashed_password if (user and user.hashed_password) else dummy_hash
    
    # This takes constant time regardless of user existence
    password_valid = verify_password(password, hash_to_verify)
    
    # Now check all conditions (after constant-time password check)
    if not user or not password_valid:
        raise AuthError("Invalid email or password", "invalid_credentials")
    
    if not user.is_active:
        raise AuthError("Account is disabled", "account_disabled")
    
    # Check if account is locked
    if user.locked_until and user.locked_until > datetime.now(timezone.utc):
        raise AuthError(
            "Account is locked due to too many failed attempts. Please try again later.",
            "account_locked"
        )
    
    # Success - reset failed attempts and update last login
    user.failed_login_attempts = 0
    user.locked_until = None
    user.last_login = datetime.now(timezone.utc)
    await db.commit()
    
    return user


async def handle_failed_login(db: AsyncSession, email: str) -> None:
    """Record failed login attempt and potentially lock account."""
    email = email.lower().strip()
    
    result = await db.execute(
        select(User).where(User.email == email)
    )
    user = result.scalar_one_or_none()
    
    if user:
        user.failed_login_attempts += 1
        
        # Lock account after 5 failed attempts
        if user.failed_login_attempts >= 5:
            user.locked_until = datetime.now(timezone.utc) + timedelta(minutes=30)
        
        await db.commit()


async def create_user_tokens(
    db: AsyncSession, 
    user: User, 
    ip_address: Optional[str] = None,
    user_agent: Optional[str] = None
) -> Tuple[str, str]:
    """
    Create access and refresh tokens for a user.
    Returns: (access_token, refresh_token)
    """
    # Create access token
    access_token = create_access_token(user.id)
    
    # Create refresh token
    refresh_token, token_hash, expires_at = create_refresh_token(user.id)
    
    # Store refresh token in database
    db_token = RefreshToken(
        user_id=user.id,
        token_hash=token_hash,
        device_info=sanitize_for_log(user_agent, 500),
        ip_address=sanitize_for_log(ip_address, 45),
        expires_at=expires_at,
    )
    db.add(db_token)
    await db.commit()
    
    return access_token, refresh_token


async def refresh_access_token(
    db: AsyncSession,
    refresh_token: str,
    ip_address: Optional[str] = None
) -> Tuple[str, str, User]:
    """
    Refresh access token using refresh token (token rotation).
    Returns: (new_access_token, new_refresh_token, user)
    """
    from app.security import verify_refresh_token
    
    # Verify token signature and expiration
    user_id = verify_refresh_token(refresh_token)
    if not user_id:
        raise AuthError("Invalid refresh token", "invalid_token")
    
    # Check if token exists in database and is not revoked
    token_hash = get_token_hash(refresh_token)
    result = await db.execute(
        select(RefreshToken).where(
            and_(
                RefreshToken.token_hash == token_hash,
                RefreshToken.revoked_at.is_(None),
                RefreshToken.expires_at > datetime.now(timezone.utc)
            )
        )
    )
    db_token = result.scalar_one_or_none()
    
    if not db_token:
        raise AuthError("Invalid or expired refresh token", "invalid_token")
    
    # Get user
    result = await db.execute(
        select(User).where(and_(User.id == user_id, User.is_active == True))
    )
    user = result.scalar_one_or_none()
    
    if not user:
        raise AuthError("User not found", "user_not_found")
    
    # Revoke old token
    db_token.revoked_at = datetime.now(timezone.utc)
    
    # Create new tokens
    new_access_token = create_access_token(user.id)
    new_refresh_token, new_token_hash, new_expires_at = create_refresh_token(user.id)
    
    # Store new refresh token with rotation info
    new_db_token = RefreshToken(
        user_id=user.id,
        token_hash=new_token_hash,
        device_info=db_token.device_info,
        ip_address=sanitize_for_log(ip_address, 45),
        expires_at=new_expires_at,
        replaced_by=new_token_hash,
    )
    db.add(new_db_token)
    db_token.replaced_by = new_token_hash
    
    await db.commit()
    
    return new_access_token, new_refresh_token, user


async def revoke_refresh_token(db: AsyncSession, refresh_token: str) -> bool:
    """Revoke a refresh token (logout)."""
    token_hash = get_token_hash(refresh_token)
    result = await db.execute(
        select(RefreshToken).where(RefreshToken.token_hash == token_hash)
    )
    db_token = result.scalar_one_or_none()
    
    if db_token and not db_token.revoked_at:
        db_token.revoked_at = datetime.now(timezone.utc)
        await db.commit()
        return True
    return False


async def revoke_all_user_tokens(db: AsyncSession, user_id: int) -> int:
    """Revoke all refresh tokens for a user (logout all devices)."""
    result = await db.execute(
        select(RefreshToken).where(
            and_(
                RefreshToken.user_id == user_id,
                RefreshToken.revoked_at.is_(None),
                RefreshToken.expires_at > datetime.now(timezone.utc)
            )
        )
    )
    tokens = result.scalars().all()
    
    now = datetime.now(timezone.utc)
    for token in tokens:
        token.revoked_at = now
    
    await db.commit()
    return len(tokens)


async def get_or_create_github_user(
    db: AsyncSession,
    github_id: str,
    github_username: str,
    email: Optional[str],
    avatar_url: Optional[str] = None
) -> User:
    """Get existing user by GitHub ID or create new one."""
    # Try to find by GitHub ID
    result = await db.execute(
        select(User).where(User.github_id == github_id)
    )
    user = result.scalar_one_or_none()
    
    if user:
        # Update GitHub info
        user.github_username = sanitize_for_log(github_username, 100)
        user.github_avatar_url = sanitize_for_log(avatar_url, 500)
        user.last_login = datetime.now(timezone.utc)
        await db.commit()
        return user
    
    # Try to find by email if provided
    if email:
        email = email.lower().strip()
        result = await db.execute(
            select(User).where(User.email == email)
        )
        user = result.scalar_one_or_none()
        if user:
            # Link GitHub to existing account
            user.github_id = github_id
            user.github_username = sanitize_for_log(github_username, 100)
            user.github_avatar_url = sanitize_for_log(avatar_url, 500)
            user.last_login = datetime.now(timezone.utc)
            await db.commit()
            return user
    
    # Create new user from GitHub
    # Generate unique username
    base_username = github_username.lower()
    username = base_username
    counter = 1
    
    while True:
        result = await db.execute(
            select(User).where(User.username == username)
        )
        if not result.scalar_one_or_none():
            break
        username = f"{base_username}_{counter}"
        counter += 1
        if counter > 1000:  # Prevent infinite loop
            raise AuthError("Could not generate unique username", "username_generation_failed")
    
    # Generate placeholder email if not provided
    safe_email = email or f"{username}@github.local"
    
    user = User(
        email=safe_email,
        username=username,
        display_name=sanitize_for_log(github_username, 100),
        github_id=github_id,
        github_username=sanitize_for_log(github_username, 100),
        github_avatar_url=sanitize_for_log(avatar_url, 500),
        is_verified=True,  # GitHub users are pre-verified
        last_login=datetime.now(timezone.utc),
    )
    
    db.add(user)
    await db.flush()
    
    # Create progress record
    progress = UserProgress(user_id=user.id)
    db.add(progress)
    
    await db.commit()
    return user


async def get_user_progress(db: AsyncSession, user_id: int) -> UserProgress:
    """Get or create user progress."""
    result = await db.execute(
        select(UserProgress).where(UserProgress.user_id == user_id)
    )
    progress = result.scalar_one_or_none()
    
    if not progress:
        progress = UserProgress(user_id=user_id)
        db.add(progress)
        await db.commit()
    
    return progress


async def update_user_progress(
    db: AsyncSession,
    user_id: int,
    problem_id: int,
    xp_gained: int = 10,
    code: Optional[str] = None
) -> UserProgress:
    """Update user progress when solving a problem."""
    progress = await get_user_progress(db, user_id)
    
    # Add problem to solved list if not already there
    if problem_id not in progress.solved_problems:
        progress.solved_problems.append(problem_id)
        progress.xp += xp_gained
        
        # Update streak
        today = datetime.now(timezone.utc).date()
        if progress.last_solved_date:
            last_date = progress.last_solved_date.date() if hasattr(progress.last_solved_date, 'date') else progress.last_solved_date
            if last_date == today:
                pass  # Already solved today
            elif (today - last_date).days == 1:
                progress.current_streak += 1
                progress.longest_streak = max(progress.current_streak, progress.longest_streak)
            else:
                progress.current_streak = 1
        else:
            progress.current_streak = 1
        
        progress.last_solved_date = datetime.now(timezone.utc)
    
    # Save code if provided (with size limit)
    if code:
        if len(code) <= 50000:  # Max 50KB
            progress.saved_code[str(problem_id)] = code
    
    await db.commit()
    return progress


async def log_audit_event(
    db: AsyncSession,
    event_type: str,
    event_status: str,
    user_id: Optional[int] = None,
    ip_address: Optional[str] = None,
    user_agent: Optional[str] = None,
    metadata: Optional[dict] = None
):
    """Log a security audit event."""
    # Sanitize metadata to prevent log injection
    safe_metadata = {}
    if metadata:
        for key, value in metadata.items():
            if isinstance(value, str):
                safe_metadata[key] = sanitize_for_log(value, 200)
            else:
                safe_metadata[key] = value
    
    log = AuditLog(
        user_id=user_id,
        event_type=sanitize_for_log(event_type, 50),
        event_status=sanitize_for_log(event_status, 20),
        ip_address=sanitize_for_log(ip_address, 45),
        user_agent=sanitize_for_log(user_agent, 500),
        meta_data=safe_metadata,
    )
    db.add(log)
    await db.commit()
