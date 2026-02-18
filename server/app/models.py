# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Database models for user management and progress tracking.
"""

import datetime
from typing import Optional

from sqlalchemy import Column, Integer, String, DateTime, Boolean, Text, Index, JSON
from sqlalchemy.orm import relationship
from sqlalchemy.sql import func

from app.database import Base


class User(Base):
    """User account model with secure password storage."""

    __tablename__ = "users"

    id = Column(Integer, primary_key=True, index=True)

    # Authentication
    email = Column(String(255), unique=True, index=True, nullable=False)
    username = Column(String(50), unique=True, index=True, nullable=False)
    hashed_password = Column(String(255), nullable=True)  # NULL for OAuth-only users

    # GitHub OAuth
    github_id = Column(String(50), unique=True, nullable=True, index=True)
    github_username = Column(String(100), nullable=True)
    github_avatar_url = Column(String(500), nullable=True)

    # Profile
    display_name = Column(String(100), nullable=True)
    is_active = Column(Boolean, default=True, nullable=False)
    is_verified = Column(Boolean, default=False, nullable=False)
    is_admin = Column(Boolean, default=False, nullable=False)

    # Security
    failed_login_attempts = Column(Integer, default=0, nullable=False)
    locked_until = Column(DateTime(timezone=True), nullable=True)
    last_login = Column(DateTime(timezone=True), nullable=True)
    password_changed_at = Column(DateTime(timezone=True), nullable=True)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )

    # Indexes for performance
    __table_args__ = (
        Index("ix_users_active_email", "is_active", "email"),
        Index("ix_users_created_at", "created_at"),
    )

    def __repr__(self) -> str:
        return f"<User(id={self.id}, username={self.username}, email={self.email})>"

    # School profile relationship
    school_profile = relationship(
        "UserSchoolProfile", back_populates="user", uselist=False
    )

    # School profile relationship
    school_profile = relationship(
        "UserSchoolProfile", back_populates="user", uselist=False
    )


class UserProgress(Base):
    """User progress tracking - isolated per user."""

    __tablename__ = "user_progress"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, nullable=False, index=True)

    # Progress data
    xp = Column(Integer, default=0, nullable=False)
    solved_problems = Column(
        JSON, default=list, nullable=False
    )  # List of solved problem IDs
    current_streak = Column(Integer, default=0, nullable=False)
    longest_streak = Column(Integer, default=0, nullable=False)
    last_solved_date = Column(DateTime(timezone=True), nullable=True)

    # Problem-specific code storage (optional - for cloud sync)
    saved_code = Column(JSON, default=dict, nullable=False)  # {problem_id: code}

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )

    # Indexes
    __table_args__ = (Index("ix_progress_user_id", "user_id", unique=True),)

    def __repr__(self) -> str:
        return f"<UserProgress(user_id={self.user_id}, xp={self.xp})>"


class RefreshToken(Base):
    """Refresh token storage for JWT rotation."""

    __tablename__ = "refresh_tokens"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, nullable=False, index=True)

    # Token data (hashed for security)
    token_hash = Column(String(255), unique=True, nullable=False, index=True)

    # Device/location info for user's token management
    device_info = Column(String(500), nullable=True)
    ip_address = Column(String(45), nullable=True)  # IPv6 compatible

    # Expiration and revocation
    expires_at = Column(DateTime(timezone=True), nullable=False)
    revoked_at = Column(DateTime(timezone=True), nullable=True)
    replaced_by = Column(
        String(255), nullable=True
    )  # Token that replaced this one (rotation)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )

    # Indexes
    __table_args__ = (
        Index("ix_refresh_tokens_user_expires", "user_id", "expires_at"),
        Index("ix_refresh_tokens_active", "revoked_at", "expires_at"),
    )

    def __repr__(self) -> str:
        return f"<RefreshToken(user_id={self.user_id}, expires_at={self.expires_at})>"


class AuditLog(Base):
    """Security audit log for important events."""

    __tablename__ = "audit_logs"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, nullable=True, index=True)  # NULL for pre-auth events

    event_type = Column(
        String(50), nullable=False, index=True
    )  # login, logout, password_change, etc.
    event_status = Column(String(20), nullable=False)  # success, failure

    # Request info
    ip_address = Column(String(45), nullable=True)
    user_agent = Column(String(500), nullable=True)

    # Additional context (safe, non-sensitive data only)
    meta_data = Column(JSON, default=dict, nullable=False)

    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )

    # Indexes
    __table_args__ = (
        Index("ix_audit_logs_user_event", "user_id", "event_type"),
        Index("ix_audit_logs_created", "created_at"),
    )

    def __repr__(self) -> str:
        return f"<AuditLog(user_id={self.user_id}, event={self.event_type}, status={self.event_status})>"
