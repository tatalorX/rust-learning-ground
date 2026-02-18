# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
API routes for classroom management.
"""
from typing import List

from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.ext.asyncio import AsyncSession

from app.database import get_db
from app.auth import get_current_user
from app.models import User
from app import crud_school as crud
from app import crud_school_batch as crud_batch
from app import schemas_school as schemas
from app.models_school import UserRole

router = APIRouter(prefix="/api/school/classrooms", tags=["School Classrooms"])


# ============================================================================
# TEACHER ENDPOINTS
# ============================================================================

@router.post("", response_model=schemas.ClassroomResponse)
async def create_classroom(
    data: schemas.ClassroomCreate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Create a new classroom (teacher only)."""
    # Check if user is teacher or admin
    profile = await crud.get_user_school_profile(db, current_user.id)
    if profile.role not in [UserRole.TEACHER, UserRole.ADMIN]:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Only teachers can create classrooms"
        )
    
    classroom = await crud.create_classroom(db, current_user.id, data)
    
    # Get member count
    member_count = await crud.get_classroom_member_count(db, classroom.id)
    
    return schemas.ClassroomResponse(
        **{k: v for k, v in vars(classroom).items() if not k.startswith('_')},
        member_count=member_count,
        teacher_username=current_user.username,
        teacher_display_name=current_user.display_name
    )


