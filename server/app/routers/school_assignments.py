# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
API routes for assignments.
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

router = APIRouter(prefix="/api/school/assignments", tags=["School Assignments"])


# ============================================================================
# TEACHER ENDPOINTS
# ============================================================================

@router.post("", response_model=schemas.AssignmentResponse)
async def create_assignment(
    data: schemas.AssignmentCreate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Create a new assignment (teacher only)."""
    # Verify teacher owns the classroom
    if not await crud.is_classroom_teacher(db, current_user.id, data.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to create assignments in this classroom"
        )
    
    assignment = await crud.create_assignment(db, data)
    
    # Get statistics
    members = await crud.get_classroom_members(db, data.classroom_id)
    submissions = await crud.list_assignment_submissions(db, assignment.id)
    
    completed = sum(1 for s in submissions if s.completed_at is not None)
    
    return schemas.AssignmentResponse(
        **{k: v for k, v in vars(assignment).items() if not k.startswith('_')},
        total_students=len(members),
        completed_count=completed,
        completion_percentage=round(completed / len(members) * 100, 1) if members else 0
    )


@router.get("/classroom/{classroom_id}", response_model=List[schemas.AssignmentResponse])
async def list_classroom_assignments(
    classroom_id: int,
    include_unpublished: bool = False,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """List all assignments for a classroom."""
    # Check authorization
    is_teacher = await crud.is_classroom_teacher(db, current_user.id, classroom_id)
    is_member = await crud.is_classroom_member(db, current_user.id, classroom_id)
    
    if not (is_teacher or is_member):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view this classroom"
        )
    
    # Students can't see unpublished assignments
    if not is_teacher:
        include_unpublished = False
    
    # Use batch query to fix N+1 issue
    assignments_data = await crud_batch.get_assignments_with_stats_batch(
        db, classroom_id, include_unpublished
    )
    
    return [
        schemas.AssignmentResponse(
            **{k: v for k, v in vars(item["assignment"]).items() if not k.startswith('_')},
            total_students=item["total_students"],
            completed_count=item["completed_count"],
            completion_percentage=item["completion_percentage"]
        )
        for item in assignments_data
    ]


@router.get("/{assignment_id}/teacher-view", response_model=schemas.AssignmentResponse)
async def get_assignment_teacher_view(
    assignment_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get assignment details with all submissions (teacher view)."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Verify teacher owns the classroom
    if not await crud.is_classroom_teacher(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view this assignment"
        )
    
    members = await crud.get_classroom_members(db, assignment.classroom_id)
    submissions = await crud.list_assignment_submissions(db, assignment_id)
    completed = sum(1 for s in submissions if s.completed_at is not None)
    
    return schemas.AssignmentResponse(
        **{k: v for k, v in vars(assignment).items() if not k.startswith('_')},
        total_students=len(members),
        completed_count=completed,
        completion_percentage=round(completed / len(members) * 100, 1) if members else 0
    )


@router.patch("/{assignment_id}", response_model=schemas.AssignmentResponse)
async def update_assignment(
    assignment_id: int,
    data: schemas.AssignmentUpdate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Update an assignment (teacher only)."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Verify teacher owns the classroom
    if not await crud.is_classroom_teacher(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to update this assignment"
        )
    
    updated = await crud.update_assignment(db, assignment_id, data)
    
    members = await crud.get_classroom_members(db, assignment.classroom_id)
    submissions = await crud.list_assignment_submissions(db, assignment_id)
    completed = sum(1 for s in submissions if s.completed_at is not None)
    
    return schemas.AssignmentResponse(
        **{k: v for k, v in vars(updated).items() if not k.startswith('_')},
        total_students=len(members),
        completed_count=completed,
        completion_percentage=round(completed / len(members) * 100, 1) if members else 0
    )


@router.delete("/{assignment_id}")
async def delete_assignment(
    assignment_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Delete an assignment (teacher only)."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Verify teacher owns the classroom
    if not await crud.is_classroom_teacher(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to delete this assignment"
        )
    
    success = await crud.delete_assignment(db, assignment_id)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Failed to delete assignment"
        )
    
    return {"message": "Assignment deleted successfully"}


@router.get("/{assignment_id}/submissions", response_model=List[schemas.AssignmentSubmissionResponse])
async def get_assignment_submissions(
    assignment_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get all submissions for an assignment (teacher only)."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Verify teacher owns the classroom
    if not await crud.is_classroom_teacher(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not authorized to view submissions"
        )
    
    # Use batch query to fix N+1 issue - single query with join
    submissions_data = await crud_batch.get_assignment_submissions_batch(db, assignment_id)
    
    return [
        schemas.AssignmentSubmissionResponse(**item)
        for item in submissions_data
    ]


# ============================================================================
# STUDENT ENDPOINTS
# ============================================================================

@router.get("/my-assignments", response_model=List[schemas.AssignmentResponse])
async def get_my_assignments(
    classroom_id: int = None,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get all assignments for student's joined classrooms."""
    # Get student's classrooms
    classrooms = await crud.list_student_classrooms(db, current_user.id)
    
    if classroom_id:
        # Filter to specific classroom
        classrooms = [c for c in classrooms if c.id == classroom_id]
        if not classrooms:
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="Not a member of this classroom"
            )
    
    result = []
    for classroom in classrooms:
        assignments = await crud.list_classroom_assignments(db, classroom.id, include_unpublished=False)
        
        for assignment in assignments:
            # Get student's submission
            submission = await crud.get_assignment_submission(db, assignment.id, current_user.id)
            
            result.append(schemas.AssignmentResponse(
                **{k: v for k, v in vars(assignment).items() if not k.startswith('_')},
                total_students=0,  # Not needed for student view
                completed_count=0,
                completion_percentage=0,
                is_completed=submission.completed_at is not None if submission else False,
                problems_completed=submission.problems_completed if submission else []
            ))
    
    return result


