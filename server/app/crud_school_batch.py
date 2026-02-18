# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Batch CRUD operations for School Platform to fix N+1 query issues.
These functions use joined/subquery loading to fetch related data efficiently.
"""
from typing import List, Dict, Any, Optional
from sqlalchemy import select, func, and_
from sqlalchemy.ext.asyncio import AsyncSession

from app.models_school import (
    Classroom, ClassroomMember, Assignment, AssignmentSubmission, UserSchoolProfile
)
from app.models import User


# ============================================================================
# BATCH CLASSROOM OPERATIONS
# ============================================================================

async def get_classrooms_with_member_counts(
    db: AsyncSession, 
    classroom_ids: List[int]
) -> Dict[int, int]:
    """
    Get member counts for multiple classrooms in a single query.
    Returns: {classroom_id: member_count}
    """
    if not classroom_ids:
        return {}
    
    result = await db.execute(
        select(
            ClassroomMember.classroom_id,
            func.count(ClassroomMember.id).label("member_count")
        )
        .where(
            and_(
                ClassroomMember.classroom_id.in_(classroom_ids),
                ClassroomMember.is_active == True
            )
        )
        .group_by(ClassroomMember.classroom_id)
    )
    
    return {row[0]: row[1] for row in result.all()}


async def get_teacher_classrooms_batch(
    db: AsyncSession, 
    teacher_id: int,
    include_archived: bool = False
) -> List[Dict[str, Any]]:
    """
    Get all classrooms for a teacher with member counts in batch.
    Fixes N+1 query issue by fetching all data in 2 queries instead of N+1.
    """
    # Query 1: Get all classrooms
    result = await db.execute(
        select(Classroom)
        .where(Classroom.teacher_id == teacher_id)
        .where(Classroom.is_archived == False if not include_archived else True)
        .order_by(Classroom.created_at.desc())
    )
    classrooms = list(result.scalars().all())
    
    if not classrooms:
        return []
    
    # Query 2: Get all member counts in batch
    classroom_ids = [c.id for c in classrooms]
    member_counts = await get_classrooms_with_member_counts(db, classroom_ids)
    
    # Combine results
    return [
        {
            "classroom": classroom,
            "member_count": member_counts.get(classroom.id, 0)
        }
        for classroom in classrooms
    ]


async def get_student_classrooms_batch(
    db: AsyncSession, 
    student_id: int
) -> List[Dict[str, Any]]:
    """
    Get all classrooms for a student with member counts in batch.
    """
    # Query 1: Get all classrooms with membership info
    result = await db.execute(
        select(Classroom, ClassroomMember)
        .join(ClassroomMember, Classroom.id == ClassroomMember.classroom_id)
        .where(ClassroomMember.user_id == student_id)
        .where(ClassroomMember.is_active == True)
        .where(Classroom.is_active == True)
        .order_by(ClassroomMember.joined_at.desc())
    )
    
    rows = result.all()
    if not rows:
        return []
    
    classrooms = []
    classroom_ids = []
    for classroom, member in rows:
        classrooms.append({
            "classroom": classroom,
            "joined_at": member.joined_at,
            "problems_solved": member.problems_solved_in_class,
            "xp_earned": member.total_xp_earned
        })
        classroom_ids.append(classroom.id)
    
    # Query 2: Get all member counts in batch
    member_counts = await get_classrooms_with_member_counts(db, classroom_ids)
    
    # Add member counts
    for item in classrooms:
        item["member_count"] = member_counts.get(item["classroom"].id, 0)
    
    return classrooms


# ============================================================================
# BATCH ASSIGNMENT OPERATIONS
# ============================================================================

async def get_assignments_with_stats_batch(
    db: AsyncSession,
    classroom_id: int,
    include_unpublished: bool = False
) -> List[Dict[str, Any]]:
    """
    Get all assignments for a classroom with completion stats in batch.
    Fixes N+1 query issue.
    """
    # Query 1: Get all assignments
    query = select(Assignment).where(Assignment.classroom_id == classroom_id)
    if not include_unpublished:
        query = query.where(Assignment.status == "published")
    query = query.order_by(Assignment.created_at.desc())
    
    result = await db.execute(query)
    assignments = list(result.scalars().all())
    
    if not assignments:
        return []
    
    # Query 2: Get total members
    member_count_result = await db.execute(
        select(func.count(ClassroomMember.id))
        .where(
            and_(
                ClassroomMember.classroom_id == classroom_id,
                ClassroomMember.is_active == True
            )
        )
    )
    total_members = member_count_result.scalar() or 0
    
    # Query 3: Get completion counts for all assignments in batch
    assignment_ids = [a.id for a in assignments]
    completion_result = await db.execute(
        select(
            AssignmentSubmission.assignment_id,
            func.count(AssignmentSubmission.id).label("completed_count")
        )
        .where(
            and_(
                AssignmentSubmission.assignment_id.in_(assignment_ids),
                AssignmentSubmission.completed_at.isnot(None)
            )
        )
        .group_by(AssignmentSubmission.assignment_id)
    )
    completion_counts = {row[0]: row[1] for row in completion_result.all()}
    
    # Combine results
    return [
        {
            "assignment": assignment,
            "total_students": total_members,
            "completed_count": completion_counts.get(assignment.id, 0),
            "completion_percentage": round(
                completion_counts.get(assignment.id, 0) / total_members * 100, 1
            ) if total_members > 0 else 0
        }
        for assignment in assignments
    ]


async def get_assignment_with_stats(
    db: AsyncSession,
    assignment_id: int
) -> Optional[Dict[str, Any]]:
    """
    Get a single assignment with stats efficiently.
    """
    # Query 1: Get assignment
    result = await db.execute(
        select(Assignment).where(Assignment.id == assignment_id)
    )
    assignment = result.scalar_one_or_none()
    
    if not assignment:
        return None
    
    # Query 2: Get member count and completion count
    member_count_result = await db.execute(
        select(func.count(ClassroomMember.id))
        .where(
            and_(
                ClassroomMember.classroom_id == assignment.classroom_id,
                ClassroomMember.is_active == True
            )
        )
    )
    total_members = member_count_result.scalar() or 0
    
    completion_count_result = await db.execute(
        select(func.count(AssignmentSubmission.id))
        .where(
            and_(
                AssignmentSubmission.assignment_id == assignment_id,
                AssignmentSubmission.completed_at.isnot(None)
            )
        )
    )
    completed_count = completion_count_result.scalar() or 0
    
    return {
        "assignment": assignment,
        "total_students": total_members,
        "completed_count": completed_count,
        "completion_percentage": round(completed_count / total_members * 100, 1) if total_members > 0 else 0
    }


async def get_assignment_submissions_batch(
    db: AsyncSession,
    assignment_id: int
) -> List[Dict[str, Any]]:
    """
    Get all submissions for an assignment with user info in batch.
    Uses a single joined query instead of N+1.
    """
    result = await db.execute(
        select(AssignmentSubmission, User)
        .join(User, AssignmentSubmission.user_id == User.id)
        .where(AssignmentSubmission.assignment_id == assignment_id)
        .order_by(AssignmentSubmission.submitted_at.desc())
    )
    
    return [
        {
            "id": submission.id,
            "user_id": user.id,
            "username": user.username,
            "display_name": user.display_name or user.username,
            "status": submission.status,
            "completed_exercises": submission.completed_exercises,
            "score": submission.score,
            "feedback": submission.feedback,
            "xp_earned": submission.xp_earned,
            "treats_earned": submission.treats_earned,
            "started_at": submission.started_at,
            "submitted_at": submission.submitted_at,
            "graded_at": submission.graded_at
        }
        for submission, user in result.all()
    ]


# ============================================================================
# BATCH USER OPERATIONS
# ============================================================================

async def get_users_by_ids_batch(
    db: AsyncSession,
    user_ids: List[int]
) -> Dict[int, User]:
    """
    Get multiple users by ID in a single query.
    Returns: {user_id: User}
    """
    if not user_ids:
        return {}
    
    result = await db.execute(
        select(User).where(User.id.in_(user_ids))
    )
    
    return {user.id: user for user in result.scalars().all()}


async def get_user_profiles_by_ids_batch(
    db: AsyncSession,
    user_ids: List[int]
) -> Dict[int, UserSchoolProfile]:
    """
    Get multiple user school profiles by user ID in a single query.
    Returns: {user_id: UserSchoolProfile}
    """
    if not user_ids:
        return {}
    
    result = await db.execute(
        select(UserSchoolProfile)
        .where(UserSchoolProfile.user_id.in_(user_ids))
    )
    
    return {profile.user_id: profile for profile in result.scalars().all()}