@router.get("/teaching", response_model=List[schemas.ClassroomListResponse])
async def list_my_teaching_classrooms(
    include_archived: bool = False,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """List all classrooms owned by current user."""
    # Use batch query to fix N+1 issue
    classrooms_data = await crud_batch.get_teacher_classrooms_batch(
        db, current_user.id, include_archived
    )
    
    return [
        schemas.ClassroomListResponse(
            **{k: v for k, v in vars(item["classroom"]).items() if not k.startswith('_')},
            member_count=item["member_count"]
        )
        for item in classrooms_data
    ]


@router.get("/teaching/{classroom_id}", response_model=schemas.ClassroomResponse)
async def get_teacher_classroom_details(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get detailed classroom info (teacher view)."""
    # Verify ownership
    if not await crud.is_classroom_teacher(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view this classroom"
        )
    
    classroom = await crud.get_classroom(db, classroom_id)
    if not classroom:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Classroom not found"
        )
    
    member_count = await crud.get_classroom_member_count(db, classroom_id)
    
    return schemas.ClassroomResponse(
        **{k: v for k, v in vars(classroom).items() if not k.startswith('_')},
        member_count=member_count,
        teacher_username=current_user.username,
        teacher_display_name=current_user.display_name
    )


@router.patch("/{classroom_id}", response_model=schemas.ClassroomResponse)
async def update_classroom(
    classroom_id: int,
    data: schemas.ClassroomUpdate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Update classroom details (teacher only)."""
    # Verify ownership
    if not await crud.is_classroom_teacher(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to update this classroom"
        )
    
    classroom = await crud.update_classroom(db, classroom_id, data)
    if not classroom:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Classroom not found"
        )
    
    member_count = await crud.get_classroom_member_count(db, classroom_id)
    
    return schemas.ClassroomResponse(
        **{k: v for k, v in vars(classroom).items() if not k.startswith('_')},
        member_count=member_count,
        teacher_username=current_user.username,
        teacher_display_name=current_user.display_name
    )


@router.post("/{classroom_id}/archive")
async def archive_classroom(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Archive a classroom (teacher only)."""
    # Verify ownership
    if not await crud.is_classroom_teacher(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to archive this classroom"
        )
    
    success = await crud.archive_classroom(db, classroom_id)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Classroom not found"
        )
    
    return {"message": "Classroom archived successfully"}


@router.delete("/{classroom_id}")
async def delete_classroom(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Delete a classroom (teacher only)."""
    # Verify ownership
    if not await crud.is_classroom_teacher(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to delete this classroom"
        )
    
    success = await crud.delete_classroom(db, classroom_id)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Classroom not found"
        )
    
    return {"message": "Classroom deleted successfully"}


@router.get("/{classroom_id}/members", response_model=List[schemas.ClassroomMemberResponse])
async def get_classroom_members(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get all members of a classroom (teacher or member)."""
    # Check authorization
    is_teacher = await crud.is_classroom_teacher(db, current_user.id, classroom_id)
    is_member = await crud.is_classroom_member(db, current_user.id, classroom_id)
    
    if not (is_teacher or is_member):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view this classroom"
        )
    
    members = await crud.get_classroom_members(db, classroom_id)
    
    return [schemas.ClassroomMemberResponse(**member) for member in members]


@router.get("/{classroom_id}/statistics", response_model=schemas.ClassStatisticsResponse)
async def get_classroom_statistics(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get detailed statistics for classroom (teacher only)."""
    # Verify ownership
    if not await crud.is_classroom_teacher(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view statistics"
        )
    
    classroom = await crud.get_classroom(db, classroom_id)
    members = await crud.get_classroom_members(db, classroom_id)
    
    # Calculate statistics
    total_problems = sum(m["problems_solved_count"] for m in members)
    avg_problems = total_problems / len(members) if members else 0
    total_xp = sum(m["xp_earned"] for m in members)
    
    # Rank distribution
    from collections import Counter
    rank_dist = Counter(m["current_rank"].value for m in members)
    
    # Top students
    top_students = sorted(members, key=lambda m: m["xp_earned"], reverse=True)[:5]
    
    return schemas.ClassStatisticsResponse(
        classroom_id=classroom_id,
        classroom_name=classroom.name,
        total_students=len(members),
        active_students=sum(1 for m in members if m["is_active"]),
        total_problems_solved=total_problems,
        average_problems_per_student=round(avg_problems, 1),
        total_xp_earned=total_xp,
        rank_distribution=dict(rank_dist),
        recent_activity=await crud.get_recent_activity(db, classroom_id, limit=10),
        top_students=[{
            "user_id": s["user_id"],
            "username": s["username"],
            "display_name": s["display_name"],
            "xp_earned": s["xp_earned"],
            "problems_solved": s["problems_solved_count"]
        } for s in top_students]
    )


# ============================================================================
# STUDENT ENDPOINTS
# ============================================================================

@router.get("/joined", response_model=List[schemas.ClassroomListResponse])
async def list_joined_classrooms(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """List all classrooms the current user has joined."""
    # Use batch query to fix N+1 issue
    classrooms_data = await crud_batch.get_student_classrooms_batch(db, current_user.id)
    
    return [
        schemas.ClassroomListResponse(
            **{k: v for k, v in vars(item["classroom"]).items() if not k.startswith('_')},
            member_count=item["member_count"]
        )
        for item in classrooms_data
    ]


@router.post("/join")
async def join_classroom(
    request: schemas.ClassroomJoinRequest,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Join a classroom using a code."""
    # Find classroom by code
    classroom = await crud.get_classroom_by_code(db, request.code)
    if not classroom:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Invalid classroom code"
        )
    
    # Join
    try:
        membership = await crud.join_classroom(db, current_user.id, classroom.id)
    except ValueError as e:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(e)
        )
    
    return {
        "message": "Successfully joined classroom",
        "classroom_id": classroom.id,
        "classroom_name": classroom.name
    }


@router.post("/{classroom_id}/leave")
async def leave_classroom(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Leave a classroom."""
    success = await crud.leave_classroom(db, current_user.id, classroom_id)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Not a member of this classroom"
        )
    
    return {"message": "Successfully left classroom"}


@router.get("/{classroom_id}/student-view", response_model=schemas.ClassroomResponse)
async def get_student_classroom_view(
    classroom_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get classroom details as a student member."""
    # Check membership
    if not await crud.is_classroom_member(db, current_user.id, classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not a member of this classroom"
        )
    
    classroom = await crud.get_classroom(db, classroom_id)
    if not classroom:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Classroom not found"
        )
    
    # Get teacher info
    from app.models import User
    from sqlalchemy import select
    
    result = await db.execute(
        select(User).where(User.id == classroom.teacher_id)
    )
    teacher = result.scalar_one()
    
    member_count = await crud.get_classroom_member_count(db, classroom_id)
    
    return schemas.ClassroomResponse(
        **{k: v for k, v in vars(classroom).items() if not k.startswith('_')},
        member_count=member_count,
        teacher_username=teacher.username,
        teacher_display_name=teacher.display_name
    )
