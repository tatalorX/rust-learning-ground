# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Extended authentication routes including password reset and email verification.
"""
from datetime import datetime, timezone
from typing import Optional

from fastapi import APIRouter, Depends, HTTPException, status, Request, BackgroundTasks
from fastapi.security import OAuth2PasswordRequestForm
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

from app import schemas, auth_service
from app.database import get_db
from app.config import Settings, get_settings
from app.auth_deps import get_current_user
from app.models import User
from app.email_service import (
    EmailService, create_password_reset_token, validate_password_reset_token,
    mark_reset_token_used
)
from app.audit_service import log_event, AuditEventType, AuditStatus

router = APIRouter(prefix="/api/auth", tags=["authentication"])


@router.post("/forgot-password", status_code=status.HTTP_202_ACCEPTED)
async def forgot_password(
    request: Request,
    data: schemas.PasswordResetRequest,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
    settings: Settings = Depends(get_settings)
):
    """
    Request a password reset email.
    Always returns 202 to prevent email enumeration attacks.
    """
    # Find user by email
    result = await db.execute(select(User).where(User.email == data.email))
    user = result.scalar_one_or_none()
    
    if user:
        # Create reset token
        token = await create_password_reset_token(db, user)
        await db.commit()
        
        # Send email in background
        email_service = EmailService(settings)
        base_url = str(request.base_url).rstrip("/")
        
        background_tasks.add_task(
            email_service.send_password_reset,
            to_email=user.email,
            reset_token=token,
            base_url=base_url
        )
        
        # Log event (don't await, fire and forget)
        await log_event(
            db=db,
            event_type=AuditEventType.PASSWORD_RESET_REQUEST,
            event_status=AuditStatus.SUCCESS,
            user_id=user.id,
            request=request,
            metadata={"email": data.email[:3] + "***@***"}  # Partially masked
        )
        await db.commit()
    
    # Always return same response (security)
    return {
        "message": "If an account exists with this email, a password reset link has been sent."
    }


@router.post("/reset-password", status_code=status.HTTP_200_OK)
async def reset_password(
    request: Request,
    data: schemas.PasswordResetConfirm,
    db: AsyncSession = Depends(get_db),
    settings: Settings = Depends(get_settings)
):
    """Reset password using a valid reset token."""
    # Validate token
    user = await validate_password_reset_token(db, data.token)
    
    if not user:
        await log_event(
            db=db,
            event_type=AuditEventType.PASSWORD_RESET_COMPLETE,
            event_status=AuditStatus.FAILURE,
            request=request,
            metadata={"reason": "invalid_or_expired_token"}
        )
        await db.commit()
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Invalid or expired reset token"
        )
    
    # Update password
    user.hashed_password = auth_service.hash_password(data.new_password)
    user.password_changed_at = datetime.now(timezone.utc)
    user.failed_login_attempts = 0
    user.locked_until = None
    
    # Mark token as used
    await mark_reset_token_used(db, data.token)
    
    # Log event
    await log_event(
        db=db,
        event_type=AuditEventType.PASSWORD_RESET_COMPLETE,
        event_status=AuditStatus.SUCCESS,
        user_id=user.id,
        request=request
    )
    await db.commit()
    
    return {"message": "Password has been reset successfully. Please log in with your new password."}


@router.post("/change-password", status_code=status.HTTP_200_OK)
async def change_password(
    request: Request,
    data: schemas.PasswordChange,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Change password (requires current password)."""
    # Verify current password
    if not auth_service.verify_password(data.current_password, current_user.hashed_password):
        await log_event(
            db=db,
            event_type=AuditEventType.PASSWORD_CHANGE,
            event_status=AuditStatus.FAILURE,
            user_id=current_user.id,
            request=request,
            metadata={"reason": "incorrect_current_password"}
        )
        await db.commit()
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Current password is incorrect"
        )
    
    # Update password
    current_user.hashed_password = auth_service.hash_password(data.new_password)
    current_user.password_changed_at = datetime.now(timezone.utc)
    
    # Log event
    await log_event(
        db=db,
        event_type=AuditEventType.PASSWORD_CHANGE,
        event_status=AuditStatus.SUCCESS,
        user_id=current_user.id,
        request=request
    )
    await db.commit()
    
    return {"message": "Password changed successfully"}


@router.get("/verify-reset-token")
async def verify_reset_token(
    token: str,
    db: AsyncSession = Depends(get_db)
):
    """Verify if a password reset token is valid (for UI validation)."""
    user = await validate_password_reset_token(db, token)
    
    if not user:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Invalid or expired token"
        )
    
    return {
        "valid": True,
        "email": user.email[:3] + "***@***"  # Partially masked
    }


@router.post("/resend-verification", status_code=status.HTTP_202_ACCEPTED)
async def resend_verification(
    request: Request,
    background_tasks: BackgroundTasks,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db),
    settings: Settings = Depends(get_settings)
):
    """Resend email verification link."""
    if current_user.is_verified:
        return {"message": "Email is already verified"}
    
    # Generate verification token (simplified - in production use JWT or similar)
    import secrets
    token = secrets.token_urlsafe(32)
    
    # Store token (simplified - in production use dedicated table)
    # For now, we'll just log it
    
    # Send email
    email_service = EmailService(settings)
    base_url = str(request.base_url).rstrip("/")
    
    background_tasks.add_task(
        email_service.send_verification_email,
        to_email=current_user.email,
        verification_token=token,
        base_url=base_url
    )
    
    await log_event(
        db=db,
        event_type=AuditEventType.EMAIL_VERIFICATION,
        event_status=AuditStatus.SUCCESS,
        user_id=current_user.id,
        request=request
    )
    await db.commit()
    
    return {"message": "Verification email sent"}
