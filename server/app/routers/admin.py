# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Admin panel API routes for platform management.

SECURITY FIXES APPLIED:
- SQL injection in search fixed by escaping wildcards (C-05)
"""

from datetime import datetime, timezone, timedelta
from typing import List, Optional

from fastapi import APIRouter, Depends, HTTPException, status, Query
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, func, desc, text

from app.database import get_db
from app.auth import get_current_user
from app.auth_deps import require_admin
from app.models import User, AuditLog, UserProgress
from app.models_school import (
    UserSchoolProfile,
    Classroom,
    Assignment,
    ActivityLog,
    UserRole,
    RankTier,
)
from app import schemas_school as schemas
from app.audit_service import get_recent_audit_logs, get_security_events

router = APIRouter(prefix="/api/admin", tags=["Admin"])


# ============================================================================
# DASHBOARD
# ============================================================================


@router.get("/dashboard")
async def get_admin_dashboard(
    db: AsyncSession = Depends(get_db), admin: User = Depends(require_admin)
):
    """Get admin dashboard statistics."""
    # User counts
    total_users = await db.scalar(select(func.count(User.id)))
    active_users = await db.scalar(
        select(func.count(User.id)).where(User.is_active == True)
    )
    new_users_24h = await db.scalar(
        select(func.count(User.id)).where(
            User.created_at >= datetime.now(timezone.utc) - timedelta(hours=24)
        )
    )

    # School platform stats
    total_students = await db.scalar(
        select(func.count(UserSchoolProfile.id)).where(
            UserSchoolProfile.role == UserRole.STUDENT
        )
    )
    total_teachers = await db.scalar(
        select(func.count(UserSchoolProfile.id)).where(
            UserSchoolProfile.role == UserRole.TEACHER
        )
    )
    total_classrooms = await db.scalar(select(func.count(Classroom.id)))
    active_classrooms = await db.scalar(
        select(func.count(Classroom.id)).where(Classroom.is_active == True)
    )

    # Assignment stats
    total_assignments = await db.scalar(select(func.count(Assignment.id)))

    # Activity stats
    logins_24h = await db.scalar(
        select(func.count(AuditLog.id)).where(
            AuditLog.event_type == "login",
            AuditLog.event_status == "success",
            AuditLog.created_at >= datetime.now(timezone.utc) - timedelta(hours=24),
        )
    )

    return {
        "users": {
            "total": total_users,
            "active": active_users,
            "new_24h": new_users_24h,
        },
        "school": {
            "students": total_students,
            "teachers": total_teachers,
            "classrooms": {"total": total_classrooms, "active": active_classrooms},
            "assignments": total_assignments,
        },
        "activity": {"logins_24h": logins_24h},
    }


# ============================================================================
# USER MANAGEMENT
# ============================================================================


@router.get("/users")
async def list_users(
    search: Optional[str] = None,
    is_active: Optional[bool] = None,
    is_verified: Optional[bool] = None,
    role: Optional[str] = None,
    limit: int = Query(50, ge=1, le=100),
    offset: int = Query(0, ge=0),
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """List all users with filtering."""
    query = select(User)

    if search:
        # SECURITY FIX: Escape SQL wildcards to prevent injection (C-05)
        safe_search = search.replace("%", r"\%").replace("_", r"\_")
        query = query.where(
            (User.username.ilike(f"%{safe_search}%", escape="\\"))
            | (User.email.ilike(f"%{safe_search}%", escape="\\"))
            | (User.display_name.ilike(f"%{safe_search}%", escape="\\"))
        )

    if is_active is not None:
        query = query.where(User.is_active == is_active)

    if is_verified is not None:
        query = query.where(User.is_verified == is_verified)

    # Get total count
    total = await db.scalar(select(func.count()).select_from(query.subquery()))

    # Get paginated results
    result = await db.execute(
        query.order_by(desc(User.created_at)).offset(offset).limit(limit)
    )
    users = result.scalars().all()

    return {
        "total": total,
        "offset": offset,
        "limit": limit,
        "users": [
            {
                "id": u.id,
                "username": u.username,
                "email": u.email,
                "display_name": u.display_name,
                "is_active": u.is_active,
                "is_verified": u.is_verified,
                "is_admin": u.is_admin,
                "created_at": u.created_at.isoformat() if u.created_at else None,
                "last_login": u.last_login.isoformat() if u.last_login else None,
                "github_username": u.github_username,
            }
            for u in users
        ],
    }


@router.get("/users/{user_id}")
async def get_user_details(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Get detailed user information."""
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # Get school profile
    result = await db.execute(
        select(UserSchoolProfile).where(UserSchoolProfile.user_id == user_id)
    )
    profile = result.scalar_one_or_none()

    # Get progress
    result = await db.execute(
        select(UserProgress).where(UserProgress.user_id == user_id)
    )
    progress = result.scalar_one_or_none()

    # Get recent audit logs
    audit_logs = await get_recent_audit_logs(db, hours=168, limit=20)
    user_logs = [log for log in audit_logs if log.user_id == user_id]

    return {
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "display_name": user.display_name,
            "is_active": user.is_active,
            "is_verified": user.is_verified,
            "is_admin": user.is_admin,
            "created_at": user.created_at.isoformat() if user.created_at else None,
            "last_login": user.last_login.isoformat() if user.last_login else None,
            "failed_login_attempts": user.failed_login_attempts,
            "locked_until": user.locked_until.isoformat()
            if user.locked_until
            else None,
            "github_username": user.github_username,
            "github_avatar_url": user.github_avatar_url,
        },
        "school_profile": {
            "role": profile.role.value if profile else None,
            "grade_level": profile.grade_level.value
            if profile and profile.grade_level
            else None,
            "total_xp": profile.total_xp if profile else 0,
            "current_rank": profile.current_rank.value if profile else None,
            "treats": profile.treats if profile else 0,
            "current_streak": profile.current_streak if profile else 0,
            "longest_streak": profile.longest_streak if profile else 0,
            "problems_solved": profile.problems_solved if profile else 0,
        }
        if profile
        else None,
        "progress": {
            "xp": progress.xp if progress else 0,
            "solved_problems_count": len(progress.solved_problems) if progress else 0,
            "current_streak": progress.current_streak if progress else 0,
            "longest_streak": progress.longest_streak if progress else 0,
        }
        if progress
        else None,
        "recent_activity": [
            {
                "event_type": log.event_type,
                "event_status": log.event_status,
                "created_at": log.created_at.isoformat() if log.created_at else None,
                "ip_address": log.ip_address,
            }
            for log in user_logs
        ],
    }


