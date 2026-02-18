# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
GDPR Compliance endpoints for data export (Article 20) and erasure (Article 17).
"""
import json
from datetime import datetime, timezone
from typing import Any, Dict, List

from fastapi import APIRouter, Depends, HTTPException, status, Request, BackgroundTasks
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, delete, func

from app.database import get_db
from app.auth_deps import get_current_user
from app.models import User, UserProgress, RefreshToken, AuditLog
from app.models_school import (
    UserSchoolProfile, ClassroomMember, AssignmentSubmission,
    UserTitle, TreatTransaction, ActivityLog
)
from app.audit_service import log_event, AuditEventType, AuditStatus

router = APIRouter(prefix="/api/users/me", tags=["GDPR"])


async def collect_user_data(db: AsyncSession, user_id: int) -> Dict[str, Any]:
    """
    Collect all personal data for a user (GDPR Article 20 - Right to data portability).
    """
    # Get user
    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one()
    
    # Get progress
    result = await db.execute(select(UserProgress).where(UserProgress.user_id == user_id))
    progress = result.scalar_one_or_none()
    
    # Get school profile
    result = await db.execute(select(UserSchoolProfile).where(UserSchoolProfile.user_id == user_id))
    school_profile = result.scalar_one_or_none()
    
    # Get classroom memberships
    result = await db.execute(
        select(ClassroomMember).where(ClassroomMember.user_id == user_id)
    )
    memberships = result.scalars().all()
    
    # Get assignment submissions
    result = await db.execute(
        select(AssignmentSubmission).where(AssignmentSubmission.user_id == user_id)
    )
    submissions = result.scalars().all()
    
    # Get titles
    result = await db.execute(select(UserTitle).where(UserTitle.user_id == user_id))
    titles = result.scalars().all()
    
    # Get treat transactions
    result = await db.execute(
        select(TreatTransaction).where(TreatTransaction.user_id == user_id)
    )
    transactions = result.scalars().all()
    
    # Get activity logs
    result = await db.execute(
        select(ActivityLog).where(ActivityLog.user_id == user_id)
    )
    activities = result.scalars().all()
    
    # Compile export data
    export_data = {
        "export_metadata": {
            "exported_at": datetime.now(timezone.utc).isoformat(),
            "user_id": user_id,
            "format_version": "1.0",
            "regulation": "GDPR Article 20"
        },
        "user_profile": {
            "id": user.id,
            "username": user.username,
            "display_name": user.display_name,
            "email": user.email,
            "github_username": user.github_username,
            "github_avatar_url": user.github_avatar_url,
            "is_verified": user.is_verified,
            "is_active": user.is_active,
            "created_at": user.created_at.isoformat() if user.created_at else None,
            "last_login": user.last_login.isoformat() if user.last_login else None,
        },
        "learning_progress": {
            "xp": progress.xp if progress else 0,
            "solved_problems": progress.solved_problems if progress else [],
            "current_streak": progress.current_streak if progress else 0,
            "longest_streak": progress.longest_streak if progress else 0,
            "last_solved_date": progress.last_solved_date.isoformat() if progress and progress.last_solved_date else None,
        } if progress else None,
        "school_profile": {
            "role": school_profile.role.value if school_profile else None,
            "grade_level": school_profile.grade_level.value if school_profile and school_profile.grade_level else None,
            "total_xp": school_profile.total_xp if school_profile else 0,
            "current_rank": school_profile.current_rank.value if school_profile else None,
            "problems_solved": school_profile.problems_solved if school_profile else 0,
            "current_streak": school_profile.current_streak if school_profile else 0,
            "longest_streak": school_profile.longest_streak if school_profile else 0,
            "treats": school_profile.treats if school_profile else 0,
            "display_title": school_profile.display_title if school_profile else None,
        } if school_profile else None,
        "classroom_memberships": [
            {
                "classroom_id": m.classroom_id,
                "joined_at": m.joined_at.isoformat() if m.joined_at else None,
                "problems_solved_in_class": m.problems_solved_in_class,
                "total_xp_earned": m.total_xp_earned,
                "is_active": m.is_active
            }
            for m in memberships
        ],
        "assignment_submissions": [
            {
                "id": s.id,
                "assignment_id": s.assignment_id,
                "status": s.status,
                "completed_exercises": s.completed_exercises,
                "score": s.score,
                "feedback": s.feedback,
                "xp_earned": s.xp_earned,
                "treats_earned": s.treats_earned,
                "started_at": s.started_at.isoformat() if s.started_at else None,
                "submitted_at": s.submitted_at.isoformat() if s.submitted_at else None,
                "graded_at": s.graded_at.isoformat() if s.graded_at else None
            }
            for s in submissions
        ],
        "earned_titles": [
            {
                "title_id": t.title_id,
                "equipped": t.equipped,
                "earned_at": t.earned_at.isoformat() if t.earned_at else None
            }
            for t in titles
        ],
        "treat_transactions": [
            {
                "id": t.id,
                "amount": t.amount,
                "balance_after": t.balance_after,
                "transaction_type": t.transaction_type,
                "description": t.description,
                "created_at": t.created_at.isoformat() if t.created_at else None
            }
            for t in transactions
        ],
        "activity_history": [
            {
                "id": a.id,
                "action": a.action,
                "target_type": a.target_type,
                "target_id": a.target_id,
                "classroom_id": a.classroom_id,
                "details": a.details,
                "created_at": a.created_at.isoformat() if a.created_at else None
            }
            for a in activities
        ]
    }
    
    return export_data


@router.get("/export", response_model=Dict[str, Any])
async def export_user_data(
    request: Request,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """
    Export all personal data for the current user (GDPR Article 20).
    
    Returns a JSON object containing all user data including:
    - Profile information
    - Learning progress
    - Classroom memberships
    - Assignment submissions
    - Activity history
    """
    export_data = await collect_user_data(db, current_user.id)
    
    # Log the export
    await log_event(
        db=db,
        event_type=AuditEventType.DATA_EXPORT,
        event_status=AuditStatus.SUCCESS,
        user_id=current_user.id,
        request=request,
        metadata={"export_format": "json"}
    )
    await db.commit()
    
    return export_data


@router.delete("", status_code=status.HTTP_202_ACCEPTED)
async def delete_user_account(
    request: Request,
    background_tasks: BackgroundTasks,
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """
    Delete user account and all associated personal data (GDPR Article 17 - Right to erasure).
    
    This will:
    - Delete or anonymize all personal data
    - Remove user from all classrooms
    - Delete all submissions and progress
    - Invalidate all sessions
    
    Note: Some data may be retained in anonymized form for analytics.
    """
    user_id = current_user.id
    
    # Log the deletion request before deleting
    await log_event(
        db=db,
        event_type=AuditEventType.ACCOUNT_DELETION,
        event_status=AuditStatus.SUCCESS,
        user_id=user_id,
        request=request,
        metadata={"username": current_user.username}
    )
    
    # Delete refresh tokens (logout from all devices)
    await db.execute(
        delete(RefreshToken).where(RefreshToken.user_id == user_id)
    )
    
    # Delete treat transactions
    await db.execute(
        delete(TreatTransaction).where(TreatTransaction.user_id == user_id)
    )
    
    # Delete user titles
    await db.execute(
        delete(UserTitle).where(UserTitle.user_id == user_id)
    )
    
    # Delete assignment submissions
    await db.execute(
        delete(AssignmentSubmission).where(AssignmentSubmission.user_id == user_id)
    )
    
    # Delete classroom memberships
    await db.execute(
        delete(ClassroomMember).where(ClassroomMember.user_id == user_id)
    )
    
    # Delete school profile
    await db.execute(
        delete(UserSchoolProfile).where(UserSchoolProfile.user_id == user_id)
    )
    
    # Delete progress
    await db.execute(
        delete(UserProgress).where(UserProgress.user_id == user_id)
    )
    
    # Anonymize user record (keep for referential integrity in logs/classrooms)
    # Set all PII to NULL/anonymized values
    current_user.email = f"deleted_{user_id}@anonymized.local"
    current_user.username = f"deleted_user_{user_id}"
    current_user.display_name = "Deleted User"
    current_user.hashed_password = "deleted"  # Invalid hash
    current_user.github_id = None
    current_user.github_username = None
    current_user.github_avatar_url = None
    current_user.is_active = False
    current_user.is_verified = False
    
    await db.commit()
    
    return {
        "message": "Account deletion initiated. All personal data will be removed within 30 days.",
        "deleted_at": datetime.now(timezone.utc).isoformat(),
        "user_id": user_id
    }


@router.get("/privacy-summary")
async def privacy_summary(
    current_user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """
    Get a summary of data stored about the current user.
    """
    user_id = current_user.id
    
    # Count various data types
    result = await db.execute(
        select(UserProgress).where(UserProgress.user_id == user_id)
    )
    progress = result.scalar_one_or_none()
    
    result = await db.execute(
        select(UserSchoolProfile).where(UserSchoolProfile.user_id == user_id)
    )
    school_profile = result.scalar_one_or_none()
    
    result = await db.execute(
        select(func.count(ClassroomMember.id)).where(ClassroomMember.user_id == user_id)
    )
    classroom_count = result.scalar() or 0
    
    result = await db.execute(
        select(func.count(AssignmentSubmission.id)).where(AssignmentSubmission.user_id == user_id)
    )
    submission_count = result.scalar() or 0
    
    result = await db.execute(
        select(func.count(ActivityLog.id)).where(ActivityLog.user_id == user_id)
    )
    activity_count = result.scalar() or 0
    
    return {
        "user_id": user_id,
        "username": current_user.username,
        "data_summary": {
            "profile": True,
            "learning_progress": progress is not None,
            "school_profile": school_profile is not None,
            "classroom_memberships": classroom_count,
            "assignment_submissions": submission_count,
            "activity_logs": activity_count,
        },
        "data_retention": {
            "active_account": "Until deletion requested",
            "activity_logs": "90 days after account deletion",
            "audit_logs": "1 year (legal requirement)"
        },
        "rights": [
            "export - Download all your data",
            "deletion - Request account deletion",
            "access - View what data we store"
        ]
    }