@router.get("/{assignment_id}/student-view", response_model=schemas.AssignmentResponse)
async def get_assignment_student_view(
    assignment_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get assignment details as a student."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Check student is member of classroom
    if not await crud.is_classroom_member(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not a member of this classroom"
        )
    
    # Get student's submission
    submission = await crud.get_assignment_submission(db, assignment_id, current_user.id)
    
    return schemas.AssignmentResponse(
        **{k: v for k, v in vars(assignment).items() if not k.startswith('_')},
        total_students=0,
        completed_count=0,
        completion_percentage=0,
        is_completed=submission.completed_at is not None if submission else False,
        problems_completed=submission.problems_completed if submission else []
    )


@router.post("/{assignment_id}/submit/{problem_id}")
async def submit_problem(
    assignment_id: int,
    problem_id: str,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Submit a problem for an assignment."""
    assignment = await crud.get_assignment(db, assignment_id)
    if not assignment:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Assignment not found"
        )
    
    # Check student is member of classroom
    if not await crud.is_classroom_member(db, current_user.id, assignment.classroom_id):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not a member of this classroom"
        )
    
    # Check if problem is part of assignment
    if problem_id not in assignment.problem_ids:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Problem not part of this assignment"
        )
    
    # Check due date
    from datetime import datetime
    if assignment.due_at and datetime.utcnow() > assignment.due_at:
        if not assignment.allow_late_submission:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Assignment deadline has passed"
            )
    
    submission = await crud.submit_assignment_problem(db, assignment_id, current_user.id, problem_id)
    
    return {
        "message": "Problem submitted successfully",
        "assignment_completed": submission.completed_at is not None,
        "problems_completed": len(submission.problems_completed),
        "total_problems": len(assignment.problem_ids),
        "xp_earned": submission.xp_earned
    }


# Add missing CRUD functions
async def list_assignment_submissions(db, assignment_id):
    """List all submissions for an assignment."""
    from sqlalchemy import select
    result = await db.execute(
        select(crud.AssignmentSubmission).where(
            crud.AssignmentSubmission.assignment_id == assignment_id
        )
    )
    return result.scalars().all()


# Monkey patch the crud module
crud.list_assignment_submissions = list_assignment_submissions