@router.post("/users/{user_id}/activate")
async def activate_user(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Activate a user account."""
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    user.is_active = True
    await db.flush()

    return {"message": "User activated successfully"}


@router.post("/users/{user_id}/deactivate")
async def deactivate_user(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Deactivate a user account."""
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # Prevent deactivating yourself
    if user.id == admin.id:
        raise HTTPException(
            status_code=400, detail="Cannot deactivate your own account"
        )

    user.is_active = False
    await db.flush()

    return {"message": "User deactivated successfully"}


@router.post("/users/{user_id}/make-admin")
async def make_user_admin(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Grant admin privileges to a user."""
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    user.is_admin = True
    await db.flush()

    return {"message": "User is now an admin"}


@router.post("/users/{user_id}/remove-admin")
async def remove_user_admin(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Remove admin privileges from a user."""
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()

    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # Prevent removing your own admin status
    if user.id == admin.id:
        raise HTTPException(
            status_code=400, detail="Cannot remove your own admin privileges"
        )

    user.is_admin = False
    await db.flush()

    return {"message": "Admin privileges removed"}


# ============================================================================
# CLASSROOM MANAGEMENT
# ============================================================================


@router.get("/classrooms")
async def list_all_classrooms(
    search: Optional[str] = None,
    is_active: Optional[bool] = None,
    limit: int = Query(50, ge=1, le=100),
    offset: int = Query(0, ge=0),
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """List all classrooms with filtering."""
    query = select(Classroom)

    if search:
        # SECURITY FIX: Escape SQL wildcards to prevent injection (C-05)
        safe_search = search.replace("%", r"\%").replace("_", r"\_")
        query = query.where(
            (Classroom.name.ilike(f"%{safe_search}%", escape="\\"))
            | (Classroom.subject.ilike(f"%{safe_search}%", escape="\\"))
        )

    if is_active is not None:
        query = query.where(Classroom.is_active == is_active)

    total = await db.scalar(select(func.count()).select_from(query.subquery()))

    result = await db.execute(
        query.order_by(desc(Classroom.created_at)).offset(offset).limit(limit)
    )
    classrooms = result.scalars().all()

    # Get member counts
    classroom_data = []
    for c in classrooms:
        member_count = await db.scalar(
            select(func.count(ClassroomMember.id)).where(
                ClassroomMember.classroom_id == c.id
            )
        )
        classroom_data.append(
            {
                "id": c.id,
                "name": c.name,
                "subject": c.subject,
                "code": c.code,
                "teacher_id": c.teacher_id,
                "grade_level": c.grade_level.value if c.grade_level else None,
                "is_active": c.is_active,
                "is_archived": c.is_archived,
                "member_count": member_count,
                "max_students": c.max_students,
                "created_at": c.created_at.isoformat() if c.created_at else None,
            }
        )

    return {
        "total": total,
        "offset": offset,
        "limit": limit,
        "classrooms": classroom_data,
    }


@router.post("/classrooms/{classroom_id}/archive")
async def admin_archive_classroom(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Archive a classroom."""
    from app import crud_school as crud

    success = await crud.archive_classroom(db, classroom_id)
    if not success:
        raise HTTPException(status_code=404, detail="Classroom not found")

    return {"message": "Classroom archived successfully"}


# ============================================================================
# AUDIT LOGS
# ============================================================================


@router.get("/audit-logs")
async def get_audit_logs(
    event_type: Optional[str] = None,
    event_status: Optional[str] = None,
    user_id: Optional[int] = None,
    hours: int = Query(24, ge=1, le=720),
    limit: int = Query(100, ge=1, le=500),
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Get audit logs with filtering."""
    query = select(AuditLog)

    if event_type:
        query = query.where(AuditLog.event_type == event_type)
    if event_status:
        query = query.where(AuditLog.event_status == event_status)
    if user_id:
        query = query.where(AuditLog.user_id == user_id)

    since = datetime.now(timezone.utc) - timedelta(hours=hours)
    query = query.where(AuditLog.created_at >= since)

    total = await db.scalar(select(func.count()).select_from(query.subquery()))

    result = await db.execute(query.order_by(desc(AuditLog.created_at)).limit(limit))
    logs = result.scalars().all()

    return {
        "total": total,
        "hours": hours,
        "logs": [
            {
                "id": log.id,
                "user_id": log.user_id,
                "event_type": log.event_type,
                "event_status": log.event_status,
                "ip_address": log.ip_address,
                "user_agent": log.user_agent,
                "meta_data": log.meta_data,
                "created_at": log.created_at.isoformat() if log.created_at else None,
            }
            for log in logs
        ],
    }


@router.get("/security-events")
async def get_security_events_endpoint(
    hours: int = Query(24, ge=1, le=168),
    db: AsyncSession = Depends(get_db),
    admin: User = Depends(require_admin),
):
    """Get security-related events."""
    events = await get_security_events(db, hours=hours)

    return {
        "count": len(events),
        "hours": hours,
        "events": [
            {
                "id": event.id,
                "user_id": event.user_id,
                "event_type": event.event_type,
                "event_status": event.event_status,
                "ip_address": event.ip_address,
                "created_at": event.created_at.isoformat()
                if event.created_at
                else None,
            }
            for event in events
        ],
    }


# ============================================================================
# SYSTEM
# ============================================================================


@router.post("/system/clear-cache")
async def clear_system_cache(admin: User = Depends(require_admin)):
    """Clear application cache."""
    from app.cache_service import cache_service

    # Clear all namespaces
    await cache_service.clear_namespace("default")
    await cache_service.clear_namespace("leaderboard")
    await cache_service.clear_namespace("profile")

    return {"message": "Cache cleared successfully"}


@router.get("/system/stats")
async def get_system_stats(
    db: AsyncSession = Depends(get_db), admin: User = Depends(require_admin)
):
    """Get system statistics."""
    from app.websocket_manager import manager

    # Database stats
    user_count = await db.scalar(select(func.count(User.id)))
    classroom_count = await db.scalar(select(func.count(Classroom.id)))

    # WebSocket stats
    ws_stats = {
        "connected_users": manager.get_user_count(),
        "total_connections": manager.get_total_connections(),
    }

    return {
        "database": {"users": user_count, "classrooms": classroom_count},
        "websockets": ws_stats,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


# Import ClassroomMember for the query
from app.models_school import ClassroomMember
