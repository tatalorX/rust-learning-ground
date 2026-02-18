# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Audit logging service for security and compliance.
Tracks important events: logins, password changes, data access, etc.
"""
from datetime import datetime, timezone
from typing import Optional, Dict, Any
import json

from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, desc, func
from fastapi import Request

from app.models import AuditLog


class AuditEventType:
    """Audit event type constants."""
    LOGIN = "login"
    LOGIN_FAILED = "login_failed"
    LOGOUT = "logout"
    PASSWORD_CHANGE = "password_change"
    PASSWORD_RESET_REQUEST = "password_reset_request"
    PASSWORD_RESET_COMPLETE = "password_reset_complete"
    REGISTRATION = "registration"
    EMAIL_VERIFICATION = "email_verification"
    PROFILE_UPDATE = "profile_update"
    DATA_EXPORT = "data_export"
    DATA_DELETION = "data_deletion"
    ACCOUNT_DELETION = "account_deletion"
    ADMIN_ACTION = "admin_action"
    API_KEY_CREATED = "api_key_created"
    API_KEY_REVOKED = "api_key_revoked"
    SUSPICIOUS_ACTIVITY = "suspicious_activity"
    RATE_LIMIT_EXCEEDED = "rate_limit_exceeded"
    CLASSROOM_CREATED = "classroom_created"
    CLASSROOM_JOINED = "classroom_joined"
    ASSIGNMENT_CREATED = "assignment_created"
    ASSIGNMENT_SUBMITTED = "assignment_submitted"


class AuditStatus:
    """Audit event status constants."""
    SUCCESS = "success"
    FAILURE = "failure"
    WARNING = "warning"


def get_client_ip(request: Optional[Request]) -> Optional[str]:
    """Extract client IP from request."""
    if not request:
        return None
    
    # Check for forwarded IP (behind proxy)
    forwarded = request.headers.get("X-Forwarded-For")
    if forwarded:
        return forwarded.split(",")[0].strip()
    
    real_ip = request.headers.get("X-Real-IP")
    if real_ip:
        return real_ip
    
    # Fall back to direct connection
    if hasattr(request, "client") and request.client:
        return request.client.host
    
    return None


def sanitize_metadata(data: Dict[str, Any]) -> Dict[str, Any]:
    """Sanitize metadata to remove sensitive information."""
    sensitive_keys = {
        "password", "token", "secret", "credit_card", "ssn",
        "api_key", "access_token", "refresh_token", "hash"
    }
    
    sanitized = {}
    for key, value in data.items():
        # Skip sensitive keys
        if any(sensitive in key.lower() for sensitive in sensitive_keys):
            sanitized[key] = "[REDACTED]"
        elif isinstance(value, dict):
            sanitized[key] = sanitize_metadata(value)
        elif isinstance(value, (str, int, float, bool)) or value is None:
            sanitized[key] = value
        else:
            sanitized[key] = str(value)
    
    return sanitized


async def log_event(
    db: AsyncSession,
    event_type: str,
    event_status: str,
    user_id: Optional[int] = None,
    request: Optional[Request] = None,
    metadata: Optional[Dict[str, Any]] = None
) -> AuditLog:
    """
    Log an audit event.
    
    Args:
        db: Database session
        event_type: Type of event (use AuditEventType constants)
        event_status: Status (success/failure/warning)
        user_id: Optional user ID
        request: Optional FastAPI request for IP/UA extraction
        metadata: Optional additional context (will be sanitized)
    
    Returns:
        Created AuditLog entry
    """
    # Sanitize metadata
    safe_metadata = sanitize_metadata(metadata or {})
    
    audit_entry = AuditLog(
        user_id=user_id,
        event_type=event_type,
        event_status=event_status,
        ip_address=get_client_ip(request),
        user_agent=request.headers.get("User-Agent") if request else None,
        meta_data=safe_metadata
    )
    
    db.add(audit_entry)
    await db.flush()
    
    return audit_entry


async def get_user_audit_logs(
    db: AsyncSession,
    user_id: int,
    limit: int = 50,
    offset: int = 0
) -> list[AuditLog]:
    """Get audit logs for a specific user."""
    result = await db.execute(
        select(AuditLog)
        .where(AuditLog.user_id == user_id)
        .order_by(desc(AuditLog.created_at))
        .limit(limit)
        .offset(offset)
    )
    return list(result.scalars().all())


async def get_recent_audit_logs(
    db: AsyncSession,
    event_type: Optional[str] = None,
    event_status: Optional[str] = None,
    hours: int = 24,
    limit: int = 100
) -> list[AuditLog]:
    """Get recent audit logs with optional filtering."""
    since = datetime.now(timezone.utc).replace(hour=datetime.now().hour - hours)
    
    query = select(AuditLog).where(AuditLog.created_at >= since)
    
    if event_type:
        query = query.where(AuditLog.event_type == event_type)
    if event_status:
        query = query.where(AuditLog.event_status == event_status)
    
    result = await db.execute(
        query.order_by(desc(AuditLog.created_at)).limit(limit)
    )
    return list(result.scalars().all())


async def get_security_events(
    db: AsyncSession,
    hours: int = 24,
    limit: int = 100
) -> list[AuditLog]:
    """Get security-related events (failed logins, suspicious activity, etc.)."""
    security_events = [
        AuditEventType.LOGIN_FAILED,
        AuditEventType.SUSPICIOUS_ACTIVITY,
        AuditEventType.RATE_LIMIT_EXCEEDED,
        AuditEventType.PASSWORD_RESET_REQUEST,
    ]
    
    since = datetime.now(timezone.utc).replace(hour=datetime.now().hour - hours)
    
    result = await db.execute(
        select(AuditLog)
        .where(AuditLog.event_type.in_(security_events))
        .where(AuditLog.created_at >= since)
        .order_by(desc(AuditLog.created_at))
        .limit(limit)
    )
    return list(result.scalars().all())


async def count_failed_logins(
    db: AsyncSession,
    ip_address: str,
    minutes: int = 30
) -> int:
    """Count failed login attempts from an IP address."""
    since = datetime.now(timezone.utc)
    since = since.replace(minute=since.minute - minutes)
    
    result = await db.execute(
        select(func.count(AuditLog.id))
        .where(AuditLog.event_type == AuditEventType.LOGIN_FAILED)
        .where(AuditLog.ip_address == ip_address)
        .where(AuditLog.created_at >= since)
    )
    return result.scalar() or 0


async def detect_suspicious_activity(
    db: AsyncSession,
    user_id: Optional[int] = None,
    ip_address: Optional[str] = None
) -> bool:
    """
    Detect suspicious activity patterns.
    Returns True if suspicious activity is detected.
    """
    # Check for rapid failed logins
    if ip_address:
        failed_count = await count_failed_logins(db, ip_address, minutes=10)
        if failed_count >= 5:
            return True
    
    # Check for unusual patterns (could be extended with ML)
    # For now, just check if there are many events in short time
    
    return False


async def log_suspicious_activity(
    db: AsyncSession,
    reason: str,
    user_id: Optional[int] = None,
    request: Optional[Request] = None,
    metadata: Optional[Dict[str, Any]] = None
) -> AuditLog:
    """Log a suspicious activity event."""
    meta = metadata or {}
    meta["reason"] = reason
    
    return await log_event(
        db=db,
        event_type=AuditEventType.SUSPICIOUS_ACTIVITY,
        event_status=AuditStatus.WARNING,
        user_id=user_id,
        request=request,
        metadata=meta
    )
