# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Pydantic schemas for request/response validation.
"""
import re
from datetime import datetime
from typing import Optional, List

from pydantic import BaseModel, Field, field_validator, EmailStr, ConfigDict

from app.config import get_settings

settings = get_settings()


# ============== Base Schemas ==============

class BaseResponse(BaseModel):
    """Base response schema."""
    success: bool = True
    message: Optional[str] = None


# ============== User Schemas ==============

class UserBase(BaseModel):
    """Base user schema with common fields."""
    username: str = Field(..., min_length=3, max_length=50)
    email: EmailStr
    display_name: Optional[str] = Field(None, max_length=100)


class UserCreate(UserBase):
    """Schema for user registration."""
    password: str = Field(..., min_length=settings.PASSWORD_MIN_LENGTH, max_length=settings.PASSWORD_MAX_LENGTH)
    
    @field_validator('password')
    @classmethod
    def validate_password_strength(cls, v: str) -> str:
        """Validate password meets security requirements."""
        if len(v) < settings.PASSWORD_MIN_LENGTH:
            raise ValueError(f'Password must be at least {settings.PASSWORD_MIN_LENGTH} characters')
        if len(v) > settings.PASSWORD_MAX_LENGTH:
            raise ValueError(f'Password must not exceed {settings.PASSWORD_MAX_LENGTH} characters')
        
        # Check for at least one uppercase, one lowercase, and one digit
        if not re.search(r'[A-Z]', v):
            raise ValueError('Password must contain at least one uppercase letter')
        if not re.search(r'[a-z]', v):
            raise ValueError('Password must contain at least one lowercase letter')
        if not re.search(r'\d', v):
            raise ValueError('Password must contain at least one digit')
        
        return v
    
    @field_validator('username')
    @classmethod
    def validate_username(cls, v: str) -> str:
        """Validate username format."""
        if not re.match(r'^[a-zA-Z0-9_]+$', v):
            raise ValueError('Username can only contain letters, numbers, and underscores')
        if v[0].isdigit():
            raise ValueError('Username cannot start with a number')
        return v.lower()
    
    @field_validator('display_name')
    @classmethod
    def validate_display_name(cls, v: Optional[str]) -> Optional[str]:
        """Sanitize display name."""
        if v is None:
            return v
        # Remove control characters
        v = ''.join(c for c in v if c.isprintable())
        return v[:100]


class UserLogin(BaseModel):
    """Schema for user login."""
    email: EmailStr
    password: str = Field(..., min_length=1)


class UserUpdate(BaseModel):
    """Schema for user profile updates."""
    display_name: Optional[str] = Field(None, max_length=100)
    
    @field_validator('display_name')
    @classmethod
    def validate_display_name(cls, v: Optional[str]) -> Optional[str]:
        """Sanitize display name."""
        if v is None:
            return v
        v = ''.join(c for c in v if c.isprintable())
        return v[:100]


class UserResponse(UserBase):
    """Schema for user data in responses."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    is_active: bool
    is_verified: bool
    created_at: datetime
    last_login: Optional[datetime] = None
    github_username: Optional[str] = None
    github_avatar_url: Optional[str] = None


class UserProfileResponse(UserResponse):
    """Extended user profile with progress."""
    progress: "ProgressResponse"


# ============== Progress Schemas ==============

class ProgressResponse(BaseModel):
    """Schema for user progress."""
    model_config = ConfigDict(from_attributes=True)
    
    xp: int = 0
    solved_problems: List[int] = []
    current_streak: int = 0
    longest_streak: int = 0
    last_solved_date: Optional[datetime] = None


class ProblemSubmission(BaseModel):
    """Schema for submitting a solved problem."""
    problem_id: int = Field(..., ge=1, le=170)
    code: Optional[str] = Field(None, max_length=50000)
    
    @field_validator('code')
    @classmethod
    def validate_code_safety(cls, v: Optional[str]) -> Optional[str]:
        """Basic validation to prevent obviously malicious code."""
        if not v:
            return v
        
        # Check for null bytes
        if '\x00' in v:
            raise ValueError('Null bytes not allowed in code')
        
        # Check for dangerous patterns
        dangerous_patterns = [
            (r'process::Command', "Process execution"),
            (r'std::process::', "Process execution"),
            (r'Command::new', "Process execution"),
            (r'unsafe\s*\{', "Unsafe block"),
            (r'include_str!\s*\(\s*["\']/(etc|proc|sys|home|root|var)', "File inclusion"),
            (r'include_bytes!\s*\(\s*["\']/(etc|proc|sys|home|root|var)', "File inclusion"),
        ]
        
        for pattern, description in dangerous_patterns:
            if re.search(pattern, v, re.IGNORECASE):
                raise ValueError(f'Code contains potentially unsafe pattern: {description}')
        
        return v


# ============== Auth Schemas ==============

class TokenResponse(BaseResponse):
    """Schema for token response."""
    access_token: str
    token_type: str = "bearer"
    expires_in: int  # seconds
    user: UserResponse


class PasswordChange(BaseModel):
    """Schema for password change."""
    current_password: str
    new_password: str = Field(..., min_length=settings.PASSWORD_MIN_LENGTH, max_length=settings.PASSWORD_MAX_LENGTH)
    
    @field_validator('new_password')
    @classmethod
    def validate_new_password(cls, v: str) -> str:
        """Validate password meets security requirements."""
        if len(v) < settings.PASSWORD_MIN_LENGTH:
            raise ValueError(f'Password must be at least {settings.PASSWORD_MIN_LENGTH} characters')
        if not re.search(r'[A-Z]', v):
            raise ValueError('Password must contain at least one uppercase letter')
        if not re.search(r'[a-z]', v):
            raise ValueError('Password must contain at least one lowercase letter')
        if not re.search(r'\d', v):
            raise ValueError('Password must contain at least one digit')
        return v


class PasswordResetRequest(BaseModel):
    """Schema for password reset request."""
    email: EmailStr


class PasswordResetConfirm(BaseModel):
    """Schema for password reset confirmation."""
    token: str
    new_password: str = Field(..., min_length=8, max_length=128)


class PasswordChange(BaseModel):
    """Schema for password change."""
    current_password: str
    new_password: str = Field(..., min_length=8, max_length=128)


# ============== GitHub OAuth Schemas ==============

class GitHubAuthUrl(BaseResponse):
    """Schema for GitHub OAuth URL."""
    auth_url: str


class GitHubCallback(BaseModel):
    """Schema for GitHub OAuth callback."""
    code: str
    state: Optional[str] = None


# ============== Error Schemas ==============

class ErrorResponse(BaseResponse):
    """Schema for error responses."""
    success: bool = False
    error_code: Optional[str] = None
    details: Optional[dict] = None


# ============== Stats Schemas ==============

class StatsResponse(BaseResponse):
    """Schema for user stats."""
    total_users: int
    total_solved: int
    top_users: List[dict]


# Update forward references
UserProfileResponse.model_rebuild()
